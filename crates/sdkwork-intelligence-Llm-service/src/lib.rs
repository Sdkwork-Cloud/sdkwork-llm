//! Business service boundary for SDKWork LLM HTTP runtime.

mod app_backend_api;
mod backend_admin_api;
mod candidate_promotion;
mod open_api;
mod platform;

pub use open_api::OpenLlmService;

pub type LlmProductService = OpenLlmService;

use sdkwork_llm_spi::error::LlmSpiError;

#[derive(Debug, Default)]
pub struct LlmService;

impl LlmService {
    pub fn new() -> Self {
        Self
    }

    pub fn health_check(&self) -> Result<&'static str, LlmSpiError> {
        Ok("ok")
    }
}
