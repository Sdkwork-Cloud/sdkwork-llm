//! SDKWork LLM native SQL runtime plugin.

pub mod admin_tables;
pub mod manifest;
pub mod pool_backend;
mod sqlx_compat;
pub mod store;

pub use admin_tables::*;
pub use manifest::*;
pub use pool_backend::{connect_any_pool, normalize_llm_database_config, LlmSqlDialect};
pub use store::*;
