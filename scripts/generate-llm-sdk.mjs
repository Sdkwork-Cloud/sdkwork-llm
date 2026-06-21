#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const sdkgen = path.resolve(workspaceRoot, "../sdkwork-sdk-generator/bin/sdkgen.js");

const families = [
  {
    input: "sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json",
    output: "sdks/sdkwork-llm-sdk/sdkwork-llm-sdk-typescript/generated/server-openapi",
    name: "sdkwork-llm-sdk",
    type: "custom",
    packageName: "@sdkwork/llm-sdk",
    apiPrefix: "/llm/v3/api",
    clientName: "SdkworkLlmOpenClient",
  },
  {
    input: "sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json",
    output: "sdks/sdkwork-llm-app-sdk/sdkwork-llm-app-sdk-typescript/generated/server-openapi",
    name: "sdkwork-llm-app-sdk",
    type: "app",
    packageName: "@sdkwork/llm-app-sdk",
    apiPrefix: "/app/v3/api",
    clientName: "SdkworkLlmAppClient",
  },
  {
    input: "sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json",
    output: "sdks/sdkwork-llm-backend-sdk/sdkwork-llm-backend-sdk-typescript/generated/server-openapi",
    name: "sdkwork-llm-backend-sdk",
    type: "backend",
    packageName: "@sdkwork/llm-backend-sdk",
    apiPrefix: "/backend/v3/api",
    clientName: "SdkworkLlmBackendClient",
  },
];

function runGenerate(family) {
  const args = [
    sdkgen,
    "generate",
    "-i",
    path.join(workspaceRoot, family.input),
    "-o",
    path.join(workspaceRoot, family.output),
    "-n",
    family.name,
    "-t",
    family.type,
    "-l",
    "typescript",
    "--package-name",
    family.packageName,
    "--api-prefix",
    family.apiPrefix,
    "--standard-profile",
    "sdkwork-v3",
    "--fixed-sdk-version",
    "0.1.0",
    "--client-name",
    family.clientName,
  ];

  const result = spawnSync("node", args, { stdio: "inherit", cwd: workspaceRoot });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

for (const family of families) {
  console.log(`Generating TypeScript SDK for ${family.name}`);
  runGenerate(family);
}

console.log("LLM SDK generation completed.");
