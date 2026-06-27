use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_llm_contract::{
    LlmCapabilities, LlmImplementationKind, LlmOpenApi, LlmOpenApiRequestContext,
    LlmProviderInterface, LlmRetrieverKind, LlmServiceResult,
};
use sdkwork_routes_llm_open_api::{
    build_router_with_shared_open_api, wrap_router_with_web_framework,
};
use sdkwork_web_core::DefaultWebRequestContextResolver;
use std::sync::{Arc, Mutex};
use tower::util::ServiceExt;

const DEV_API_KEY: &str = "api_key_id=dev-key;tenant_id=100_001;user_id=1;app_id=sdkwork-llm";

#[tokio::test]
async fn open_router_web_framework_rejects_unauthenticated_requests() {
    let app = wrap_router_with_web_framework(
        DefaultWebRequestContextResolver::default(),
        build_router_with_shared_open_api(Arc::new(RecordingOpenApi::default())),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/llm/v3/api/llm/capabilities")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn open_router_web_framework_accepts_dev_inline_api_key_before_handler() {
    let service = RecordingOpenApi::default();
    let app = wrap_router_with_web_framework(
        DefaultWebRequestContextResolver::default(),
        build_router_with_shared_open_api(Arc::new(service.clone())),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/llm/v3/api/llm/capabilities")
                .header("x-api-key", DEV_API_KEY)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(service.contexts(), vec![("dev-key".to_owned(), 100_001)]);
}

#[derive(Clone, Default)]
struct RecordingOpenApi {
    contexts: Arc<Mutex<Vec<(String, u64)>>>,
}

impl RecordingOpenApi {
    fn contexts(&self) -> Vec<(String, u64)> {
        self.contexts.lock().unwrap().clone()
    }
}

#[async_trait]
impl LlmOpenApi for RecordingOpenApi {
    async fn retrieve_capabilities(
        &self,
        ctx: LlmOpenApiRequestContext,
    ) -> LlmServiceResult<LlmCapabilities> {
        self.contexts
            .lock()
            .unwrap()
            .push((ctx.api_key_id, ctx.tenant_id));
        Ok(LlmCapabilities {
            embedding_optional: true,
            retrievers: vec![LlmRetrieverKind::Keyword],
            provider_interfaces: vec![LlmProviderInterface::Memory],
            implementation_kinds: vec![LlmImplementationKind::NativeSql],
            open_api_prefix: "/llm/v3/api".to_string(),
            sdk_family: "sdkwork-llm-sdk".to_string(),
            checked_at: "2026-06-10T00:00:00Z".to_string(),
            metadata: None,
        })
    }
}
