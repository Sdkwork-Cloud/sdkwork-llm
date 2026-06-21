use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use sdkwork_llm_spi::{
    AppendLlmAuditCommand, AppendLlmEventCommand, AppendLlmOutboxCommand,
    AppendLlmRetrievalTraceCommand, ApproveLlmCandidateCommand, AssembleLlmContextCommand,
    CreateLlmCandidateCommand, CreateLlmRecordCommand, DecayLlmHabitCommand,
    DeleteLlmRecordCommand, ExternalLlmBridgePort, ExternalLlmDeleteCommand,
    ExternalLlmDeleteReceipt, ExternalLlmExportCommand, ExternalLlmExportResult,
    ExternalLlmImportCommand, ExternalLlmImportResult, ExternalLlmShadowReadCommand,
    ExternalLlmShadowReadResult, ListLlmRetrievalTracesQuery, ListPendingLlmOutboxQuery,
    MarkLlmOutboxFailedCommand, MarkLlmOutboxPublishedCommand, LlmAuditRecord,
    LlmAuditStorePort, LlmCandidate, LlmCandidateStorePort, LlmContextAssemblerPort,
    LlmContextPackDraft, LlmDeletionReceipt, LlmEvalRunResult, LlmEvaluationPort,
    LlmEvent, LlmEventStorePort, LlmHabit, LlmHabitStorePort, LlmIndexPort,
    LlmIndexReceipt, LlmOutboxEvent, LlmOutboxStorePort, LlmRecord,
    LlmRecordStorePort, LlmRetrievalTrace, LlmRetrievalTraceStorePort,
    LlmRetrieverPort, LlmRetrieverResult, LlmScopeContext, LlmSpiError,
    LlmSpiResult, PromoteLlmHabitCommand, RejectLlmCandidateCommand,
    RetrieveLlmAuditQuery, RetrieveLlmCandidateQuery, RetrieveLlmCandidatesCommand,
    RetrieveLlmEventQuery, RetrieveLlmHabitQuery, RetrieveLlmOutboxQuery,
    RetrieveLlmRecordQuery, RetrieveLlmRetrievalTraceQuery, RunLlmEvalCommand,
    UpsertLlmHabitCommand,
};

#[derive(Debug, Default)]
pub struct ReferenceLlmRuntime {
    records: Mutex<HashMap<ScopedId, LlmRecordState>>,
    events: Mutex<HashMap<ScopedId, LlmEvent>>,
    audits: Mutex<HashMap<ScopedId, LlmAuditRecord>>,
    outbox: Mutex<HashMap<ScopedId, LlmOutboxEvent>>,
    candidates: Mutex<HashMap<ScopedId, LlmCandidate>>,
    habits: Mutex<HashMap<ScopedHabitKey, LlmHabit>>,
    retrieval_traces: Mutex<HashMap<ScopedId, LlmRetrievalTrace>>,
}

impl ReferenceLlmRuntime {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl LlmRecordStorePort for ReferenceLlmRuntime {
    async fn create(&self, command: CreateLlmRecordCommand) -> LlmSpiResult<LlmRecord> {
        let record = LlmRecord {
            record_id: command.record_id.clone(),
            content: command.content,
        };
        let key = ScopedId::new(&command.scope, command.record_id);
        self.records
            .lock()
            .map_err(lock_error)?
            .insert(key, LlmRecordState::active(record.clone()));

        Ok(record)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmRecordQuery,
    ) -> LlmSpiResult<Option<LlmRecord>> {
        let key = ScopedId::new(&query.scope, query.record_id);
        let records = self.records.lock().map_err(lock_error)?;
        Ok(records
            .get(&key)
            .and_then(LlmRecordState::visible_record))
    }

