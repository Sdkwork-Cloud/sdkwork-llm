//! SDKWork LLM database pool bootstrap via `sdkwork-database`.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::create_pool_from_config;
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;

pub use sdkwork_llm_database_host::{
    bootstrap_llm_database, bootstrap_llm_database_from_env, LlmDatabaseHost,
};

use crate::db::{
    connect_llm_pool_from_env, install_sqlite_schema, open_native_sql_store_from_pool,
    LlmDatabasePool,
};

pub struct LlmDataPlane {
    pub pool: LlmDatabasePool,
    pub store: NativeSqlLlmStore,
}

pub async fn connect_and_bootstrap_llm_database_from_env() -> Result<LlmDatabaseHost, String> {
    let config = DatabaseConfig::from_env("LLM")
        .map_err(|error| error.to_string())?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_llm_database(pool).await
}

/// Single bootstrap entry for the API server and integration tests.
pub async fn bootstrap_llm_data_plane_from_env() -> Result<LlmDataPlane, String> {
    let pool = connect_llm_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;

    if pool.as_postgres().is_some() {
        bootstrap_llm_database(pool.clone()).await?;
    } else {
        install_sqlite_schema(&pool).await?;
    }

    let store = open_native_sql_store_from_pool(&pool).await?;
    Ok(LlmDataPlane { pool, store })
}
