// xrpl
import {
  Client,
  decodeAccountID,
  Invoke,
  Transaction,
  Wallet,
} from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";

const HOOK_NAME = "util_accid";

describe("util_accid.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // rUVWhZnU2AYWYXzHpCUcSVtcuuNXkuDD1X
    alice = Wallet.fromSecret(`snudyVMLzKNzXZ9HbzdiNFzfDKP2F`);
    // r4tryuYjz7ZxtJFb2ELWGTNnEFsT1ZxaJp
    bob = Wallet.fromSecret(`snmxzsV6WCiZ1RCg566sKfMkzPcwB`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "converts r-address to an account id",
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

      const [hookExecution] = txResponse.result.meta.HookExecutions as [
        HookExecution,
      ];

      const { HookReturnString, HookReturnCode } = hookExecution.HookExecution;

      expect(
        TestUtils.deserializeHexStringAsBigInt(HookReturnCode.toString()),
      ).toBe(0n);
      expect(HookReturnString).toMatch(
        decodeAccountID("rLqUFYGLMBS9jF63iRkadvu3cTixadRTd3")
          .toString(`hex`)
          .toUpperCase(),
      );
    },
    3 * 60_000,
  );
});
