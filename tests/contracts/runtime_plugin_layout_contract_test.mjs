import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

function collectFiles(root, predicate) {
  if (!fs.existsSync(root)) {
    return [];
  }

  const result = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const fullPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      result.push(...collectFiles(fullPath, predicate));
    } else if (predicate(fullPath)) {
      result.push(fullPath.replaceAll(path.sep, "/"));
    }
  }
  return result;
}

const runtimePluginManifests = collectFiles(
  "plugins",
  (file) => path.basename(file) === "sdkwork.llm.plugin.json",
);
const agentPluginRuntimeManifests = collectFiles(
  ".sdkwork/plugins",
  (file) => path.basename(file) === "sdkwork.llm.plugin.json",
);

assert.ok(
  runtimePluginManifests.includes(
    "plugins/sdkwork-llm-plugin-native-sql/sdkwork.llm.plugin.json",
  ),
  "native SQL runtime plugin manifest must live under plugins/",
);
assert.ok(
  runtimePluginManifests.includes(
    "plugins/sdkwork-llm-plugin-reference-profiles/sdkwork.llm.plugin.json",
  ),
  "reference implementation profile runtime plugin manifest must live under plugins/",
);
assert.deepEqual(
  agentPluginRuntimeManifests,
  [],
  "runtime LLM plugin manifests must not live under .sdkwork/plugins/",
);

for (const manifestPath of runtimePluginManifests) {
  const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  assert.equal(manifest.kind, "sdkwork.llm.plugin");
  assert.ok(
    manifest.pluginId.startsWith("sdkwork-llm-plugin-"),
    `${manifestPath} pluginId must use SDKWork LLM runtime plugin naming`,
  );
}
