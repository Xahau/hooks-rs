import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";
import { TestUtils } from "./setup";

const HOOK_NAME = "slot_account";

describe("slot_account.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // Because Faucet only allows one account to be created every 60 seconds,
    // we will use the following accounts for testing. Change the secrets when
    // running out of funds.
    // rh49mCBgncu9xdyyeRwU8LHi3feUxUUrHj
    alice = Wallet.fromSecret(`shrXvXogexynVPmEMJ3WAvgHNBBpB`);
    // rEHSy16Fs7Yw5tMrGpGw9PUotrq5AwkSuh
    bob = Wallet.fromSecret(`snNBCoUnM7y5NuPduZVfBk8r4cwLv`);

    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "produces keylet",
    async () => {
      const tx: Invoke & Transaction = {
        TransactionType: "Invoke",
        Account: bob.classicAddress,
        Destination: alice.classicAddress,
      };
      // Autofilling fee does not work with hooks yet
      const { Fee, ...rest } = await client.autofill(tx);
      const fee = await TestUtils.getTransactionFee(client, rest);
      const txResponse = await TestUtils.submitAndWaitWithRetries(
        client,
        {
          ...tx,
          Fee: fee,
        },
        {
          wallet: bob,
          autofill: true,
        },
      );
      if (!txResponse.result.meta) {
        throw new Error("No meta in tx response");
      }
      if (typeof txResponse.result.meta === "string") {
        throw new Error("Meta is string, not object");
      }
      const { meta } = txResponse.result;
      if (!(meta.HookExecutions && meta.HookExecutions.length > 0)) {
        throw new Error(`Hook execution data is empty`);
      }

      if (meta.HookExecutions.length > 1) {
        throw new Error(`Hook execution happened more than once`);
      }

      if (txResponse.result.meta.TransactionResult !== "tesSUCCESS") {
        console.error(JSON.stringify(txResponse, null, 2));

        throw new Error(`Transaction failed`);
      }

      // safe type: we checked everything
      const [hookExecution] = meta.HookExecutions as [HookExecution];

      const { HookReturnString, HookReturnCode } = hookExecution.HookExecution;
      expect(Buffer.from(HookReturnString, "hex").toString()).toBe(
        "passing",
      );
      // Account has 1000000000 drops of XAH
      expect(BigInt("0x" + HookReturnCode)).toBe(BigInt(1000000000n));
    },
    3 * 60_000,
  );
});
