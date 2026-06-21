import assert from "node:assert/strict";
import fs from "node:fs";

const requiredTables = [
  "llm_space",
  "llm_event",
  "llm_record",
  "llm_record_source",
  "llm_candidate",
  "llm_habit",
  "llm_retrieval_trace",
  "llm_retrieval_hit",
  "llm_context_pack",
];

for (const file of [
  "plugins/sdkwork-llm-plugin-native-sql/migrations/postgres/V202606100001__llm_phase1.sql",
  "plugins/sdkwork-llm-plugin-native-sql/migrations/sqlite/V202606100001__llm_phase1.sql",
]) {
  const sql = fs.readFileSync(file, "utf8").toLowerCase();
  for (const table of requiredTables) {
    assert.match(
      sql,
      new RegExp(`create\\s+table\\s+(if\\s+not\\s+exists\\s+)?${table}\\b`),
      `${file} missing ${table}`,
    );
  }
  assert.doesNotMatch(
    sql,
    /vector|embedding\(/,
    `${file} must not require vector or embedding storage in Phase 1`,
  );
}

console.log("Schema registry phase1 contract test passed");
