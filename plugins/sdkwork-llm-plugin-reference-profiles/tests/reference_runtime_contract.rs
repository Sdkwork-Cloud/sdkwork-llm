use sdkwork_llm_plugin_reference_profiles::ReferenceLlmRuntime;
use sdkwork_llm_spi::{
    AppendLlmAuditCommand, AppendLlmEventCommand, AppendLlmOutboxCommand,
    AppendLlmRetrievalTraceCommand, ApproveLlmCandidateCommand, AssembleLlmContextCommand,
    CreateLlmCandidateCommand, CreateLlmRecordCommand, DecayLlmHabitCommand,
    ExternalLlmBridgePort, ExternalLlmImportCommand, ListLlmRetrievalTracesQuery,
    ListPendingLlmOutboxQuery, MarkLlmOutboxPublishedCommand, LlmAuditStorePort,
    LlmCandidateStorePort, LlmContextAssemblerPort, LlmContextPackSnapshot,
    LlmEvaluationPort, LlmEventStorePort, LlmHabitStorePort, LlmIndexPort,
    LlmOutboxStorePort, LlmRecordStorePort, LlmRetrievalHitDraft,
    LlmRetrievalTraceStorePort, LlmRetrieverPort, LlmScopeContext,
    PromoteLlmHabitCommand, RejectLlmCandidateCommand, RetrieveLlmAuditQuery,
    RetrieveLlmCandidateQuery, RetrieveLlmCandidatesCommand, RetrieveLlmEventQuery,
    RetrieveLlmHabitQuery, RetrieveLlmRecordQuery, RetrieveLlmRetrievalTraceQuery,
    RunLlmEvalCommand, UpsertLlmHabitCommand,
};

