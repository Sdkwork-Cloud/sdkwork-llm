//! SQL storage support for SDKWork LLM.

mod bootstrap;
pub mod db;

// DATABASE_SPEC.md section 34: repository-sqlx anchors the shared repository crate.
use sdkwork_database_repository as _;

pub use bootstrap::{
    bootstrap_llm_database, bootstrap_llm_database_from_env,
    bootstrap_llm_data_plane_from_env, connect_and_bootstrap_llm_database_from_env,
    LlmDataPlane,
};
pub use db::{
    connect_llm_pool_from_env, install_sqlite_schema, open_native_sql_store_from_pool,
    LlmDatabasePool,
};