    async fn mark_deleted(
        &self,
        command: DeleteLlmRecordCommand,
    ) -> LlmSpiResult<LlmDeletionReceipt> {
        let key = ScopedId::new(&command.scope, command.record_id.clone());
        let mut records = self.records.lock().map_err(lock_error)?;

        let Some(record) = records.get_mut(&key) else {
            return Ok(LlmDeletionReceipt {
                record_id: command.record_id,
                deleted: false,
                already_deleted: false,
            });
        };

        let already_deleted = record.deleted;
        record.deleted = true;

        Ok(LlmDeletionReceipt {
            record_id: command.record_id,
            deleted: true,
            already_deleted,
        })
    }
}

#[async_trait]
impl LlmEventStorePort for ReferenceLlmRuntime {
    async fn append(&self, command: AppendLlmEventCommand) -> LlmSpiResult<LlmEvent> {
        let event = LlmEvent {
            event_id: command.event_id.clone(),
            content: command.content,
        };
        let key = ScopedId::new(&command.scope, command.event_id);
        self.events
            .lock()
            .map_err(lock_error)?
            .insert(key, event.clone());

        Ok(event)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmEventQuery,
    ) -> LlmSpiResult<Option<LlmEvent>> {
        let key = ScopedId::new(&query.scope, query.event_id);
        Ok(self.events.lock().map_err(lock_error)?.get(&key).cloned())
    }
}

#[async_trait]
impl LlmAuditStorePort for ReferenceLlmRuntime {
    async fn append(
        &self,
        command: AppendLlmAuditCommand,
    ) -> LlmSpiResult<LlmAuditRecord> {
        let audit = LlmAuditRecord {
            audit_id: command.audit_id.clone(),
            action: command.action,
            resource_type: command.resource_type,
            resource_id: command.resource_id,
            result: command.result,
        };
        let key = ScopedId::new(&command.scope, command.audit_id);
        self.audits
            .lock()
            .map_err(lock_error)?
            .insert(key, audit.clone());

        Ok(audit)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmAuditQuery,
    ) -> LlmSpiResult<Option<LlmAuditRecord>> {
        let key = ScopedId::new(&query.scope, query.audit_id);
        Ok(self.audits.lock().map_err(lock_error)?.get(&key).cloned())
    }
}

#[async_trait]
impl LlmOutboxStorePort for ReferenceLlmRuntime {
    async fn append(
        &self,
        command: AppendLlmOutboxCommand,
    ) -> LlmSpiResult<LlmOutboxEvent> {
        let outbox = LlmOutboxEvent {
            outbox_id: command.outbox_id.clone(),
            aggregate_type: command.aggregate_type,
            aggregate_id: command.aggregate_id,
            event_type: command.event_type,
            event_version: command.event_version,
            payload_json: command.payload_json,
            publish_state: "pending".to_string(),
            published_at: None,
            retry_count: 0,
        };
        let key = ScopedId::new(&command.scope, command.outbox_id);
        self.outbox
            .lock()
            .map_err(lock_error)?
            .insert(key, outbox.clone());

        Ok(outbox)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmOutboxQuery,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>> {
        let key = ScopedId::new(&query.scope, query.outbox_id);
        Ok(self.outbox.lock().map_err(lock_error)?.get(&key).cloned())
    }

    async fn list_pending(
        &self,
        query: ListPendingLlmOutboxQuery,
    ) -> LlmSpiResult<Vec<LlmOutboxEvent>> {
        let outbox = self.outbox.lock().map_err(lock_error)?;
        let mut pending = outbox
            .iter()
            .filter(|(key, event)| {
                key.matches_scope(&query.scope) && event.publish_state == "pending"
            })
            .map(|(_, event)| event.clone())
            .collect::<Vec<_>>();
        pending.sort_by(|left, right| left.outbox_id.cmp(&right.outbox_id));
        pending.truncate(query.limit as usize);
        Ok(pending)
    }

    async fn mark_published(
        &self,
        command: MarkLlmOutboxPublishedCommand,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>> {
        self.update_outbox_state(
            &command.scope,
            command.outbox_id,
            "published",
            Some(now_text()),
        )
    }

    async fn mark_failed(
        &self,
        command: MarkLlmOutboxFailedCommand,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>> {
        let key = ScopedId::new(&command.scope, command.outbox_id);
        let mut outbox = self.outbox.lock().map_err(lock_error)?;
        let Some(event) = outbox.get_mut(&key) else {
            return Ok(None);
        };
        event.publish_state = "failed".to_string();
        event.retry_count += 1;
        Ok(Some(event.clone()))
    }
}

impl ReferenceLlmRuntime {
    fn update_outbox_state(
        &self,
        scope: &LlmScopeContext,
        outbox_id: String,
        publish_state: &str,
        published_at: Option<String>,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>> {
        let key = ScopedId::new(scope, outbox_id);
        let mut outbox = self.outbox.lock().map_err(lock_error)?;
        let Some(event) = outbox.get_mut(&key) else {
            return Ok(None);
        };
        event.publish_state = publish_state.to_string();
        event.published_at = published_at;
        Ok(Some(event.clone()))
    }
}

#[async_trait]
impl LlmCandidateStorePort for ReferenceLlmRuntime {
    async fn create(
        &self,
        command: CreateLlmCandidateCommand,
    ) -> LlmSpiResult<LlmCandidate> {
        let candidate = LlmCandidate {
            candidate_id: command.candidate_id.clone(),
            candidate_type: command.candidate_type,
            record_type: command.record_type,
            proposed_text: command.proposed_text,
            proposed_payload_json: command.proposed_payload_json,
            evidence_json: command.evidence_json,
            confidence: command.confidence,
            decision_state: "pending".to_string(),
            decision_reason: None,
            decided_by: None,
            decided_at: None,
        };
        let key = ScopedId::new(&command.scope, command.candidate_id);
        self.candidates
            .lock()
            .map_err(lock_error)?
            .insert(key, candidate.clone());

        Ok(candidate)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmCandidateQuery,
    ) -> LlmSpiResult<Option<LlmCandidate>> {
        let key = ScopedId::new(&query.scope, query.candidate_id);
        Ok(self
            .candidates
            .lock()
            .map_err(lock_error)?
            .get(&key)
            .cloned())
    }

    async fn approve(
        &self,
        command: ApproveLlmCandidateCommand,
    ) -> LlmSpiResult<Option<LlmCandidate>> {
        self.decide_candidate(
            &command.scope,
            command.candidate_id,
            "approved",
            command.decision_reason,
            command.decided_by,
        )
    }

    async fn reject(
        &self,
        command: RejectLlmCandidateCommand,
    ) -> LlmSpiResult<Option<LlmCandidate>> {
        self.decide_candidate(
            &command.scope,
            command.candidate_id,
            "rejected",
            command.decision_reason,
            command.decided_by,
        )
    }
}

impl ReferenceLlmRuntime {
    fn decide_candidate(
        &self,
        scope: &LlmScopeContext,
        candidate_id: String,
        decision_state: &str,
        decision_reason: Option<String>,
        decided_by: Option<i64>,
    ) -> LlmSpiResult<Option<LlmCandidate>> {
        let key = ScopedId::new(scope, candidate_id);
        let mut candidates = self.candidates.lock().map_err(lock_error)?;
        let Some(candidate) = candidates.get_mut(&key) else {
            return Ok(None);
        };

        candidate.decision_state = decision_state.to_string();
        candidate.decision_reason = decision_reason;
        candidate.decided_by = decided_by;
        candidate.decided_at = Some(now_text());

        Ok(Some(candidate.clone()))
    }
}

#[async_trait]
impl LlmHabitStorePort for ReferenceLlmRuntime {
    async fn upsert(&self, command: UpsertLlmHabitCommand) -> LlmSpiResult<LlmHabit> {
        let key = ScopedHabitKey::new(&command.scope, command.user_id, command.habit_key.clone());
        let habit = LlmHabit {
            habit_id: command.habit_id,
            user_id: command.user_id,
            habit_key: command.habit_key,
            habit_type: command.habit_type,
            description: command.description,
            stage: command.stage,
            strength: command.strength,
            confidence: command.confidence,
            support_count: command.support_count,
            last_signal_at: Some(now_text()),
            promoted_record_id: None,
            decay_after: None,
            metadata_json: command.metadata_json,
        };

        self.habits
            .lock()
            .map_err(lock_error)?
            .insert(key, habit.clone());

        Ok(habit)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmHabitQuery,
    ) -> LlmSpiResult<Option<LlmHabit>> {
        let key = ScopedHabitKey::new(&query.scope, query.user_id, query.habit_key);
        Ok(self.habits.lock().map_err(lock_error)?.get(&key).cloned())
    }

    async fn promote(
        &self,
        command: PromoteLlmHabitCommand,
    ) -> LlmSpiResult<Option<LlmHabit>> {
        let key = ScopedHabitKey::new(&command.scope, command.user_id, command.habit_key);
        let mut habits = self.habits.lock().map_err(lock_error)?;
        let Some(habit) = habits.get_mut(&key) else {
            return Ok(None);
        };

        habit.stage = "promoted".to_string();
        habit.promoted_record_id = command.promoted_record_id;

        Ok(Some(habit.clone()))
    }

    async fn decay(
        &self,
        command: DecayLlmHabitCommand,
    ) -> LlmSpiResult<Option<LlmHabit>> {
        let key = ScopedHabitKey::new(&command.scope, command.user_id, command.habit_key);
        let mut habits = self.habits.lock().map_err(lock_error)?;
        let Some(habit) = habits.get_mut(&key) else {
            return Ok(None);
        };

        habit.stage = "decayed".to_string();
        habit.strength = (habit.strength - command.strength_delta).max(0.0);

        Ok(Some(habit.clone()))
    }
}

#[async_trait]
impl LlmRetrievalTraceStorePort for ReferenceLlmRuntime {
    async fn append(
        &self,
        command: AppendLlmRetrievalTraceCommand,
    ) -> LlmSpiResult<LlmRetrievalTrace> {
        let trace = LlmRetrievalTrace {
            trace_id: command.trace_id.clone(),
            actor_id: command.actor_id,
            query_text: command.query_text,
            query_hash: command.query_hash,
            retrievers_json: command.retrievers_json,
            latency_ms: command.latency_ms,
            result_count: command.hits.len() as i64,
            degraded: command.degraded,
            metadata_json: command.metadata_json,
            hits: command.hits,
            context_pack: command.context_pack,
        };
        let key = ScopedId::new(&command.scope, command.trace_id);
        self.retrieval_traces
            .lock()
            .map_err(lock_error)?
            .insert(key, trace.clone());

        Ok(trace)
    }

    async fn retrieve(
        &self,
        query: RetrieveLlmRetrievalTraceQuery,
    ) -> LlmSpiResult<Option<LlmRetrievalTrace>> {
        let key = ScopedId::new(&query.scope, query.trace_id);
        Ok(self
            .retrieval_traces
            .lock()
            .map_err(lock_error)?
            .get(&key)
            .cloned())
    }

    async fn list_recent(
        &self,
        query: ListLlmRetrievalTracesQuery,
    ) -> LlmSpiResult<Vec<LlmRetrievalTrace>> {
        let traces = self.retrieval_traces.lock().map_err(lock_error)?;
        let mut recent = traces
            .iter()
            .filter(|(key, _trace)| key.matches_scope(&query.scope))
            .map(|(_, trace)| trace.clone())
            .collect::<Vec<_>>();
        recent.sort_by(|left, right| right.trace_id.cmp(&left.trace_id));
        recent.truncate(query.limit as usize);
        Ok(recent)
    }
}

#[async_trait]
impl LlmRetrieverPort for ReferenceLlmRuntime {
    fn retriever_code(&self) -> &str {
        "reference_keyword"
    }

    async fn retrieve(
        &self,
        command: RetrieveLlmCandidatesCommand,
    ) -> LlmSpiResult<LlmRetrieverResult> {
        let query = command.query.to_ascii_lowercase();
        let records = self.records.lock().map_err(lock_error)?;
        let mut record_ids = records
            .values()
            .filter_map(|state| {
                state
                    .visible_record()
                    .filter(|record| record.content.to_ascii_lowercase().contains(&query))
                    .map(|record| record.record_id)
            })
            .collect::<Vec<_>>();
        record_ids.sort();
        Ok(LlmRetrieverResult { record_ids })
    }
}

#[async_trait]
impl LlmIndexPort for ReferenceLlmRuntime {
    fn index_kind(&self) -> &str {
        "reference_sql_keyword"
    }

    async fn index(&self, record_id: String) -> LlmSpiResult<LlmIndexReceipt> {
        Ok(LlmIndexReceipt { record_id })
    }
}

#[async_trait]
impl ExternalLlmBridgePort for ReferenceLlmRuntime {
    fn provider_code(&self) -> &str {
        "reference_external_bridge_unconfigured"
    }

    async fn import(
        &self,
        _command: ExternalLlmImportCommand,
    ) -> LlmSpiResult<ExternalLlmImportResult> {
        Err(external_bridge_unconfigured())
    }

    async fn export(
        &self,
        _command: ExternalLlmExportCommand,
    ) -> LlmSpiResult<ExternalLlmExportResult> {
        Err(external_bridge_unconfigured())
    }

    async fn delete(
        &self,
        _command: ExternalLlmDeleteCommand,
    ) -> LlmSpiResult<ExternalLlmDeleteReceipt> {
        Err(external_bridge_unconfigured())
    }

    async fn shadow_read(
        &self,
        _command: ExternalLlmShadowReadCommand,
    ) -> LlmSpiResult<ExternalLlmShadowReadResult> {
        Err(external_bridge_unconfigured())
    }
}

#[async_trait]
impl LlmContextAssemblerPort for ReferenceLlmRuntime {
    async fn assemble(
        &self,
        command: AssembleLlmContextCommand,
    ) -> LlmSpiResult<LlmContextPackDraft> {
        let records = self.records.lock().map_err(lock_error)?;
        let context_lines = command
            .record_ids
            .iter()
            .filter_map(|record_id| {
                records.values().find_map(|state| {
                    state
                        .visible_record()
                        .filter(|record| &record.record_id == record_id)
                })
            })
            .map(|record| record.content)
            .collect::<Vec<_>>();

        Ok(LlmContextPackDraft {
            record_ids: command.record_ids,
            context_text: context_lines.join("\n"),
        })
    }
}

#[async_trait]
impl LlmEvaluationPort for ReferenceLlmRuntime {
    async fn run(&self, command: RunLlmEvalCommand) -> LlmSpiResult<LlmEvalRunResult> {
        Ok(LlmEvalRunResult {
            eval_type: command.eval_type,
        })
    }
}

#[derive(Debug, Clone)]
struct LlmRecordState {
    record: LlmRecord,
    deleted: bool,
}

impl LlmRecordState {
    fn active(record: LlmRecord) -> Self {
        Self {
            record,
            deleted: false,
        }
    }

    fn visible_record(&self) -> Option<LlmRecord> {
        if self.deleted {
            None
        } else {
            Some(self.record.clone())
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ScopedId {
    tenant_id: i64,
    space_id: i64,
    id: String,
}

impl ScopedId {
    fn new(scope: &LlmScopeContext, id: String) -> Self {
        Self {
            tenant_id: scope.tenant_id,
            space_id: scope.space_id,
            id,
        }
    }

    fn matches_scope(&self, scope: &LlmScopeContext) -> bool {
        self.tenant_id == scope.tenant_id && self.space_id == scope.space_id
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ScopedHabitKey {
    tenant_id: i64,
    space_id: i64,
    user_id: i64,
    habit_key: String,
}

impl ScopedHabitKey {
    fn new(scope: &LlmScopeContext, user_id: i64, habit_key: String) -> Self {
        Self {
            tenant_id: scope.tenant_id,
            space_id: scope.space_id,
            user_id,
            habit_key,
        }
    }
}

fn external_bridge_unconfigured() -> LlmSpiError {
    LlmSpiError::PortOperationFailed {
        port: "ExternalLlmBridgePort".to_string(),
        message: "reference external memory bridge is fail-closed until a reviewed provider adapter is configured".to_string(),
    }
}

fn lock_error<T>(_error: std::sync::PoisonError<T>) -> LlmSpiError {
    LlmSpiError::PortOperationFailed {
        port: "ReferenceLlmRuntime".to_string(),
        message: "reference runtime lock is poisoned".to_string(),
    }
}

fn now_text() -> String {
    sdkwork_utils_rust::format_datetime(sdkwork_utils_rust::now(), None)
}
