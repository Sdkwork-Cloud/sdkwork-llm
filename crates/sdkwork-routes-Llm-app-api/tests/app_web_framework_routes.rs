use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_iam_web_adapter::IamWebRequestContextResolver;
use sdkwork_llm_contract::{
    LlmAppApi, LlmAppRequestContext, LlmLearningSettings, LlmServiceResult,
};
use sdkwork_routes_llm_app_api::{
    build_router_with_shared_app_api, wrap_router_with_iam_database_web_framework,
};
use std::sync::{Arc, Mutex};
use tower::util::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=100_001;user_id=1;session_id=s-1;app_id=sdkwork-llm;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=100_001;user_id=1;session_id=s-1;app_id=sdkwork-llm;environment=dev;deployment_mode=saas";

#[tokio::test]
async fn app_router_web_framework_rejects_unauthenticated_requests() {
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_shared_app_api(Arc::new(RecordingAppApi::default())),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/llm/learning_settings")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn app_router_web_framework_accepts_dev_inline_dual_tokens_before_handler() {
    let service = RecordingAppApi::default();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_shared_app_api(Arc::new(service.clone())),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/llm/learning_settings")
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
struct RecordingAppApi {
    tenant_ids: Arc<Mutex<Vec<u64>>>,
}

impl RecordingAppApi {
    fn tenant_ids(&self) -> Vec<u64> {
        self.tenant_ids.lock().unwrap().clone()
    }
}

#[async_trait]
impl LlmAppApi for RecordingAppApi {
    async fn retrieve_learning_settings(
        &self,
        ctx: LlmAppRequestContext,
    ) -> LlmServiceResult<LlmLearningSettings> {
        self.tenant_ids.lock().unwrap().push(ctx.tenant_id);
        Ok(LlmLearningSettings {
            auto_promote_candidates: false,
            habit_learning_enabled: true,
            updated_at: "2026-06-10T00:00:00Z".to_string(),
        })
    }
}
