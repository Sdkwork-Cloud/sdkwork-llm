use sdkwork_llm_plugin_native_sql::{
    build_native_sql_candidate_store, build_native_sql_habit_store,
    build_native_sql_retrieval_trace_store, NativeSqlAppendOutboxEventCommand,
    NativeSqlLlmStore, NativeSqlStoreError,
};
use sdkwork_llm_spi::{
    AppendLlmAuditCommand, AppendLlmEventCommand, AppendLlmOutboxCommand,
    AppendLlmRetrievalTraceCommand, ApproveLlmCandidateCommand, CreateLlmCandidateCommand,
    CreateLlmRecordCommand, DecayLlmHabitCommand, DeleteLlmRecordCommand,
    ListLlmRetrievalTracesQuery, ListPendingLlmOutboxQuery, MarkLlmOutboxFailedCommand,
    MarkLlmOutboxPublishedCommand, LlmAuditStorePort, LlmCandidateStorePort,
    LlmContextPackSnapshot, LlmEventStorePort, LlmHabitStorePort, LlmOutboxStorePort,
    LlmRecordStorePort, LlmRetrievalHitDraft, LlmRetrievalTraceStorePort,
    LlmScopeContext, LlmSpiError, PromoteLlmHabitCommand, RejectLlmCandidateCommand,
    RetrieveLlmAuditQuery, RetrieveLlmCandidateQuery, RetrieveLlmEventQuery,
    RetrieveLlmHabitQuery, RetrieveLlmOutboxQuery, RetrieveLlmRecordQuery,
    RetrieveLlmRetrievalTraceQuery, UpsertLlmHabitCommand,
};

fn assert_utc_timestamp(value: Option<&str>) {
    let Some(text) = value else {
        panic!("expected UTC timestamp");
    };
    assert!(text.ends_with('Z'), "timestamp must be UTC RFC3339: {text}");
}

fn outbox_command<'a>(
    scope: &'a LlmScopeContext,
    outbox_id: &'a str,
    aggregate_id: &'a str,
    payload_json: &'a str,
) -> NativeSqlAppendOutboxEventCommand<'a> {
    NativeSqlAppendOutboxEventCommand {
        scope,
        outbox_id,
        aggregate_type: "llm_record",
        aggregate_id,
        event_type: "llm.record.created",
        event_version: "1",
        payload_json,
    }
}

