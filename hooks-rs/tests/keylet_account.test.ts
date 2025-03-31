import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";
import { TestUtils } from "./setup";

const HOOK_NAME = "keylet_account";

describe("keylet_account.rs", () => {
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
    // rHExWv7T4WV3MLSn8okiBwEKt2gRZRfAs2
    alice = Wallet.fromSecret(`ssNt8v9WvQ5WR6orqfe6LU6sHh7R6`);
    // r3NkZcLESTsCmg1VtL7542nvwDBwjCWpbJ
    bob = Wallet.fromSecret(`snVo4N7YW3xfYHA64nrBgL8UUkq4X`);

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
      console.log(HookReturnString);
      expect(BigInt(HookReturnCode)).toBe(0n);

      const accountKeylet = HookReturnString;
      // Keylet is always serialized to 34 bytes
      const accountKeyletBuffer = Buffer.from(accountKeylet, `hex`);
      expect(accountKeyletBuffer.length).toBe(34);

      // Check at https://richardah.github.io/xrpl-keylet-tools/
      // by inserting r3zEReqN3Ge3g1GXUThuVSrn4dM8xpoA7i
      expect(accountKeylet.toUpperCase()).toBe(
        "0061355E27663861169420D62A5955FECE7FB519121F8CDC15963FD1561653531F9C",
      );
    },
    3 * 60_000,
  );
});
