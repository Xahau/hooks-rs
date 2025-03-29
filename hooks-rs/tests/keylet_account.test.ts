// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { ExecutionUtility } from "@transia/hooks-toolkit";
import { TestUtils } from "./setup";

const HOOK_NAME = "keylet_account_id";

describe("keylet_account_id.rs", () => {
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
    // rULqKiLvbGda4VKAqsto5hxmmDavJm2oav
    alice = Wallet.fromSecret(`shW8QFpXBGGh86mjdkpMTKk54kqwh`);
    // rnvgdKhaMnLtNMZmGAkKq1VNUx25Qg8Vxz
    bob = Wallet.fromSecret(`snSRRGDAgcUB9F4JWgx2YCt6aS7h3`);

    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "accepts an incoming txn",
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

      if (txResponse.result.meta.TransactionResult !== "tesSUCCESS") {
        console.error(JSON.stringify(txResponse, null, 2));

        throw new Error(`Transaction failed`);
      }

      const hookExecutions = await ExecutionUtility.getHookExecutionsFromMeta(
        client,
        txResponse.result.meta,
      );
      if (!hookExecutions.executions[0]) {
        throw new Error(`Hook execution data is empty`);
      }

      expect(hookExecutions.executions[0].HookReturnString).toMatch(
        "accept.rs: Finished.",
      );
    },
    3 * 60_000,
  );
});
