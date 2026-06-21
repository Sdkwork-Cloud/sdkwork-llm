use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;

pub type LlmDatabasePool = DatabasePool;

pub async fn connect_llm_pool_from_env() -> Result<LlmDatabasePool, PoolError> {
    // DATABASE_SPEC serviceCode `LLM` → env prefix `SDKWORK_LLM_*`
    let config = DatabaseConfig::from_env("LLM")?;
    create_pool_from_config(config).await
}

pub async fn install_sqlite_schema(pool: &LlmDatabasePool) -> Result<(), String> {
    if let Some(sqlite) = pool.as_sqlite() {
        NativeSqlLlmStore::install_sqlite_phase1_schema(sqlite)
            .await
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub async fn open_native_sql_store_from_pool(
    pool: &LlmDatabasePool,
) -> Result<sdkwork_llm_plugin_native_sql::NativeSqlLlmStore, String> {
    let sqlite = pool
        .as_sqlite()
        .ok_or_else(|| "llm database pool is not sqlite".to_string())?
        .clone();
    NativeSqlLlmStore::from_sqlite_pool(sqlite)
        .await
        .map_err(|error| error.to_string())
}
