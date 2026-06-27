use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use sdkwork_iam_web_adapter::IamWebRequestContextResolver;
use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;
use sdkwork_llm_spi::{LlmHabitStorePort, LlmScopeContext, UpsertLlmHabitCommand};
use sdkwork_routes_llm_app_api::{
    build_router_with_app_api, wrap_router_with_iam_database_web_framework,
};
use serde_json::json;
use tower::util::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=100_001;user_id=1;session_id=s-1;app_id=sdkwork-llm;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=100_001;user_id=1;session_id=s-1;app_id=sdkwork-llm;environment=dev;deployment_mode=saas";

fn authed_json_request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header("Authorization", DEV_AUTH_TOKEN)
        .header("Access-Token", DEV_ACCESS_TOKEN)
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn authed_get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .header("Authorization", DEV_AUTH_TOKEN)
        .header("Access-Token", DEV_ACCESS_TOKEN)
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn app_api_mvp_flow_space_memory_and_retrieval_via_dual_token() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let create_space = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/spaces",
            json!({
                "ownerSubjectType": "user",
                "ownerSubjectId": "2001",
                "spaceType": "personal",
                "displayName": "Personal memory space"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(create_space.status(), StatusCode::CREATED);
    let space_body = to_bytes(create_space.into_body(), usize::MAX)
        .await
        .unwrap();
    let space_json: serde_json::Value = serde_json::from_slice(&space_body).unwrap();
    let space_id = space_json["spaceId"].as_str().unwrap();

    let create_memory = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/records",
            json!({
                "spaceId": space_id,
                "scope": "user",
                "recordType": "semantic",
                "canonicalText": "User prefers concise answers"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(create_memory.status(), StatusCode::CREATED);

    let retrieval = app
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/retrievals",
            json!({
                "query": "concise answers",
                "spaceIds": [space_id],
                "topK": 5,
                "contextBudgetTokens": 512
            }),
        ))
        .await
        .unwrap();
    assert_eq!(retrieval.status(), StatusCode::CREATED);
    let retrieval_body = to_bytes(retrieval.into_body(), usize::MAX).await.unwrap();
    let retrieval_json: serde_json::Value = serde_json::from_slice(&retrieval_body).unwrap();
    assert!(retrieval_json["hits"]
        .as_array()
        .unwrap()
        .iter()
        .any(|hit| hit["retrieverName"] == "keyword"));
}

#[tokio::test]
async fn app_api_habit_confirm_flow_via_dual_token() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);
    LlmHabitStorePort::upsert(
        &store,
        UpsertLlmHabitCommand {
            scope: scope.clone(),
            habit_id: "9001".to_string(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
            habit_type: "preference".to_string(),
            description: "Prefers concise answers".to_string(),
            stage: "candidate".to_string(),
            strength: 0.4,
            confidence: 0.8,
            support_count: 2,
            metadata_json: None,
        },
    )
    .await
    .expect("seed habit");

    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let confirm = app
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/habits/9001/confirm",
            json!({}),
        ))
        .await
        .unwrap();
    assert_eq!(confirm.status(), StatusCode::OK);
    let confirm_body = to_bytes(confirm.into_body(), usize::MAX).await.unwrap();
    let confirm_json: serde_json::Value = serde_json::from_slice(&confirm_body).unwrap();
    assert_eq!(confirm_json["stage"], "confirmed");
}

