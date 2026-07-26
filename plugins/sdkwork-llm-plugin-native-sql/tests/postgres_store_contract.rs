//! Optional PostgreSQL contract coverage using connection-local temporary tables.

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_llm_plugin_native_sql::{NativeSqlAppendOutboxEventCommand, NativeSqlLlmStore};
use sdkwork_llm_spi::{
    AppendLlmRetrievalTraceCommand, ApproveLlmCandidateCommand, CreateLlmCandidateCommand,
    LlmContextPackSnapshot, LlmRetrievalHitDraft, LlmRetrievalTraceStorePort, LlmScopeContext,
    RetrieveLlmRetrievalTraceQuery, UpsertLlmHabitCommand,
};

async fn postgres_store() -> Option<NativeSqlLlmStore> {
    let url = match std::env::var("SDKWORK_LLM_POSTGRES_TEST_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ => return None,
    };
    let config = DatabaseConfig {
        engine: DatabaseEngine::Postgres,
        url,
        max_connections: 1,
        ..DatabaseConfig::default()
    };
    let store = NativeSqlLlmStore::connect(&config)
        .await
        .expect("connect to PostgreSQL LLM contract schema");

    for table in [
        "llm_space",
        "llm_event",
        "llm_record",
        "llm_retrieval_trace",
        "llm_retrieval_hit",
        "llm_context_pack",
        "llm_index",
        "llm_retrieval_profile",
        "llm_implementation_profile",
        "llm_provider_binding",
        "llm_eval_run",
        "llm_audit_log",
        "llm_outbox_event",
        "llm_candidate",
        "llm_habit",
        "llm_record_source",
    ] {
        sqlx::query(&format!(
            "CREATE TEMP TABLE {table} (LIKE {table} INCLUDING ALL)"
        ))
        .execute(store.pool())
        .await
        .unwrap_or_else(|error| panic!("create temporary {table}: {error}"));
    }

    Some(store)
}

