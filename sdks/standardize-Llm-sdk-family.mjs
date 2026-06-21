#!/usr/bin/env node
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const checkOnly = process.argv.includes("--check");

const owner = "sdkwork-llm";
const standardVersion = "2026-06-10";

const families = [
  {
    root: "sdks/sdkwork-llm-sdk",
    authority: "sdkwork-llm-open-api",
    input: "openapi/llm-open-api.openapi.json",
    packageName: "@sdkwork/llm-sdk",
    apiPrefix: "/llm/v3/api",
    clientName: "SdkworkLlmOpenClient",
    forbiddenPathPrefixes: ["/app/v3/api/", "/backend/v3/api/"],
  },
  {
    root: "sdks/sdkwork-llm-app-sdk",
    authority: "sdkwork-llm.app",
    input: "openapi/llm-app-api.openapi.json",
    packageName: "@sdkwork/llm-app-sdk",
    apiPrefix: "/app/v3/api",
    clientName: "SdkworkLlmAppClient",
    forbiddenPathPrefixes: ["/backend/v3/api/", "/llm/v3/api/"],
  },
  {
    root: "sdks/sdkwork-llm-backend-sdk",
    authority: "sdkwork-llm.backend",
    input: "openapi/llm-backend-api.openapi.json",
    packageName: "@sdkwork/llm-backend-sdk",
    apiPrefix: "/backend/v3/api",
    clientName: "SdkworkLlmBackendClient",
    forbiddenPathPrefixes: ["/app/v3/api/", "/llm/v3/api/"],
  },
];

function readJson(relativePath) {
  return JSON.parse(readFileSync(path.join(workspaceRoot, relativePath), "utf8"));
}

const failures = [];

for (const family of families) {
  const assembly = readJson(path.join(family.root, ".sdkwork-assembly.json"));
  const manifest = readJson(path.join(family.root, "sdk-manifest.json"));
  const component = readJson(path.join(family.root, "specs/component.spec.json"));

  if (assembly.sdkOwner !== owner) {
    failures.push(`${family.root} assembly sdkOwner must be ${owner}`);
  }
  if (manifest.sdkOwner !== owner) {
    failures.push(`${family.root} manifest sdkOwner must be ${owner}`);
  }
  if (assembly.apiAuthority !== family.authority || manifest.apiAuthority !== family.authority) {
    failures.push(`${family.root} apiAuthority mismatch`);
  }
  if (assembly.generationInputSpec !== family.input || manifest.generationInputSpec !== family.input) {
    failures.push(`${family.root} generationInputSpec mismatch`);
  }
  if (manifest.standardProfile !== "sdkwork-v3") {
    failures.push(`${family.root} must declare standardProfile sdkwork-v3`);
  }
  if (manifest.packageName !== family.packageName) {
    failures.push(`${family.root} packageName mismatch`);
  }
  if (!component.contracts.sdkClients.includes(family.clientName)) {
    failures.push(`${family.root} component spec must declare ${family.clientName}`);
  }

  const openapi = readJson(path.join(family.root, family.input));
  if (openapi["x-sdkwork-owner"] !== owner) {
    failures.push(`${family.root} OpenAPI x-sdkwork-owner mismatch`);
  }
  for (const [routePath, pathItem] of Object.entries(openapi.paths ?? {})) {
    for (const prefix of family.forbiddenPathPrefixes) {
      if (routePath.startsWith(prefix)) {
        failures.push(`${family.root} must not include dependency route ${routePath}`);
      }
    }
  }
}

if (failures.length > 0) {
  console.error(JSON.stringify({ ok: false, mode: checkOnly ? "check" : "validate", failures }, null, 2));
  process.exit(1);
}

console.log(
  JSON.stringify({ ok: true, mode: checkOnly ? "check" : "validate", owner, standardVersion, families: families.length }, null, 2),
);