fn candidate_command(
    scope: LlmScopeContext,
    candidate_id: &str,
) -> CreateLlmCandidateCommand {
    CreateLlmCandidateCommand {
        scope,
        candidate_id: candidate_id.to_string(),
        candidate_type: "observation".to_string(),
        record_type: "semantic".to_string(),
        proposed_text: "User prefers concise answers".to_string(),
        proposed_payload_json: Some(r#"{"preference":"concise"}"#.to_string()),
        evidence_json: Some(r#"{"eventId":"evt-1"}"#.to_string()),
        confidence: 0.91,
    }
}

fn habit_command(
    scope: LlmScopeContext,
    habit_id: &str,
    user_id: i64,
) -> UpsertLlmHabitCommand {
    UpsertLlmHabitCommand {
        scope,
        habit_id: habit_id.to_string(),
        user_id,
        habit_key: "answer_style:concise".to_string(),
        habit_type: "preference".to_string(),
        description: "Prefers concise answers".to_string(),
        stage: "candidate".to_string(),
        strength: 0.4,
        confidence: 0.8,
        support_count: 2,
        metadata_json: Some(r#"{"source":"signals"}"#.to_string()),
    }
}

fn retrieval_trace_command(
    scope: LlmScopeContext,
    trace_id: &str,
) -> AppendLlmRetrievalTraceCommand {
    AppendLlmRetrievalTraceCommand {
        scope,
        trace_id: trace_id.to_string(),
        actor_id: Some("user-42".to_string()),
        query_text: Some("concise answer preference".to_string()),
        query_hash: format!("hash:{trace_id}"),
        retrievers_json: Some(r#"["native_sql"]"#.to_string()),
        latency_ms: Some(17),
        degraded: false,
        metadata_json: Some(r#"{"profile":"native_sql"}"#.to_string()),
        hits: vec![
            LlmRetrievalHitDraft {
                hit_id: format!("{trace_id}-hit-1"),
                record_id: Some("rec-trace-1".to_string()),
                retriever_name: "native_sql".to_string(),
                result_rank: 1,
                raw_score: Some(0.75),
                fused_score: Some(0.9),
                explanation_json: Some(r#"{"match":"keyword"}"#.to_string()),
                status: "selected".to_string(),
            },
            LlmRetrievalHitDraft {
                hit_id: format!("{trace_id}-hit-2"),
                record_id: None,
                retriever_name: "native_sql".to_string(),
                result_rank: 2,
                raw_score: Some(0.5),
                fused_score: Some(0.6),
                explanation_json: None,
                status: "candidate".to_string(),
            },
        ],
        context_pack: Some(LlmContextPackSnapshot {
            context_pack_id: format!("{trace_id}-pack"),
            pack_json: r#"{"recordIds":["rec-trace-1"]}"#.to_string(),
            estimated_tokens: 12,
            truncated: false,
        }),
    }
}

#[tokio::test]
async fn sqlite_store_applies_phase1_migration_and_round_trips_event_and_record() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_event(&scope, "evt-1", "User prefers concise answers")
        .await
        .unwrap();
    store
        .create_record(&scope, "rec-1", "answer_style", "concise")
        .await
        .unwrap();

    let event = store
        .retrieve_event(&scope, "evt-1")
        .await
        .unwrap()
        .unwrap();
    let record = store
        .retrieve_record(&scope, "rec-1")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(event.event_id, "evt-1");
    assert_eq!(event.content, "User prefers concise answers");
    assert_eq!(record.record_id, "rec-1");
    assert_eq!(record.content, "concise");
}

#[tokio::test]
async fn sqlite_store_preserves_event_content_with_json_sensitive_characters() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    let content = r#"User said "use C:\sdkwork\memory" for local tests"#;
    store
        .append_event(&scope, "evt-json", content)
        .await
        .unwrap();

    let event = store
        .retrieve_event(&scope, "evt-json")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(event.content, content);
}

#[tokio::test]
async fn sqlite_store_reads_event_payload_as_structured_json() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    let content = "line one\nline two";
    store
        .append_event(&scope, "evt-payload", content)
        .await
        .unwrap();

    let payload = store
        .retrieve_event_payload(&scope, "evt-payload")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(payload["content"].as_str(), Some(content));
}

#[tokio::test]
async fn sqlite_store_implements_record_and_event_store_spi_ports() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    let event = LlmEventStorePort::append(
        &store,
        AppendLlmEventCommand {
            scope: scope.clone(),
            event_id: "evt-spi".to_string(),
            content: "SPI event payload".to_string(),
        },
    )
    .await
    .unwrap();
    let record = LlmRecordStorePort::create(
        &store,
        CreateLlmRecordCommand {
            scope: scope.clone(),
            record_id: "rec-spi".to_string(),
            content: "SPI record payload".to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(event.event_id, "evt-spi");
    assert_eq!(event.content, "SPI event payload");
    assert_eq!(record.record_id, "rec-spi");
    assert_eq!(record.content, "SPI record payload");
}

#[tokio::test]
async fn sqlite_store_keeps_records_and_events_isolated_by_tenant_and_space() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let wrong_space = LlmScopeContext::for_test(100_001, 2);

    store
        .append_event(&tenant_one, "evt-shared", "tenant one event")
        .await
        .unwrap();
    store
        .append_event(&tenant_two, "evt-shared", "tenant two event")
        .await
        .unwrap();
    store
        .create_record(&tenant_one, "rec-shared", "preference", "tenant one record")
        .await
        .unwrap();
    store
        .create_record(&tenant_two, "rec-shared", "preference", "tenant two record")
        .await
        .unwrap();

    let tenant_one_event = store
        .retrieve_event(&tenant_one, "evt-shared")
        .await
        .unwrap()
        .unwrap();
    let tenant_two_event = store
        .retrieve_event(&tenant_two, "evt-shared")
        .await
        .unwrap()
        .unwrap();
    let tenant_one_record = store
        .retrieve_record(&tenant_one, "rec-shared")
        .await
        .unwrap()
        .unwrap();
    let tenant_two_record = store
        .retrieve_record(&tenant_two, "rec-shared")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(tenant_one_event.content, "tenant one event");
    assert_eq!(tenant_two_event.content, "tenant two event");
    assert_eq!(tenant_one_record.content, "tenant one record");
    assert_eq!(tenant_two_record.content, "tenant two record");
    assert!(store
        .retrieve_event(&wrong_space, "evt-shared")
        .await
        .unwrap()
        .is_none());
    assert!(store
        .retrieve_record(&wrong_space, "rec-shared")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn sqlite_store_spi_retrieve_methods_require_matching_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);

    LlmEventStorePort::append(
        &store,
        AppendLlmEventCommand {
            scope: tenant_one.clone(),
            event_id: "evt-spi-scoped".to_string(),
            content: "tenant one event".to_string(),
        },
    )
    .await
    .unwrap();
    LlmRecordStorePort::create(
        &store,
        CreateLlmRecordCommand {
            scope: tenant_one.clone(),
            record_id: "rec-spi-scoped".to_string(),
            content: "tenant one record".to_string(),
        },
    )
    .await
    .unwrap();

    assert!(LlmEventStorePort::retrieve(
        &store,
        RetrieveLlmEventQuery {
            scope: tenant_two.clone(),
            event_id: "evt-spi-scoped".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
    assert!(LlmRecordStorePort::retrieve(
        &store,
        RetrieveLlmRecordQuery {
            scope: tenant_two,
            record_id: "rec-spi-scoped".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
}

#[tokio::test]
async fn sqlite_store_soft_deletes_records_and_suppresses_retrieve() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .create_record(&scope, "rec-delete", "preference", "delete me")
        .await
        .unwrap();

    let receipt = store
        .mark_record_deleted(&scope, "rec-delete")
        .await
        .unwrap();
    let retrieved = store.retrieve_record(&scope, "rec-delete").await.unwrap();
    let lifecycle = store
        .retrieve_record_lifecycle(&scope, "rec-delete")
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.deleted);
    assert!(!receipt.already_deleted);
    assert!(retrieved.is_none());
    assert_eq!(lifecycle.record_id, "rec-delete");
    assert_eq!(lifecycle.status, "deleted");
    assert_utc_timestamp(lifecycle.deleted_at.as_deref());
}

#[tokio::test]
async fn sqlite_store_record_delete_is_idempotent_for_already_deleted_records() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .create_record(&scope, "rec-delete-repeat", "preference", "delete me")
        .await
        .unwrap();

    let first = store
        .mark_record_deleted(&scope, "rec-delete-repeat")
        .await
        .unwrap();
    let second = store
        .mark_record_deleted(&scope, "rec-delete-repeat")
        .await
        .unwrap();

    assert!(first.deleted);
    assert!(!first.already_deleted);
    assert!(second.deleted);
    assert!(second.already_deleted);
}

#[tokio::test]
async fn sqlite_store_record_delete_does_not_cross_tenant_or_space_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let wrong_space = LlmScopeContext::for_test(100_001, 2);

    store
        .create_record(&tenant_one, "rec-delete-scoped", "preference", "tenant one")
        .await
        .unwrap();
    store
        .create_record(&tenant_two, "rec-delete-scoped", "preference", "tenant two")
        .await
        .unwrap();

    let missing = store
        .mark_record_deleted(&wrong_space, "rec-delete-scoped")
        .await
        .unwrap();
    let deleted = store
        .mark_record_deleted(&tenant_one, "rec-delete-scoped")
        .await
        .unwrap();
    let tenant_two_record = store
        .retrieve_record(&tenant_two, "rec-delete-scoped")
        .await
        .unwrap()
        .unwrap();

    assert!(!missing.deleted);
    assert!(deleted.deleted);
    assert!(store
        .retrieve_record(&tenant_one, "rec-delete-scoped")
        .await
        .unwrap()
        .is_none());
    assert_eq!(tenant_two_record.content, "tenant two");
}

#[tokio::test]
async fn sqlite_store_implements_record_delete_spi_port() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    LlmRecordStorePort::create(
        &store,
        CreateLlmRecordCommand {
            scope: scope.clone(),
            record_id: "rec-spi-delete".to_string(),
            content: "SPI delete payload".to_string(),
        },
    )
    .await
    .unwrap();

    let receipt = LlmRecordStorePort::mark_deleted(
        &store,
        DeleteLlmRecordCommand {
            scope: scope.clone(),
            record_id: "rec-spi-delete".to_string(),
        },
    )
    .await
    .unwrap();
    let retrieved = LlmRecordStorePort::retrieve(
        &store,
        RetrieveLlmRecordQuery {
            scope,
            record_id: "rec-spi-delete".to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(receipt.record_id, "rec-spi-delete");
    assert!(receipt.deleted);
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn sqlite_store_event_append_is_idempotent_for_same_scope_event_and_content() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_event(&scope, "evt-idempotent", "same content")
        .await
        .unwrap();
    store
        .append_event(&scope, "evt-idempotent", "same content")
        .await
        .unwrap();

    let event = store
        .retrieve_event(&scope, "evt-idempotent")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(event.content, "same content");
}

#[tokio::test]
async fn sqlite_store_event_append_rejects_same_scope_event_with_different_content() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_event(&scope, "evt-conflict", "alpha")
        .await
        .unwrap();
    let err = store
        .append_event(&scope, "evt-conflict", "omega")
        .await
        .unwrap_err();

    assert!(matches!(err, NativeSqlStoreError::EventConflict { .. }));
}

#[tokio::test]
async fn sqlite_store_event_append_rejects_same_tenant_event_reuse_in_different_space() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let first_space = LlmScopeContext::for_test(100_001, 1);
    let second_space = LlmScopeContext::for_test(100_001, 2);

    store
        .append_event(&first_space, "evt-space-conflict", "same content")
        .await
        .unwrap();
    let err = store
        .append_event(&second_space, "evt-space-conflict", "same content")
        .await
        .unwrap_err();

    assert!(matches!(err, NativeSqlStoreError::EventConflict { .. }));
    assert!(store
        .retrieve_event(&second_space, "evt-space-conflict")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn sqlite_store_spi_event_append_maps_idempotency_conflict_to_spi_conflict() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    LlmEventStorePort::append(
        &store,
        AppendLlmEventCommand {
            scope: scope.clone(),
            event_id: "evt-spi-conflict".to_string(),
            content: "alpha".to_string(),
        },
    )
    .await
    .unwrap();
    let err = LlmEventStorePort::append(
        &store,
        AppendLlmEventCommand {
            scope,
            event_id: "evt-spi-conflict".to_string(),
            content: "omega".to_string(),
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(err, LlmSpiError::IdempotencyConflict { .. }));
}

#[tokio::test]
async fn sqlite_store_appends_and_retrieves_audit_records_by_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);

    store
        .append_audit(
            &tenant_one,
            "aud-shared",
            "llm.record.created",
            "llm_record",
            "rec-1",
            "success",
        )
        .await
        .unwrap();
    store
        .append_audit(
            &tenant_two,
            "aud-shared",
            "llm.record.created",
            "llm_record",
            "rec-2",
            "success",
        )
        .await
        .unwrap();

    let tenant_one_audit = store
        .retrieve_audit(&tenant_one, "aud-shared")
        .await
        .unwrap()
        .unwrap();
    let tenant_two_audit = store
        .retrieve_audit(&tenant_two, "aud-shared")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(tenant_one_audit.action, "llm.record.created");
    assert_eq!(tenant_one_audit.resource_id, "rec-1");
    assert_eq!(tenant_two_audit.resource_id, "rec-2");
    assert!(store
        .retrieve_audit(&LlmScopeContext::for_test(100_003, 3), "aud-shared")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn sqlite_store_implements_audit_store_spi_port() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    let audit = LlmAuditStorePort::append(
        &store,
        AppendLlmAuditCommand {
            scope: scope.clone(),
            audit_id: "aud-spi".to_string(),
            action: "llm.event.appended".to_string(),
            resource_type: "llm_event".to_string(),
            resource_id: "evt-spi".to_string(),
            result: "success".to_string(),
        },
    )
    .await
    .unwrap();
    let retrieved = LlmAuditStorePort::retrieve(
        &store,
        RetrieveLlmAuditQuery {
            scope,
            audit_id: "aud-spi".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(audit.audit_id, "aud-spi");
    assert_eq!(retrieved.action, "llm.event.appended");
    assert_eq!(retrieved.resource_type, "llm_event");
    assert_eq!(retrieved.resource_id, "evt-spi");
    assert_eq!(retrieved.result, "success");
}

#[tokio::test]
async fn sqlite_store_appends_and_retrieves_outbox_events_by_tenant_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);

    store
        .append_outbox_event(outbox_command(
            &tenant_one,
            "out-shared",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();
    store
        .append_outbox_event(outbox_command(
            &tenant_two,
            "out-shared",
            "rec-2",
            r#"{"recordId":"rec-2"}"#,
        ))
        .await
        .unwrap();

    let tenant_one_outbox = store
        .retrieve_outbox_event(&tenant_one, "out-shared")
        .await
        .unwrap()
        .unwrap();
    let tenant_two_outbox = store
        .retrieve_outbox_event(&tenant_two, "out-shared")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(tenant_one_outbox.aggregate_id, "rec-1");
    assert_eq!(tenant_one_outbox.publish_state, "pending");
    assert_eq!(tenant_one_outbox.retry_count, 0);
    assert_eq!(tenant_two_outbox.aggregate_id, "rec-2");
    assert!(store
        .retrieve_outbox_event(&LlmScopeContext::for_test(100_003, 3), "out-shared")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn sqlite_store_outbox_append_is_idempotent_for_same_tenant_event_and_payload() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_outbox_event(outbox_command(
            &scope,
            "out-idempotent",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();
    store
        .append_outbox_event(outbox_command(
            &scope,
            "out-idempotent",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();

    let outbox = store
        .retrieve_outbox_event(&scope, "out-idempotent")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(outbox.aggregate_id, "rec-1");
}

#[tokio::test]
async fn sqlite_store_outbox_append_rejects_same_tenant_event_with_different_payload() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_outbox_event(outbox_command(
            &scope,
            "out-conflict",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();
    let err = store
        .append_outbox_event(outbox_command(
            &scope,
            "out-conflict",
            "rec-1",
            r#"{"recordId":"rec-other"}"#,
        ))
        .await
        .unwrap_err();

    assert!(matches!(err, NativeSqlStoreError::OutboxConflict { .. }));
}

#[tokio::test]
async fn sqlite_store_implements_outbox_store_spi_port() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    let outbox = LlmOutboxStorePort::append(
        &store,
        AppendLlmOutboxCommand {
            scope: scope.clone(),
            outbox_id: "out-spi".to_string(),
            aggregate_type: "llm_event".to_string(),
            aggregate_id: "evt-spi".to_string(),
            event_type: "llm.event.appended".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"eventId":"evt-spi"}"#.to_string(),
        },
    )
    .await
    .unwrap();
    let retrieved = LlmOutboxStorePort::retrieve(
        &store,
        RetrieveLlmOutboxQuery {
            scope,
            outbox_id: "out-spi".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(outbox.outbox_id, "out-spi");
    assert_eq!(retrieved.aggregate_type, "llm_event");
    assert_eq!(retrieved.aggregate_id, "evt-spi");
    assert_eq!(retrieved.event_type, "llm.event.appended");
    assert_eq!(retrieved.event_version, "1");
    assert_eq!(retrieved.payload_json, r#"{"eventId":"evt-spi"}"#);
    assert_eq!(retrieved.publish_state, "pending");
}

#[tokio::test]
async fn sqlite_store_spi_outbox_append_maps_idempotency_conflict_to_spi_conflict() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    LlmOutboxStorePort::append(
        &store,
        AppendLlmOutboxCommand {
            scope: scope.clone(),
            outbox_id: "out-spi-conflict".to_string(),
            aggregate_type: "llm_record".to_string(),
            aggregate_id: "rec-1".to_string(),
            event_type: "llm.record.created".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"recordId":"rec-1"}"#.to_string(),
        },
    )
    .await
    .unwrap();
    let err = LlmOutboxStorePort::append(
        &store,
        AppendLlmOutboxCommand {
            scope,
            outbox_id: "out-spi-conflict".to_string(),
            aggregate_type: "llm_record".to_string(),
            aggregate_id: "rec-1".to_string(),
            event_type: "llm.record.created".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"recordId":"rec-other"}"#.to_string(),
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(err, LlmSpiError::IdempotencyConflict { .. }));
}

#[tokio::test]
async fn sqlite_store_lists_pending_outbox_events_by_tenant_scope_and_limit() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);

    store
        .append_outbox_event(outbox_command(
            &tenant_one,
            "out-pending-1",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();
    store
        .append_outbox_event(outbox_command(
            &tenant_one,
            "out-pending-2",
            "rec-2",
            r#"{"recordId":"rec-2"}"#,
        ))
        .await
        .unwrap();
    store
        .append_outbox_event(outbox_command(
            &tenant_two,
            "out-pending-tenant-two",
            "rec-3",
            r#"{"recordId":"rec-3"}"#,
        ))
        .await
        .unwrap();

    let pending = store
        .list_pending_outbox_events(&tenant_one, 1)
        .await
        .unwrap();

    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].outbox_id, "out-pending-1");
    assert_eq!(pending[0].publish_state, "pending");
}

#[tokio::test]
async fn sqlite_store_marks_outbox_published_and_excludes_it_from_pending() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_outbox_event(outbox_command(
            &scope,
            "out-publish",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();

    let published = store
        .mark_outbox_published(&scope, "out-publish")
        .await
        .unwrap()
        .unwrap();
    let retrieved = store
        .retrieve_outbox_event(&scope, "out-publish")
        .await
        .unwrap()
        .unwrap();
    let pending = store.list_pending_outbox_events(&scope, 10).await.unwrap();

    assert_eq!(published.publish_state, "published");
    assert_utc_timestamp(published.published_at.as_deref());
    assert_eq!(retrieved.publish_state, "published");
    assert!(pending.is_empty());
}

#[tokio::test]
async fn sqlite_store_marks_outbox_failed_increments_retry_and_excludes_it_from_pending() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    store
        .append_outbox_event(outbox_command(
            &scope,
            "out-fail",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();

    let failed = store
        .mark_outbox_failed(&scope, "out-fail")
        .await
        .unwrap()
        .unwrap();
    let pending = store.list_pending_outbox_events(&scope, 10).await.unwrap();

    assert_eq!(failed.publish_state, "failed");
    assert_eq!(failed.retry_count, 1);
    assert!(failed.published_at.is_none());
    assert!(pending.is_empty());
}

#[tokio::test]
async fn sqlite_store_outbox_delivery_lifecycle_does_not_cross_tenant_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let missing_tenant = LlmScopeContext::for_test(100_003, 3);

    store
        .append_outbox_event(outbox_command(
            &tenant_one,
            "out-scoped",
            "rec-1",
            r#"{"recordId":"rec-1"}"#,
        ))
        .await
        .unwrap();
    store
        .append_outbox_event(outbox_command(
            &tenant_two,
            "out-scoped",
            "rec-2",
            r#"{"recordId":"rec-2"}"#,
        ))
        .await
        .unwrap();

    let missing = store
        .mark_outbox_published(&missing_tenant, "out-scoped")
        .await
        .unwrap();
    let tenant_one_published = store
        .mark_outbox_published(&tenant_one, "out-scoped")
        .await
        .unwrap()
        .unwrap();
    let tenant_two_pending = store
        .list_pending_outbox_events(&tenant_two, 10)
        .await
        .unwrap();

    assert!(missing.is_none());
    assert_eq!(tenant_one_published.publish_state, "published");
    assert_eq!(tenant_two_pending.len(), 1);
    assert_eq!(tenant_two_pending[0].aggregate_id, "rec-2");
}

#[tokio::test]
async fn sqlite_store_implements_outbox_delivery_lifecycle_spi_port() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let scope = LlmScopeContext::for_test(100_001, 1);

    LlmOutboxStorePort::append(
        &store,
        AppendLlmOutboxCommand {
            scope: scope.clone(),
            outbox_id: "out-spi-lifecycle".to_string(),
            aggregate_type: "llm_record".to_string(),
            aggregate_id: "rec-1".to_string(),
            event_type: "llm.record.created".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"recordId":"rec-1"}"#.to_string(),
        },
    )
    .await
    .unwrap();

    let pending = LlmOutboxStorePort::list_pending(
        &store,
        ListPendingLlmOutboxQuery {
            scope: scope.clone(),
            limit: 10,
        },
    )
    .await
    .unwrap();
    let published = LlmOutboxStorePort::mark_published(
        &store,
        MarkLlmOutboxPublishedCommand {
            scope: scope.clone(),
            outbox_id: "out-spi-lifecycle".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let pending_after_publish = LlmOutboxStorePort::list_pending(
        &store,
        ListPendingLlmOutboxQuery {
            scope: scope.clone(),
            limit: 10,
        },
    )
    .await
    .unwrap();

    LlmOutboxStorePort::append(
        &store,
        AppendLlmOutboxCommand {
            scope: scope.clone(),
            outbox_id: "out-spi-failed".to_string(),
            aggregate_type: "llm_event".to_string(),
            aggregate_id: "evt-1".to_string(),
            event_type: "llm.event.appended".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"eventId":"evt-1"}"#.to_string(),
        },
    )
    .await
    .unwrap();
    let failed = LlmOutboxStorePort::mark_failed(
        &store,
        MarkLlmOutboxFailedCommand {
            scope,
            outbox_id: "out-spi-failed".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].outbox_id, "out-spi-lifecycle");
    assert_eq!(published.publish_state, "published");
    assert_utc_timestamp(published.published_at.as_deref());
    assert!(pending_after_publish.is_empty());
    assert_eq!(failed.publish_state, "failed");
    assert_eq!(failed.retry_count, 1);
}

#[tokio::test]
async fn sqlite_store_creates_and_decides_candidates_by_tenant_and_space_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let wrong_space = LlmScopeContext::for_test(100_001, 2);

    let tenant_one_candidate = LlmCandidateStorePort::create(
        &store,
        candidate_command(tenant_one.clone(), "cand-shared"),
    )
    .await
    .unwrap();
    let tenant_two_candidate = LlmCandidateStorePort::create(
        &store,
        candidate_command(tenant_two.clone(), "cand-shared"),
    )
    .await
    .unwrap();

    let approved = LlmCandidateStorePort::approve(
        &store,
        ApproveLlmCandidateCommand {
            scope: tenant_one.clone(),
            candidate_id: "cand-shared".to_string(),
            decision_reason: Some("confirmed by user".to_string()),
            decided_by: Some(7),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let rejected = LlmCandidateStorePort::reject(
        &store,
        RejectLlmCandidateCommand {
            scope: tenant_two.clone(),
            candidate_id: "cand-shared".to_string(),
            decision_reason: Some("stale signal".to_string()),
            decided_by: Some(8),
        },
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(tenant_one_candidate.decision_state, "pending");
    assert_eq!(tenant_two_candidate.decision_state, "pending");
    assert_eq!(approved.decision_state, "approved");
    assert_eq!(
        approved.decision_reason.as_deref(),
        Some("confirmed by user")
    );
    assert_eq!(approved.decided_by, Some(7));
    assert_utc_timestamp(approved.decided_at.as_deref());
    assert_eq!(rejected.decision_state, "rejected");
    assert_eq!(rejected.decision_reason.as_deref(), Some("stale signal"));
    assert!(LlmCandidateStorePort::retrieve(
        &store,
        RetrieveLlmCandidateQuery {
            scope: wrong_space,
            candidate_id: "cand-shared".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
}

#[tokio::test]
async fn sqlite_store_upserts_promotes_and_decays_habits_by_tenant_space_and_user_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let wrong_user = 43;

    store
        .create_record(&tenant_one, "rec-promoted", "answer_style", "concise")
        .await
        .unwrap();
    let inserted =
        LlmHabitStorePort::upsert(&store, habit_command(tenant_one.clone(), "habit-1", 1))
            .await
            .unwrap();
    let updated = LlmHabitStorePort::upsert(
        &store,
        UpsertLlmHabitCommand {
            strength: 0.7,
            support_count: 4,
            ..habit_command(tenant_one.clone(), "habit-1", 1)
        },
    )
    .await
    .unwrap();
    let tenant_two_habit =
        LlmHabitStorePort::upsert(&store, habit_command(tenant_two.clone(), "habit-2", 1))
            .await
            .unwrap();
    let promoted = LlmHabitStorePort::promote(
        &store,
        PromoteLlmHabitCommand {
            scope: tenant_one.clone(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
            promoted_record_id: Some("rec-promoted".to_string()),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let decayed = LlmHabitStorePort::decay(
        &store,
        DecayLlmHabitCommand {
            scope: tenant_one.clone(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
            strength_delta: 0.2,
        },
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(inserted.strength, 0.4);
    assert_eq!(updated.strength, 0.7);
    assert_eq!(updated.support_count, 4);
    assert_eq!(tenant_two_habit.habit_id, "habit-2");
    assert_eq!(promoted.stage, "promoted");
    assert_eq!(promoted.promoted_record_id.as_deref(), Some("rec-promoted"));
    assert_eq!(decayed.stage, "decayed");
    assert!((decayed.strength - 0.5).abs() < f64::EPSILON);
    assert!(LlmHabitStorePort::retrieve(
        &store,
        RetrieveLlmHabitQuery {
            scope: tenant_one,
            user_id: wrong_user,
            habit_key: "answer_style:concise".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
}

#[tokio::test]
async fn sqlite_store_appends_retrieval_trace_with_hits_and_context_pack_by_scope() {
    let store = NativeSqlLlmStore::new_in_memory_sqlite().await.unwrap();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let wrong_space = LlmScopeContext::for_test(100_001, 2);

    store
        .create_record(&tenant_one, "rec-trace-1", "answer_style", "concise")
        .await
        .unwrap();
    let appended = LlmRetrievalTraceStorePort::append(
        &store,
        retrieval_trace_command(tenant_one.clone(), "trace-shared"),
    )
    .await
    .unwrap();
    LlmRetrievalTraceStorePort::append(
        &store,
        AppendLlmRetrievalTraceCommand {
            query_text: Some("tenant two query".to_string()),
            ..retrieval_trace_command(tenant_two.clone(), "trace-shared")
        },
    )
    .await
    .unwrap();

    let retrieved = LlmRetrievalTraceStorePort::retrieve(
        &store,
        RetrieveLlmRetrievalTraceQuery {
            scope: tenant_one.clone(),
            trace_id: "trace-shared".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let tenant_two_trace = LlmRetrievalTraceStorePort::retrieve(
        &store,
        RetrieveLlmRetrievalTraceQuery {
            scope: tenant_two,
            trace_id: "trace-shared".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let recent = LlmRetrievalTraceStorePort::list_recent(
        &store,
        ListLlmRetrievalTracesQuery {
            scope: tenant_one.clone(),
            limit: 1,
        },
    )
    .await
    .unwrap();

    assert_eq!(appended.trace_id, "trace-shared");
    assert_eq!(retrieved.query_hash, "hash:trace-shared");
    assert_eq!(retrieved.result_count, 2);
    assert_eq!(retrieved.hits.len(), 2);
    assert_eq!(retrieved.hits[0].hit_id, "trace-shared-hit-1");
    assert_eq!(retrieved.hits[0].record_id.as_deref(), Some("rec-trace-1"));
    assert_eq!(retrieved.hits[1].record_id, None);
    assert_eq!(
        retrieved
            .context_pack
            .as_ref()
            .map(|pack| pack.context_pack_id.as_str()),
        Some("trace-shared-pack")
    );
    assert_eq!(
        tenant_two_trace.query_text.as_deref(),
        Some("tenant two query")
    );
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].trace_id, "trace-shared");
    assert!(LlmRetrievalTraceStorePort::retrieve(
        &store,
        RetrieveLlmRetrievalTraceQuery {
            scope: wrong_space,
            trace_id: "trace-shared".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
}

#[test]
fn native_sql_manifest_exports_candidate_habit_and_retrieval_trace_builders() {
    let candidate = build_native_sql_candidate_store();
    let habit = build_native_sql_habit_store();
    let retrieval_trace = build_native_sql_retrieval_trace_store();

    assert_eq!(candidate.port_name, "LlmCandidateStorePort");
    assert_eq!(candidate.builder_name, "build_native_sql_candidate_store");
    assert!(candidate.ready);
    assert_eq!(habit.port_name, "LlmHabitStorePort");
    assert_eq!(habit.builder_name, "build_native_sql_habit_store");
    assert!(habit.ready);
    assert_eq!(retrieval_trace.port_name, "LlmRetrievalTraceStorePort");
    assert_eq!(
        retrieval_trace.builder_name,
        "build_native_sql_retrieval_trace_store"
    );
    assert!(retrieval_trace.ready);
}
