import * as path from "jsr:@std/path@1.0.8";
import * as xrpl from "npm:@transia/xrpl@2.7.3-alpha.28";
import { getFeeEstimateXrp } from "npm:@transia/xrpl@2.7.3-alpha.28/dist/npm/sugar/index.js";
import { Hex, Logger } from "../misc/mod.ts";
import type { HookGrant, HookParameter, HookPayload } from "../types/hooks.ts";

async function hexNamespace(hookNamespaceSeed: string): Promise<string> {
  const data = new TextEncoder().encode(hookNamespaceSeed);
  const hashBuffer = await crypto.subtle.digest("SHA-256", data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  const hashHex = hashArray.map((b) => b.toString(16).padStart(2, "0")).join("")
    .toUpperCase();
  return hashHex;
}

function hexHookParameters(data: HookParameter[]): HookParameter[] {
  const hookParameters: HookParameter[] = [];
  for (const parameter of data) {
    let hookPName = parameter.HookParameter.HookParameterName;
    let hookPValue = parameter.HookParameter.HookParameterValue;

    if (!Hex.isHex(hookPName)) {
      hookPName = Hex.stringToHexString(hookPName);
    }

    if (!Hex.isHex(hookPValue)) {
      hookPValue = Hex.stringToHexString(hookPValue);
    }

    hookParameters.push({
      HookParameter: {
        HookParameterName: hookPName,
        HookParameterValue: hookPValue,
      },
    });
  }
  return hookParameters;
}

async function createHookPayload(
  version?: number | null,
  namespace?: string | null,
  flags?: number | 0,
  HookOn = `0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbfffff`,
  hookParams?: HookParameter[] | null,
  hookGrants?: HookGrant[] | null,
): Promise<HookPayload> {
  const hook = {
    HookOn,
  } as HookPayload;
  if (typeof version === "number") {
    hook.HookApiVersion = version;
  }
  if (namespace) {
    hook.HookNamespace = await hexNamespace(namespace);
  }
  if (flags) {
    hook.Flags = flags;
  }
  if (hookParams) {
    hook.HookParameters = hexHookParameters(hookParams);
  }
  if (hookGrants) {
    hook.HookGrants = hookGrants;
  }
  return hook;
}

export async function buildHook(hookName: string): Promise<HookPayload> {
  Logger.log(`info`, `Building hook "${hookName}"`);
  const cargoBuildOutput = new Deno.Command(`cargo`, {
    args: [
      "build",
      "--release",
    ],
    stderr: `piped`,
    stdout: `piped`,
  }).spawn();
  await Logger.handleOutput(cargoBuildOutput);
  const hook = await createHookPayload(
    0,
    `${hookName}namespace`,
    undefined,
  );
  const wasmDir = path.join(
    Deno.cwd(),
    `target`,
    `wasm32-unknown-unknown`,
    `release`,
  );
  const wasmInFile = path.join(wasmDir, `${hookName}.wasm`);
  const wasmOutFlattened = path.join(
    wasmDir,
    `${hookName}-flattened.wasm`,
  );
  // Maximum allowable depth of blocks reached is 16 levels in hooks GuardCheck.
  // Otherwise, the node will not validate the SetHook transaction.
  // Therefore, flatten it using wasm-opt.
  const wasmOptOutput = new Deno.Command(
    `wasm-opt`,
    {
      args: [
        wasmInFile,
        `--flatten`,
        `--rereloop`,
        `-Oz`,
        `-Oz`,
        `-o`,
        wasmOutFlattened,
      ],
      stderr: `piped`,
      stdout: `piped`,
    },
  ).spawn();
  await Logger.handleOutput(wasmOptOutput, false);
  const wasmOutCleaned = path.join(wasmDir, `${hookName}-cleaned.wasm`);
  const hookCleanerOut = new Deno.Command(
    `hook-cleaner`,
    {
      args: [
        wasmOutFlattened,
        wasmOutCleaned,
      ],
      stderr: `piped`,
      stdout: `piped`,
    },
  ).spawn();
  await Logger.handleOutput(hookCleanerOut);
  const guardCheckerOut = new Deno.Command(`guard_checker`, {
    args: [
      wasmOutCleaned,
    ],
    stderr: `piped`,
    stdout: `piped`,
  }).spawn();
  await Logger.handleOutput(guardCheckerOut);

  const wasm = await Deno.readFile(wasmOutCleaned);
  const wasmHex = Hex.uint8ArrayToHexString(wasm).toUpperCase();
  hook.CreateCode = wasmHex;

  return hook;
}

export async function getTransactionFee(
  client: xrpl.Client,
  transaction: xrpl.Transaction,
): Promise<string> {
  const copyTx = JSON.parse(JSON.stringify(transaction));
  copyTx.Fee = `0`;
  copyTx.SigningPubKey = ``;

  const preparedTx = await client.autofill(copyTx);

  const tx_blob = xrpl.encode(preparedTx);

  const result = await getFeeEstimateXrp(client, tx_blob);

  return result;
}

export async function submitAndWaitWithRetries(
  client: xrpl.Client,
  ...params: Parameters<xrpl.Client["submitAndWait"]>
) {
  let tries = 0;
  while (tries < 3) {
    try {
      const result = await client.submitAndWait(...params);
      return result;
    } catch (e) {
      console.error(`${e} - retrying...`);
      await new Promise((resolve) => setTimeout(resolve, 1000));
      tries++;
      continue;
    }
  }

  throw new Error(`Could not submit transaction after ${tries} tries`);
}

export async function setHook(
  client: xrpl.Client,
  secret: string,
  hook: HookPayload,
) {
  const wallet = xrpl.Wallet.fromSecret(secret);
  const tx: xrpl.SetHook = {
    TransactionType: `SetHook`,
    Account: wallet.address,
    Hooks: [{ Hook: hook }],
  };

  const fee = await getTransactionFee(client, tx);
  tx.Fee = fee;

  const submitResponse = await submitAndWaitWithRetries(
    client,
    tx,
    {
      wallet,
      failHard: true,
      autofill: true,
    },
  );

  return submitResponse;
}
