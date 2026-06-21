import assert from "node:assert/strict";
import fs from "node:fs";

const migrationPaths = [
  "plugins/sdkwork-llm-plugin-native-sql/migrations/sqlite/V202606100001__llm_phase1.sql",
  "plugins/sdkwork-llm-plugin-native-sql/migrations/postgres/V202606100001__llm_phase1.sql",
];

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
  "llm_index",
  "llm_retrieval_profile",
  "llm_implementation_profile",
  "llm_provider_binding",
  "llm_eval_run",
  "llm_audit_log",
  "llm_outbox_event",
];

for (const migrationPath of migrationPaths) {
  assert.ok(fs.existsSync(migrationPath), `${migrationPath} must exist`);
  const sql = fs.readFileSync(migrationPath, "utf8").toLowerCase();

  for (const table of requiredTables) {
    assert.match(
      sql,
      new RegExp(`create\\s+table\\s+(if\\s+not\\s+exists\\s+)?${table}\\b`),
      `${migrationPath} must create ${table}`,
    );
  }

  assert.doesNotMatch(
    sql,
    /\b(vector|embedding|embeddings|pgvector)\b/,
    `${migrationPath} must not require vector or embedding storage in native_sql phase1`,
  );
}