#[tokio::test]
async fn postgres_store_round_trips_rich_types_without_persistent_rows() {
    let Some(store) = postgres_store().await else {
        return;
    };
    let scope = LlmScopeContext::for_test(100_001, 91_001);

    store
        .append_event(&scope, "pg-event-1", "PostgreSQL JSON and timestamp probe")
        .await
        .expect("append PostgreSQL event");
    store
        .create_record(&scope, "pg-record-1", "answer_style", "concise")
        .await
        .expect("create PostgreSQL record");

    let event = store
        .retrieve_event(&scope, "pg-event-1")
        .await
        .expect("retrieve PostgreSQL event")
        .expect("PostgreSQL event must exist");
    let record = store
        .retrieve_record(&scope, "pg-record-1")
        .await
        .expect("retrieve PostgreSQL record")
        .expect("PostgreSQL record must exist");
    assert_eq!(event.content, "PostgreSQL JSON and timestamp probe");
    assert_eq!(record.content, "concise");

    store
        .append_open_api_event(
            &scope,
            "pg-event-open-1",
            "llm.observation",
            "contract-test",
            "2026-07-25T00:00:00.000Z",
            &serde_json::json!({"content":"open event"}),
        )
        .await
        .expect("append PostgreSQL open API event");
    assert!(store
        .retrieve_open_api_event_for_tenant(scope.tenant_id, "pg-event-open-1")
        .await
        .expect("retrieve PostgreSQL open API event")
        .is_some());

    LlmRetrievalTraceStorePort::append(
        &store,
        AppendLlmRetrievalTraceCommand {
            scope: scope.clone(),
            trace_id: "pg-trace-1".to_string(),
            actor_id: Some("user-42".to_string()),
            query_text: Some("PostgreSQL boolean probe".to_string()),
            query_hash: "hash:pg-trace-1".to_string(),
            retrievers_json: Some(r#"["native_sql"]"#.to_string()),
            latency_ms: Some(17),
            degraded: true,
            metadata_json: Some(r#"{"profile":"native_sql"}"#.to_string()),
            hits: vec![LlmRetrievalHitDraft {
                hit_id: "pg-hit-1".to_string(),
                record_id: Some("pg-record-1".to_string()),
                retriever_name: "native_sql".to_string(),
                result_rank: 1,
                raw_score: Some(0.75),
                fused_score: Some(0.9),
                explanation_json: Some(r#"{"match":"keyword"}"#.to_string()),
                status: "selected".to_string(),
            }],
            context_pack: Some(LlmContextPackSnapshot {
                context_pack_id: "pg-pack-1".to_string(),
                pack_json: r#"{"recordIds":["pg-record-1"]}"#.to_string(),
                estimated_tokens: 8,
                truncated: true,
            }),
        },
    )
    .await
    .expect("append PostgreSQL retrieval trace");

    let trace = LlmRetrievalTraceStorePort::retrieve(
        &store,
        RetrieveLlmRetrievalTraceQuery {
            scope: scope.clone(),
            trace_id: "pg-trace-1".to_string(),
        },
    )
    .await
    .expect("retrieve PostgreSQL retrieval trace")
    .expect("PostgreSQL retrieval trace must exist");
    assert!(trace.degraded);
    assert!(trace.context_pack.expect("context pack").truncated);

    store
        .insert_llm_index(
            100_001,
            "pg-index-1",
            Some(91_001),
            "keyword",
            "1",
            "active",
            Some(r#"{"analyzer":"standard"}"#),
        )
        .await
        .expect("insert PostgreSQL admin index");
    let indexes = store
        .list_llm_indexes_for_tenant(100_001, 10)
        .await
        .expect("list PostgreSQL admin indexes");
    assert_eq!(indexes.len(), 1);
    assert_eq!(indexes[0].index_uuid, "pg-index-1");

    store
        .append_audit_with_metadata(
            &scope,
            "pg-audit-1",
            "llm.contract.probed",
            "llm_record",
            "pg-record-1",
            "success",
            r#"{"driver":"postgres"}"#,
        )
        .await
        .expect("append PostgreSQL audit");
    store
        .append_outbox_event(NativeSqlAppendOutboxEventCommand {
            scope: &scope,
            outbox_id: "pg-outbox-1",
            aggregate_type: "llm_record",
            aggregate_id: "pg-record-1",
            event_type: "llm.record.created",
            event_version: "1",
            payload_json: r#"{"recordId":"pg-record-1"}"#,
        })
        .await
        .expect("append PostgreSQL outbox event");
    let outbox = store
        .mark_outbox_published(&scope, "pg-outbox-1")
        .await
        .expect("publish PostgreSQL outbox event")
        .expect("PostgreSQL outbox event must exist");
    assert_eq!(outbox.publish_state, "published");

    let candidate = CreateLlmCandidateCommand {
        scope: scope.clone(),
        candidate_id: "pg-candidate-1".to_string(),
        candidate_type: "observation".to_string(),
        record_type: "semantic".to_string(),
        proposed_text: "Prefer concise answers".to_string(),
        proposed_payload_json: Some(r#"{"preference":"concise"}"#.to_string()),
        evidence_json: Some(r#"{"eventId":"pg-event-1"}"#.to_string()),
        confidence: 0.91,
    };
    store
        .create_candidate(&candidate)
        .await
        .expect("create PostgreSQL candidate");
    let approved = store
        .approve_candidate(&ApproveLlmCandidateCommand {
            scope: scope.clone(),
            candidate_id: candidate.candidate_id,
            decision_reason: Some("confirmed".to_string()),
            decided_by: Some(42),
        })
        .await
        .expect("approve PostgreSQL candidate")
        .expect("PostgreSQL candidate must exist");
    assert_eq!(approved.decision_state, "approved");

    let habit = store
        .upsert_habit(&UpsertLlmHabitCommand {
            scope: scope.clone(),
            habit_id: "pg-habit-1".to_string(),
            user_id: 42,
            habit_key: "answer_style:concise".to_string(),
            habit_type: "preference".to_string(),
            description: "Prefers concise answers".to_string(),
            stage: "candidate".to_string(),
            strength: 0.4,
            confidence: 0.8,
            support_count: 2,
            metadata_json: Some(r#"{"source":"signals"}"#.to_string()),
        })
        .await
        .expect("upsert PostgreSQL habit");
    assert_eq!(habit.habit_id, "pg-habit-1");

    store
        .insert_llm_retrieval_profile(
            scope.tenant_id,
            "pg-profile-1",
            Some(scope.space_id),
            "Contract profile",
            "hybrid",
            r#"["keyword"]"#,
            10,
            2048,
            "active",
        )
        .await
        .expect("insert PostgreSQL retrieval profile");
    store
        .insert_llm_implementation_profile(
            scope.tenant_id,
            "pg-implementation-1",
            "Native SQL",
            "native_sql",
            "primary",
            "active",
            r#"{"keyword":true}"#,
        )
        .await
        .expect("insert PostgreSQL implementation profile");
    store
        .insert_llm_provider_binding(
            scope.tenant_id,
            "pg-provider-1",
            "native",
            "postgres",
            "PostgreSQL",
            "healthy",
        )
        .await
        .expect("insert PostgreSQL provider binding");
    store
        .insert_llm_eval_run(
            scope.tenant_id,
            "pg-eval-1",
            "retrieval_quality",
            "queued",
            Some(r#"{"precision":1.0}"#),
        )
        .await
        .expect("insert PostgreSQL eval run");
    assert_eq!(
        store
            .list_llm_retrieval_profiles_for_tenant(scope.tenant_id, 10)
            .await
            .expect("list PostgreSQL retrieval profiles")
            .len(),
        1
    );
    assert_eq!(
        store
            .list_llm_implementation_profiles_for_tenant(scope.tenant_id, 10)
            .await
            .expect("list PostgreSQL implementation profiles")
            .len(),
        1
    );
    assert_eq!(
        store
            .list_llm_provider_bindings_for_tenant(scope.tenant_id, 10)
            .await
            .expect("list PostgreSQL provider bindings")
            .len(),
        1
    );
    assert_eq!(
        store
            .list_llm_eval_runs_for_tenant(scope.tenant_id, 10)
            .await
            .expect("list PostgreSQL eval runs")
            .len(),
        1
    );
}
