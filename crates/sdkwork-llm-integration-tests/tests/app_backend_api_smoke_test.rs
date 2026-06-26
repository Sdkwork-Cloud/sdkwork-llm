use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_iam_web_adapter::IamWebRequestContextResolver;
use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;
use sdkwork_routes_llm_app_api::{
    build_router_with_app_api, wrap_router_with_iam_database_web_framework,
};
use sdkwork_routes_llm_backend_api::{
    build_router_with_backend_api,
    wrap_router_with_iam_database_web_framework as wrap_backend_router,
};
use tower::util::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=1001;user_id=2001;session_id=s-1;app_id=sdkwork-llm;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=1001;user_id=2001;session_id=s-1;app_id=sdkwork-llm;environment=dev;deployment_mode=saas";

#[tokio::test]
async fn app_api_rejects_unauthenticated_requests() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/app/v3/api/llm/learning_settings")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn app_api_rejects_auth_token_without_access_token() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/app/v3/api/llm/learning_settings")
                .header("Authorization", DEV_AUTH_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn app_api_accepts_dual_token_context() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/app/v3/api/llm/learning_settings")
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn backend_api_rejects_unauthenticated_requests() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = wrap_backend_router(
        IamWebRequestContextResolver::new(None),
        build_router_with_backend_api(OpenLlmService::new(store)),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/llm/provider_health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn backend_api_accepts_dual_token_context() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = wrap_backend_router(
        IamWebRequestContextResolver::new(None),
        build_router_with_backend_api(OpenLlmService::new(store)),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/llm/provider_health")
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
