//! Shared LLM router auth wiring for sdkwork-web-framework integration.

use async_trait::async_trait;
use sdkwork_iam_web_adapter::IamWebRequestContextResolver;
use sdkwork_llm_contract::{
    llm_is_production_like_environment, llm_use_dev_inline_auth_resolver,
};
use sdkwork_web_core::{WebFrameworkError, WebRequestContextResolver, WebRequestPrincipal};

/// How HTTP routers should resolve request context from environment.
pub enum LlmWebAuthMode {
    DevInline,
    IamDatabase(IamWebRequestContextResolver),
    ProductionFailClosed,
}

/// Resolve the LLM web auth mode from runtime environment variables.
pub async fn llm_web_auth_mode_from_env() -> LlmWebAuthMode {
    if llm_use_dev_inline_auth_resolver() {
        return LlmWebAuthMode::DevInline;
    }

    let iam_database_explicitly_configured = std::env::var("SDKWORK_IAM_DATABASE_URL")
        .or_else(|_| std::env::var("SDKWORK_IAM_DATABASE_ENGINE"))
        .is_ok();

    if llm_is_production_like_environment() && !iam_database_explicitly_configured {
        return LlmWebAuthMode::ProductionFailClosed;
    }

    LlmWebAuthMode::IamDatabase(
        sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await,
    )
}

#[derive(Clone, Default)]
pub struct ProductionFailClosedResolver;

#[async_trait]
impl WebRequestContextResolver for ProductionFailClosedResolver {
    async fn resolve_api_key(
        &self,
        _raw_api_key: &str,
    ) -> Result<WebRequestPrincipal, WebFrameworkError> {
        Err(WebFrameworkError::invalid_credentials(
            "production llm auth requires IAM PostgreSQL database",
        ))
    }

    async fn resolve_access_token(
        &self,
        _raw_access_token: &str,
    ) -> Result<WebRequestPrincipal, WebFrameworkError> {
        Err(WebFrameworkError::invalid_credentials(
            "production llm auth requires IAM PostgreSQL database",
        ))
    }

    async fn resolve_oauth_bearer(
        &self,
        _raw_bearer_token: &str,
    ) -> Result<WebRequestPrincipal, WebFrameworkError> {
        Err(WebFrameworkError::invalid_credentials(
            "production llm auth requires IAM PostgreSQL database",
        ))
    }

    async fn resolve_dual_token(
        &self,
        _raw_auth_token: &str,
        _raw_access_token: &str,
    ) -> Result<WebRequestPrincipal, WebFrameworkError> {
        Err(WebFrameworkError::invalid_credentials(
            "production llm auth requires IAM PostgreSQL database",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn production_without_iam_database_uses_fail_closed_mode() {
        let _guard = sdkwork_llm_contract::runtime_env::env_test_lock();
        let previous_environment = std::env::var("SDKWORK_LLM_ENVIRONMENT").ok();
        let previous_bypass = std::env::var("SDKWORK_LLM_DEV_AUTH_BYPASS").ok();
        std::env::set_var("SDKWORK_LLM_ENVIRONMENT", "production");
        std::env::remove_var("SDKWORK_LLM_DEV_AUTH_BYPASS");
        std::env::remove_var("SDKWORK_IAM_DATABASE_URL");

        let mode = llm_web_auth_mode_from_env().await;
        assert!(matches!(mode, LlmWebAuthMode::ProductionFailClosed));

        if let Some(value) = previous_environment {
            std::env::set_var("SDKWORK_LLM_ENVIRONMENT", value);
        } else {
            std::env::remove_var("SDKWORK_LLM_ENVIRONMENT");
        }
        if let Some(value) = previous_bypass {
            std::env::set_var("SDKWORK_LLM_DEV_AUTH_BYPASS", value);
        } else {
            std::env::remove_var("SDKWORK_LLM_DEV_AUTH_BYPASS");
        }
    }
}