#[tokio::test]
async fn reference_runtime_round_trips_core_ports_and_retrieves_by_keyword() {
    let runtime = ReferenceLlmRuntime::new();
    let scope = LlmScopeContext::for_test(100_001, 1);

    LlmRecordStorePort::create(
        &runtime,
        CreateLlmRecordCommand {
            scope: scope.clone(),
            record_id: "rec-reference".to_string(),
            content: "reference memory supports keyword lookup".to_string(),
        },
    )
    .await
    .unwrap();
    LlmEventStorePort::append(
        &runtime,
        AppendLlmEventCommand {
            scope: scope.clone(),
            event_id: "evt-reference".to_string(),
            content: "event sourced baseline".to_string(),
        },
    )
    .await
    .unwrap();
    LlmAuditStorePort::append(
        &runtime,
        AppendLlmAuditCommand {
            scope: scope.clone(),
            audit_id: "aud-reference".to_string(),
            action: "memory.reference.checked".to_string(),
            resource_type: "llm_record".to_string(),
            resource_id: "rec-reference".to_string(),
            result: "success".to_string(),
        },
    )
    .await
    .unwrap();
    LlmOutboxStorePort::append(
        &runtime,
        AppendLlmOutboxCommand {
            scope: scope.clone(),
            outbox_id: "out-reference".to_string(),
            aggregate_type: "llm_record".to_string(),
            aggregate_id: "rec-reference".to_string(),
            event_type: "llm.record.created".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"recordId":"rec-reference"}"#.to_string(),
        },
    )
    .await
    .unwrap();

    let record = LlmRecordStorePort::retrieve(
        &runtime,
        RetrieveLlmRecordQuery {
            scope: scope.clone(),
            record_id: "rec-reference".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let event = LlmEventStorePort::retrieve(
        &runtime,
        RetrieveLlmEventQuery {
            scope: scope.clone(),
            event_id: "evt-reference".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let audit = LlmAuditStorePort::retrieve(
        &runtime,
        RetrieveLlmAuditQuery {
            scope: scope.clone(),
            audit_id: "aud-reference".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let hits = LlmRetrieverPort::retrieve(
        &runtime,
        RetrieveLlmCandidatesCommand {
            query: "keyword".to_string(),
        },
    )
    .await
    .unwrap();
    let receipt = LlmIndexPort::index(&runtime, "rec-reference".to_string())
        .await
        .unwrap();

    assert_eq!(record.content, "reference memory supports keyword lookup");
    assert_eq!(event.content, "event sourced baseline");
    assert_eq!(audit.result, "success");
    assert_eq!(hits.record_ids, vec!["rec-reference".to_string()]);
    assert_eq!(receipt.record_id, "rec-reference");
}

#[tokio::test]
async fn reference_runtime_outbox_context_eval_and_bridge_fail_closed_are_deterministic() {
    let runtime = ReferenceLlmRuntime::new();
    let scope = LlmScopeContext::for_test(100_001, 1);

    LlmRecordStorePort::create(
        &runtime,
        CreateLlmRecordCommand {
            scope: scope.clone(),
            record_id: "rec-context".to_string(),
            content: "context line".to_string(),
        },
    )
    .await
    .unwrap();
    LlmOutboxStorePort::append(
        &runtime,
        AppendLlmOutboxCommand {
            scope: scope.clone(),
            outbox_id: "out-context".to_string(),
            aggregate_type: "llm_record".to_string(),
            aggregate_id: "rec-context".to_string(),
            event_type: "llm.record.created".to_string(),
            event_version: "1".to_string(),
            payload_json: r#"{"recordId":"rec-context"}"#.to_string(),
        },
    )
    .await
    .unwrap();

    let pending = LlmOutboxStorePort::list_pending(
        &runtime,
        ListPendingLlmOutboxQuery {
            scope: scope.clone(),
            limit: 10,
        },
    )
    .await
    .unwrap();
    let published = LlmOutboxStorePort::mark_published(
        &runtime,
        MarkLlmOutboxPublishedCommand {
            scope: scope.clone(),
            outbox_id: "out-context".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let context = LlmContextAssemblerPort::assemble(
        &runtime,
        AssembleLlmContextCommand {
            record_ids: vec!["rec-context".to_string()],
        },
    )
    .await
    .unwrap();
    let eval = LlmEvaluationPort::run(
        &runtime,
        RunLlmEvalCommand {
            eval_type: "baseline".to_string(),
        },
    )
    .await
    .unwrap();
    let bridge_error = ExternalLlmBridgePort::import(&runtime, ExternalLlmImportCommand)
        .await
        .unwrap_err();

    assert_eq!(pending.len(), 1);
    assert_eq!(published.publish_state, "published");
    assert!(
        published
            .published_at
            .as_deref()
            .is_some_and(|value| value.ends_with('Z'))
    );
    assert_eq!(context.context_text, "context line");
    assert_eq!(eval.eval_type, "baseline");
    assert!(bridge_error
        .to_string()
        .contains("fail-closed until a reviewed provider adapter is configured"));
}

#[tokio::test]
async fn reference_runtime_round_trips_learning_and_trace_ports_by_scope() {
    let runtime = ReferenceLlmRuntime::new();
    let tenant_one = LlmScopeContext::for_test(100_001, 1);
    let tenant_two = LlmScopeContext::for_test(100_002, 2);
    let wrong_space = LlmScopeContext::for_test(100_001, 2);

    LlmRecordStorePort::create(
        &runtime,
        CreateLlmRecordCommand {
            scope: tenant_one.clone(),
            record_id: "rec-trace".to_string(),
            content: "traceable reference memory".to_string(),
        },
    )
    .await
    .unwrap();

    let created_candidate = LlmCandidateStorePort::create(
        &runtime,
        CreateLlmCandidateCommand {
            scope: tenant_one.clone(),
            candidate_id: "cand-reference".to_string(),
            candidate_type: "observation".to_string(),
            record_type: "semantic".to_string(),
            proposed_text: "User prefers concise answers".to_string(),
            proposed_payload_json: Some(r#"{"preference":"concise"}"#.to_string()),
            evidence_json: Some(r#"{"source":"event"}"#.to_string()),
            confidence: 0.91,
        },
    )
    .await
    .unwrap();
    LlmCandidateStorePort::create(
        &runtime,
        CreateLlmCandidateCommand {
            scope: tenant_two.clone(),
            candidate_id: "cand-reference".to_string(),
            candidate_type: "observation".to_string(),
            record_type: "semantic".to_string(),
            proposed_text: "Tenant two candidate".to_string(),
            proposed_payload_json: None,
            evidence_json: None,
            confidence: 0.51,
        },
    )
    .await
    .unwrap();
    let approved = LlmCandidateStorePort::approve(
        &runtime,
        ApproveLlmCandidateCommand {
            scope: tenant_one.clone(),
            candidate_id: "cand-reference".to_string(),
            decision_reason: Some("confirmed".to_string()),
            decided_by: Some(7),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let rejected = LlmCandidateStorePort::reject(
        &runtime,
        RejectLlmCandidateCommand {
            scope: tenant_two.clone(),
            candidate_id: "cand-reference".to_string(),
            decision_reason: Some("stale".to_string()),
            decided_by: Some(8),
        },
    )
    .await
    .unwrap()
    .unwrap();

    let inserted_habit = LlmHabitStorePort::upsert(
        &runtime,
        UpsertLlmHabitCommand {
            scope: tenant_one.clone(),
            habit_id: "habit-reference".to_string(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
            habit_type: "preference".to_string(),
            description: "Prefers concise answers".to_string(),
            stage: "candidate".to_string(),
            strength: 0.4,
            confidence: 0.8,
            support_count: 2,
            metadata_json: Some(r#"{"source":"signals"}"#.to_string()),
        },
    )
    .await
    .unwrap();
    let promoted = LlmHabitStorePort::promote(
        &runtime,
        PromoteLlmHabitCommand {
            scope: tenant_one.clone(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
            promoted_record_id: Some("rec-trace".to_string()),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let decayed = LlmHabitStorePort::decay(
        &runtime,
        DecayLlmHabitCommand {
            scope: tenant_one.clone(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
            strength_delta: 0.1,
        },
    )
    .await
    .unwrap()
    .unwrap();

    let trace = LlmRetrievalTraceStorePort::append(
        &runtime,
        AppendLlmRetrievalTraceCommand {
            scope: tenant_one.clone(),
            trace_id: "trace-reference".to_string(),
            actor_id: Some("user-42".to_string()),
            query_text: Some("traceable".to_string()),
            query_hash: "hash:trace-reference".to_string(),
            retrievers_json: Some(r#"["reference_keyword"]"#.to_string()),
            latency_ms: Some(3),
            degraded: false,
            metadata_json: Some(r#"{"profile":"reference"}"#.to_string()),
            hits: vec![LlmRetrievalHitDraft {
                hit_id: "hit-reference".to_string(),
                record_id: Some("rec-trace".to_string()),
                retriever_name: "reference_keyword".to_string(),
                result_rank: 1,
                raw_score: Some(0.9),
                fused_score: Some(0.95),
                explanation_json: Some(r#"{"match":"keyword"}"#.to_string()),
                status: "selected".to_string(),
            }],
            context_pack: Some(LlmContextPackSnapshot {
                context_pack_id: "pack-reference".to_string(),
                pack_json: r#"{"recordIds":["rec-trace"]}"#.to_string(),
                estimated_tokens: 9,
                truncated: false,
            }),
        },
    )
    .await
    .unwrap();
    let retrieved_trace = LlmRetrievalTraceStorePort::retrieve(
        &runtime,
        RetrieveLlmRetrievalTraceQuery {
            scope: tenant_one.clone(),
            trace_id: "trace-reference".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();
    let recent = LlmRetrievalTraceStorePort::list_recent(
        &runtime,
        ListLlmRetrievalTracesQuery {
            scope: tenant_one.clone(),
            limit: 1,
        },
    )
    .await
    .unwrap();

    assert_eq!(created_candidate.decision_state, "pending");
    assert_eq!(approved.decision_state, "approved");
    assert_eq!(approved.decided_by, Some(7));
    assert_eq!(rejected.decision_state, "rejected");
    assert_eq!(inserted_habit.strength, 0.4);
    assert_eq!(promoted.promoted_record_id.as_deref(), Some("rec-trace"));
    assert_eq!(decayed.stage, "decayed");
    assert!((decayed.strength - 0.3).abs() < f64::EPSILON);
    assert_eq!(trace.result_count, 1);
    assert_eq!(
        retrieved_trace.hits[0].record_id.as_deref(),
        Some("rec-trace")
    );
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].trace_id, "trace-reference");
    assert!(LlmCandidateStorePort::retrieve(
        &runtime,
        RetrieveLlmCandidateQuery {
            scope: wrong_space.clone(),
            candidate_id: "cand-reference".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
    assert!(LlmHabitStorePort::retrieve(
        &runtime,
        RetrieveLlmHabitQuery {
            scope: wrong_space.clone(),
            user_id: 1,
            habit_key: "answer_style:concise".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
    assert!(LlmRetrievalTraceStorePort::retrieve(
        &runtime,
        RetrieveLlmRetrievalTraceQuery {
            scope: wrong_space,
            trace_id: "trace-reference".to_string(),
        },
    )
    .await
    .unwrap()
    .is_none());
}
