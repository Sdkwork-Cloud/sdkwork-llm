use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_llm_contract::LlmBackendRequestContext;
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;
use sdkwork_routes_llm_backend_api::{backend_route_manifest, build_router_with_backend_api};
use tower::util::ServiceExt;

fn backend_context() -> LlmBackendRequestContext {
    LlmBackendRequestContext {
        tenant_id: 100_001,
        operator_id: Some(9001),
    }
}

#[tokio::test]
async fn backend_provider_health_route_returns_healthy() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = build_router_with_backend_api(OpenLlmService::new(store));

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/llm/provider_health")
                .extension(backend_context())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "healthy");
}

#[test]
fn backend_route_manifest_resolves_provider_health_route() {
    let manifest = backend_route_manifest();
    assert!(manifest
        .match_route("GET", "/backend/v3/api/llm/provider_health")
        .is_some());
}
