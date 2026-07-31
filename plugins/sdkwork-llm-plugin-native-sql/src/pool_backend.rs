use crate::sqlx_compat as sqlx;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_database_sqlx::create_any_pool_from_config;
use sqlx::AnyPool;

use crate::store::NativeSqlStoreError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmSqlDialect {
    Sqlite,
    Postgres,
}

impl LlmSqlDialect {
    pub fn from_config(config: &DatabaseConfig) -> Self {
        match config.engine {
            DatabaseEngine::Postgres => Self::Postgres,
            DatabaseEngine::Sqlite => Self::Sqlite,
        }
    }
}

pub fn normalize_llm_database_url(url: &str) -> String {
    match url {
        "sqlite::memory:" | "sqlite:memory:" => "sqlite::memory:?cache=shared".to_string(),
        other => other.to_string(),
    }
}

pub fn normalize_llm_database_config(mut config: DatabaseConfig) -> DatabaseConfig {
    config.url = normalize_llm_database_url(&config.url);
    if matches!(config.engine, DatabaseEngine::Sqlite) {
        config.max_connections = 1;
    }
    config
}

pub async fn connect_any_pool(
    config: &DatabaseConfig,
) -> Result<(AnyPool, LlmSqlDialect), NativeSqlStoreError> {
    ensure_database_engine_enabled(config.engine)?;
    sqlx::any::install_default_drivers();
    let config = normalize_llm_database_config(config.clone());
    let dialect = LlmSqlDialect::from_config(&config);
    let pool = create_any_pool_from_config(config).await?;
    if matches!(dialect, LlmSqlDialect::Sqlite) {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await?;
    }
    Ok((pool, dialect))
}

fn ensure_database_engine_enabled(engine: DatabaseEngine) -> Result<(), NativeSqlStoreError> {
    match engine {
        DatabaseEngine::Postgres if !cfg!(feature = "postgres") => {
            Err(NativeSqlStoreError::DatabaseEngineDisabled {
                engine: "postgres",
                required_feature: "postgres",
            })
        }
        DatabaseEngine::Sqlite if !cfg!(feature = "client-local-sqlite") => {
            Err(NativeSqlStoreError::DatabaseEngineDisabled {
                engine: "sqlite",
                required_feature: "client-local-sqlite",
            })
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiled_database_engines_match_declared_features() {
        assert_eq!(
            ensure_database_engine_enabled(DatabaseEngine::Postgres).is_ok(),
            cfg!(feature = "postgres")
        );
        assert_eq!(
            ensure_database_engine_enabled(DatabaseEngine::Sqlite).is_ok(),
            cfg!(feature = "client-local-sqlite")
        );
    }
}