#[tokio::test]
async fn app_api_memory_sources_list_returns_linked_event_sources() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let pool = store.pool().clone();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let create_space = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/spaces",
            json!({
                "ownerSubjectType": "user",
                "ownerSubjectId": "2001",
                "spaceType": "personal",
                "displayName": "Source test space"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(create_space.status(), StatusCode::CREATED);
    let space_body = to_bytes(create_space.into_body(), usize::MAX)
        .await
        .unwrap();
    let space_json: serde_json::Value = serde_json::from_slice(&space_body).unwrap();
    let space_id = space_json["spaceId"].as_str().unwrap();

    let create_memory = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/records",
            json!({
                "spaceId": space_id,
                "scope": "user",
                "recordType": "semantic",
                "canonicalText": "User prefers concise answers"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(create_memory.status(), StatusCode::CREATED);
    let memory_body = to_bytes(create_memory.into_body(), usize::MAX)
        .await
        .unwrap();
    let memory_json: serde_json::Value = serde_json::from_slice(&memory_body).unwrap();
    let record_id = memory_json["recordId"].as_str().unwrap();

    let seed_store = NativeSqlLlmStore::from_sqlite_pool(pool).await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, space_id.parse().unwrap());
    seed_store
        .append_open_api_event(
            &scope,
            "8001",
            "message.user",
            "chat",
            "2026-06-10T00:00:00Z",
            &json!({ "text": "keep answers concise" }),
        )
        .await
        .expect("seed event");
    seed_store
        .append_record_source_for_tenant(100_001, "8101", record_id, "8001", "evidence", Some(0.2))
        .await
        .expect("seed record source");

    let sources = app
        .oneshot(authed_get_request(&format!(
            "/app/v3/api/llm/records/{record_id}/sources"
        )))
        .await
        .unwrap();
    assert_eq!(sources.status(), StatusCode::OK);
    let sources_body = to_bytes(sources.into_body(), usize::MAX).await.unwrap();
    let sources_json: serde_json::Value = serde_json::from_slice(&sources_body).unwrap();
    let items = sources_json["items"].as_array().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["sourceId"], "8101");
    assert_eq!(items[0]["eventId"], "8001");
    assert_eq!(items[0]["sourceRole"], "evidence");
}

#[tokio::test]
async fn app_api_candidate_approve_promotes_memory_and_links_event_sources() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let pool = store.pool().clone();
    let app = wrap_router_with_iam_database_web_framework(
        IamWebRequestContextResolver::new(None),
        build_router_with_app_api(OpenLlmService::new(store)),
    );

    let create_space = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/spaces",
            json!({
                "ownerSubjectType": "user",
                "ownerSubjectId": "2001",
                "spaceType": "personal",
                "displayName": "Candidate promotion space"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(create_space.status(), StatusCode::CREATED);
    let space_body = to_bytes(create_space.into_body(), usize::MAX)
        .await
        .unwrap();
    let space_json: serde_json::Value = serde_json::from_slice(&space_body).unwrap();
    let space_id = space_json["spaceId"].as_str().unwrap();

    let seed_store = NativeSqlLlmStore::from_sqlite_pool(pool).await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, space_id.parse().unwrap());
    seed_store
        .append_open_api_event(
            &scope,
            "7001",
            "message.user",
            "chat",
            "2026-06-10T00:00:00Z",
            &json!({ "content": "User prefers concise answers" }),
        )
        .await
        .expect("seed event");

    let extraction = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/app/v3/api/llm/extractions",
            json!({
                "spaceId": space_id,
                "inputEvents": ["7001"],
                "extractionMode": "deterministic"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(extraction.status(), StatusCode::CREATED);

    let candidates = app
        .clone()
        .oneshot(authed_get_request(&format!(
            "/app/v3/api/llm/candidates?spaceId={space_id}"
        )))
        .await
        .unwrap();
    assert_eq!(candidates.status(), StatusCode::OK);
    let candidates_body = to_bytes(candidates.into_body(), usize::MAX).await.unwrap();
    let candidates_json: serde_json::Value = serde_json::from_slice(&candidates_body).unwrap();
    let candidate_id = candidates_json["items"][0]["candidateId"]
        .as_str()
        .expect("candidate id");

    let approve = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/app/v3/api/llm/candidates/{candidate_id}/approve"),
            json!({}),
        ))
        .await
        .unwrap();
    assert_eq!(approve.status(), StatusCode::OK);
    let approve_body = to_bytes(approve.into_body(), usize::MAX).await.unwrap();
    let approve_json: serde_json::Value = serde_json::from_slice(&approve_body).unwrap();
    assert_eq!(approve_json["decisionState"], "approved");

    let memories = app
        .clone()
        .oneshot(authed_get_request(&format!(
            "/app/v3/api/llm/records?spaceId={space_id}"
        )))
        .await
        .unwrap();
    assert_eq!(memories.status(), StatusCode::OK);
    let memories_body = to_bytes(memories.into_body(), usize::MAX).await.unwrap();
    let memories_json: serde_json::Value = serde_json::from_slice(&memories_body).unwrap();
    let record_id = memories_json["items"][0]["recordId"].as_str().unwrap();

    let sources = app
        .oneshot(authed_get_request(&format!(
            "/app/v3/api/llm/records/{record_id}/sources"
        )))
        .await
        .unwrap();
    assert_eq!(sources.status(), StatusCode::OK);
    let sources_body = to_bytes(sources.into_body(), usize::MAX).await.unwrap();
    let sources_json: serde_json::Value = serde_json::from_slice(&sources_body).unwrap();
    assert_eq!(sources_json["items"][0]["eventId"], "7001");
    assert_eq!(sources_json["items"][0]["sourceRole"], "evidence");
}
