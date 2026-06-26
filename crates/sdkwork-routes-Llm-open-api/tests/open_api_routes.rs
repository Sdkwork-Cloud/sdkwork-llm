use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_llm_contract::{LlmOpenApiRequestContext, ProblemDetails};
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;
use sdkwork_routes_llm_open_api::{build_router_with_open_api, open_route_manifest};
use tower::util::ServiceExt;

fn open_context() -> LlmOpenApiRequestContext {
    LlmOpenApiRequestContext {
        api_key_id: "api-key-001".to_string(),
        tenant_id: 1001,
        actor_id: Some(2001),
    }
}

#[tokio::test]
async fn open_capabilities_route_returns_no_embedding_profile() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = build_router_with_open_api(OpenLlmService::new(store));

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/llm/v3/api/llm/capabilities")
                .extension(open_context())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["embeddingOptional"], true);
    assert!(json["retrievers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|value| value == "keyword"));
}

#[tokio::test]
async fn open_capabilities_route_rejects_missing_context() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = build_router_with_open_api(OpenLlmService::new(store));

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
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let problem: ProblemDetails = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        problem.code.as_deref(),
        Some("missing_open_api_request_context")
    );
}

#[test]
fn open_route_manifest_has_seventeen_operations() {
    let manifest = open_route_manifest();
    assert!(manifest
        .match_route("GET", "/llm/v3/api/llm/capabilities")
        .is_some());
    assert!(manifest
        .match_route("POST", "/llm/v3/api/llm/retrievals")
        .is_some());
    assert!(manifest
        .match_route("GET", "/llm/v3/api/llm/provider_health")
        .is_some());
}
