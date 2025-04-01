import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";
import { TestUtils } from "./setup";

const HOOK_NAME = "otxn_slot";

describe("otxn_slot.rs", () => {
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
    // rwavcD9KQpDtZ1enE1MUfvYXiPdDqQ9QFh
    alice = Wallet.fromSecret(`snZo7aSjs4F1FvoHYY7dCcjFfukpD`);
    // rhHb7UuZNi86ft1TeNNPbUEHjcbKhMskXE
    bob = Wallet.fromSecret(`ssdfQdix3uHynzXMHqhWQFMtUACae`);

    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "can otxn_slot",
    async () => {
      const tx: Invoke & Transaction = {
        TransactionType: "Invoke",
        Account: bob.classicAddress,
        Destination: alice.classicAddress,
        Memos: [
          {
            Memo: {
              MemoData: "21",
            },
          },
          {
            Memo: {
              MemoData: "31",
            },
          },
        ],
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
      expect(Buffer.from(HookReturnString, "hex").toString()).toBe("passing");
      expect(BigInt("0x" + HookReturnCode)).toBe(BigInt(0));
    },
    3 * 60_000,
  );
});
