use std::sync::OnceLock;

use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_llm_contract::{LlmServiceError, LlmServiceResult};

static ID_GENERATOR: OnceLock<SnowflakeIdGenerator> = OnceLock::new();

fn id_generator() -> &'static SnowflakeIdGenerator {
    ID_GENERATOR.get_or_init(|| {
        let node_id = std::env::var("SDKWORK_LLM_SNOWFLAKE_NODE_ID")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        SnowflakeIdGenerator::new(node_id).expect("memory snowflake generator must initialize")
    })
}

pub fn next_numeric_id() -> LlmServiceResult<u64> {
    let id = id_generator()
        .generate()
        .map_err(|error| LlmServiceError::storage(format!("id generation failed: {error}")))?;
    u64::try_from(id)
        .map_err(|_| LlmServiceError::storage(format!("id out of u64 range: {id}")))
}

pub fn current_timestamp() -> String {
    sdkwork_utils_rust::format_datetime(sdkwork_utils_rust::now(), None)
}

pub fn parse_numeric_id(value: &str) -> Option<u64> {
    value.parse().ok()
}
