use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;
use sdkwork_routes_llm_open_api::{build_router_with_open_api, wrap_router_with_web_framework};
use sdkwork_web_core::DefaultWebRequestContextResolver;
use tower::util::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=100_001;user_id=2001;session_id=s-1;app_id=sdkwork-llm;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=100_001;user_id=2001;session_id=s-1;app_id=sdkwork-llm;environment=dev;deployment_mode=saas";
const DEV_API_KEY: &str = "api_key_id=key-1;tenant_id=100_001;user_id=2001;app_id=sdkwork-llm";

async fn wrapped_open_api_router() -> axum::Router {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let business = build_router_with_open_api(OpenLlmService::new(store));
    wrap_router_with_web_framework(DefaultWebRequestContextResolver::default(), business)
}

#[tokio::test]
async fn open_api_rejects_missing_api_key() {
    let app = wrapped_open_api_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/llm/v3/api/llm/capabilities")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn open_api_does_not_accept_dual_token_fallback() {
    let app = wrapped_open_api_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/llm/v3/api/llm/capabilities")
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn open_api_accepts_api_key_before_handler() {
    let app = wrapped_open_api_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/llm/v3/api/llm/capabilities")
                .header("X-API-Key", DEV_API_KEY)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
