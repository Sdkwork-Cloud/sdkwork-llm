#!/usr/bin/env node
import { spawn } from "node:child_process";
import { parseArgs } from "node:util";

import {
  DEFAULT_DEV_PROFILE_ID,
  REPO_ROOT,
  loadEnvFile,
  loadProfile,
  mergeRuntimeEnv,
  resolveDevProfileId,
} from "./lib/llm-topology.mjs";

const { values } = parseArgs({
  options: {
    target: { type: "string", default: "server" },
    "deployment-profile": { type: "string", default: "standalone" },
    "service-layout": { type: "string", default: "unified-process" },
    database: { type: "string", default: "sqlite" },
    "dev-env-file": { type: "string" },
  },
});

const profileId =
  values["dev-env-file"] == null
    ? resolveDevProfileId(values["deployment-profile"], values["service-layout"])
    : (loadEnvFile(values["dev-env-file"], REPO_ROOT).SDKWORK_LLM_PROFILE_ID
      ?? DEFAULT_DEV_PROFILE_ID);

const profileEnv = values["dev-env-file"]
  ? loadEnvFile(values["dev-env-file"], REPO_ROOT)
  : loadProfile(profileId);

const runtimeEnv = mergeRuntimeEnv(process.env, profileEnv, {
  SDKWORK_LLM_RUNTIME_TARGET: values.target,
  SDKWORK_LLM_DEPLOYMENT_PROFILE: values["deployment-profile"],
  SDKWORK_LLM_SERVICE_LAYOUT: values["service-layout"],
  SDKWORK_LLM_DATABASE: values.database,
  SDKWORK_LLM_PROFILE_ID: profileId,
});

console.log(
  `Starting SDKWork LLM dev server (${values.target}, ${values.database}, ${values["deployment-profile"]}, ${values["service-layout"]}, profile=${profileId})`,
);

const child = spawn("cargo", ["run", "-p", "sdkwork-llm-standalone-gateway"], {
  cwd: REPO_ROOT,
  stdio: "inherit",
  shell: true,
  env: runtimeEnv,
});

child.on("exit", (code) => process.exit(code ?? 1));
