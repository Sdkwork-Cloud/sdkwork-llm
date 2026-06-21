use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_llm_contract::{
    LlmContextPackRequest, LlmOpenApi, LlmOpenApiRequestContext, LlmRecordRequest,
    LlmRetrievalRequest, LlmRecordType,
};
use sdkwork_llm_plugin_native_sql::NativeSqlLlmStore;

fn open_context() -> LlmOpenApiRequestContext {
    LlmOpenApiRequestContext {
        api_key_id: "api-key-001".to_string(),
        tenant_id: 1001,
        actor_id: Some(2001),
    }
}

#[tokio::test]
async fn remembers_retrieves_and_builds_context_without_embeddings() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let service = OpenLlmService::new(store);
    let context = open_context();

    service
        .create_memory(
            context.clone(),
            LlmRecordRequest {
                space_id: 1,
                scope: "user".to_string(),
                record_type: LlmRecordType::Semantic,
                subject: None,
                predicate: None,
                object_text: Some("concise".to_string()),
                canonical_text: "User prefers concise answers".to_string(),
                summary_text: None,
                user_id: None,
                language: None,
                sensitivity_level: None,
                metadata: None,
                tags: None,
            },
        )
        .await
        .expect("create llm record");

    let retrieval = service
        .create_retrieval(
            context.clone(),
            LlmRetrievalRequest {
                query: "concise answers".to_string(),
                space_ids: vec![1],
                actor_id: None,
                retrieval_profile_id: None,
                record_types: None,
                filters: None,
                top_k: 5,
                context_budget_tokens: 512,
                include_trace: None,
            },
        )
        .await
        .expect("retrieve");

    assert!(retrieval
        .hits
        .iter()
        .any(|hit| hit.retriever_name == "keyword"));
    assert!(!retrieval
        .hits
        .iter()
        .any(|hit| hit.retriever_name == "vector"));

    let pack = service
        .create_context_pack(
            context,
            LlmContextPackRequest {
                query: "concise answers".to_string(),
                space_ids: vec![1],
                actor_id: None,
                retrieval_profile_id: None,
                context_budget_tokens: 512,
                include_citations: None,
                filters: None,
            },
        )
        .await
        .expect("context pack");

    let fragments = pack.pack["fragments"].as_array().expect("fragments");
    assert!(fragments.iter().any(|fragment| {
        fragment["canonicalText"]
            .as_str()
            .unwrap_or("")
            .contains("concise")
    }));
    assert_eq!(pack.pack["embeddingOptional"], true);
}
