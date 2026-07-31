use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;

pub type LlmDatabasePool = DatabasePool;

pub async fn connect_llm_pool_from_env() -> Result<LlmDatabasePool, PoolError> {
    // DATABASE_SPEC serviceCode `LLM` → env prefix `SDKWORK_LLM_*`
    let config = DatabaseConfig::from_env("LLM")?;
    create_pool_from_config(config).await
}

pub async fn open_native_sql_store_from_pool(
    pool: &LlmDatabasePool,
) -> Result<sdkwork_llm_plugin_native_sql::NativeSqlLlmStore, String> {
    if pool.as_postgres().is_none() {
        return Err("LLM server repository requires PostgreSQL".to_string());
    }
    NativeSqlLlmStore::from_database_pool(pool)
        .await
        .map_err(|error| error.to_string())
}
