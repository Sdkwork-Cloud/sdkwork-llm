# MEMORY Database Module

Canonical lifecycle assets for `sdkwork-llm` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `memory`
- serviceCode: `MEMORY`
- tablePrefix: `llm_`
- engines: `postgres` (production), `sqlite` (local/dev standalone)

## Commands

```bash
pnpm run db:materialize:contract
pnpm run db:validate
```

Legacy SQL: `plugins/sdkwork-llm-plugin-native-sql/migrations/postgres/V202606100001__llm_phase1.sql` → `database/ddl/baseline/postgres/0001_llm_legacy_baseline.sql`

Runtime bootstrap: `sdkwork-llm-database-host` via `bootstrap_llm_database()` when postgres pool is configured; SQLite path continues to use `install_sqlite_schema()`.
