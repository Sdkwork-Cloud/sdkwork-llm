use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_iam_web_adapter::IamWebRequestContextResolver;
use sdkwork_llm_contract::{
    LlmBackendApi, LlmBackendRequestContext, LlmProviderHealth,
    LlmProviderHealthStatus, LlmServiceResult,
};
use sdkwork_routes_llm_backend_api::{
    build_router_with_shared_backend_api, wrap_router_with_iam_database_web_framework,
};
use std::sync::{Arc, Mutex};
use tower::util::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=100_001;user_id=9001;session_id=s-1;app_id=sdkwork-llm;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=100_001;user_id=9001;session_id=s-1;app_id=sdkwork-llm;environment=dev;deployment_mode=saas";

#[tokio::test]
async fn backend_router_web_framework_rejects_unauthenticated_requests() {
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_shared_backend_api(Arc::new(RecordingBackendApi::default())),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/llm/provider_health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn backend_router_web_framework_accepts_dev_inline_dual_tokens_before_handler() {
    let service = RecordingBackendApi::default();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_shared_backend_api(Arc::new(service.clone())),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/llm/provider_health")
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(service.tenant_ids(), vec![100_001]);
}

#[derive(Clone, Default)]
struct RecordingBackendApi {
    tenant_ids: Arc<Mutex<Vec<u64>>>,
}

impl RecordingBackendApi {
    fn tenant_ids(&self) -> Vec<u64> {
        self.tenant_ids.lock().unwrap().clone()
    }
}

#[async_trait]
impl LlmBackendApi for RecordingBackendApi {
    async fn retrieve_provider_health(
        &self,
        ctx: LlmBackendRequestContext,
    ) -> LlmServiceResult<LlmProviderHealth> {
        self.tenant_ids.lock().unwrap().push(ctx.tenant_id);
        Ok(LlmProviderHealth {
            status: LlmProviderHealthStatus::Healthy,
            checked_at: "2026-06-10T00:00:00Z".to_string(),
            providers: vec![],
        })
    }
}
