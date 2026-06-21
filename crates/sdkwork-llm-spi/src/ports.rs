use async_trait::async_trait;

use crate::LlmSpiResult;

pub trait LlmRuntimePlugin: Send + Sync {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmScopeContext {
    pub tenant_id: i64,
    pub space_id: i64,
    pub organization_id: Option<i64>,
    pub user_id: Option<i64>,
}

impl LlmScopeContext {
    pub fn for_test(tenant_id: i64, space_id: i64) -> Self {
        Self {
            tenant_id,
            space_id,
            organization_id: None,
            user_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLlmRecordCommand {
    pub scope: LlmScopeContext,
    pub record_id: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmRecord {
    pub record_id: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmRecordQuery {
    pub scope: LlmScopeContext,
    pub record_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteLlmRecordCommand {
    pub scope: LlmScopeContext,
    pub record_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmDeletionReceipt {
    pub record_id: String,
    pub deleted: bool,
    pub already_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendLlmEventCommand {
    pub scope: LlmScopeContext,
    pub event_id: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmEvent {
    pub event_id: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmEventQuery {
    pub scope: LlmScopeContext,
    pub event_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmAuditRecord {
    pub audit_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendLlmAuditCommand {
    pub scope: LlmScopeContext,
    pub audit_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmAuditQuery {
    pub scope: LlmScopeContext,
    pub audit_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmOutboxEvent {
    pub outbox_id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub event_version: String,
    pub payload_json: String,
    pub publish_state: String,
    pub published_at: Option<String>,
    pub retry_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendLlmOutboxCommand {
    pub scope: LlmScopeContext,
    pub outbox_id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub event_version: String,
    pub payload_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmOutboxQuery {
    pub scope: LlmScopeContext,
    pub outbox_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPendingLlmOutboxQuery {
    pub scope: LlmScopeContext,
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkLlmOutboxPublishedCommand {
    pub scope: LlmScopeContext,
    pub outbox_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkLlmOutboxFailedCommand {
    pub scope: LlmScopeContext,
    pub outbox_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateLlmCandidateCommand {
    pub scope: LlmScopeContext,
    pub candidate_id: String,
    pub candidate_type: String,
    pub record_type: String,
    pub proposed_text: String,
    pub proposed_payload_json: Option<String>,
    pub evidence_json: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmCandidateQuery {
    pub scope: LlmScopeContext,
    pub candidate_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApproveLlmCandidateCommand {
    pub scope: LlmScopeContext,
    pub candidate_id: String,
    pub decision_reason: Option<String>,
    pub decided_by: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectLlmCandidateCommand {
    pub scope: LlmScopeContext,
    pub candidate_id: String,
    pub decision_reason: Option<String>,
    pub decided_by: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlmCandidate {
    pub candidate_id: String,
    pub candidate_type: String,
    pub record_type: String,
    pub proposed_text: String,
    pub proposed_payload_json: Option<String>,
    pub evidence_json: Option<String>,
    pub confidence: f64,
    pub decision_state: String,
    pub decision_reason: Option<String>,
    pub decided_by: Option<i64>,
    pub decided_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpsertLlmHabitCommand {
    pub scope: LlmScopeContext,
    pub habit_id: String,
    pub user_id: i64,
    pub habit_key: String,
    pub habit_type: String,
    pub description: String,
    pub stage: String,
    pub strength: f64,
    pub confidence: f64,
    pub support_count: i64,
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmHabitQuery {
    pub scope: LlmScopeContext,
    pub user_id: i64,
    pub habit_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromoteLlmHabitCommand {
    pub scope: LlmScopeContext,
    pub user_id: i64,
    pub habit_key: String,
    pub promoted_record_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecayLlmHabitCommand {
    pub scope: LlmScopeContext,
    pub user_id: i64,
    pub habit_key: String,
    pub strength_delta: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlmHabit {
    pub habit_id: String,
    pub user_id: i64,
    pub habit_key: String,
    pub habit_type: String,
    pub description: String,
    pub stage: String,
    pub strength: f64,
    pub confidence: f64,
    pub support_count: i64,
    pub last_signal_at: Option<String>,
    pub promoted_record_id: Option<String>,
    pub decay_after: Option<String>,
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlmRetrievalHitDraft {
    pub hit_id: String,
    pub record_id: Option<String>,
    pub retriever_name: String,
    pub result_rank: i64,
    pub raw_score: Option<f64>,
    pub fused_score: Option<f64>,
    pub explanation_json: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmContextPackSnapshot {
    pub context_pack_id: String,
    pub pack_json: String,
    pub estimated_tokens: i64,
    pub truncated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppendLlmRetrievalTraceCommand {
    pub scope: LlmScopeContext,
    pub trace_id: String,
    pub actor_id: Option<String>,
    pub query_text: Option<String>,
    pub query_hash: String,
    pub retrievers_json: Option<String>,
    pub latency_ms: Option<i64>,
    pub degraded: bool,
    pub metadata_json: Option<String>,
    pub hits: Vec<LlmRetrievalHitDraft>,
    pub context_pack: Option<LlmContextPackSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmRetrievalTraceQuery {
    pub scope: LlmScopeContext,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListLlmRetrievalTracesQuery {
    pub scope: LlmScopeContext,
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlmRetrievalTrace {
    pub trace_id: String,
    pub actor_id: Option<String>,
    pub query_text: Option<String>,
    pub query_hash: String,
    pub retrievers_json: Option<String>,
    pub latency_ms: Option<i64>,
    pub result_count: i64,
    pub degraded: bool,
    pub metadata_json: Option<String>,
    pub hits: Vec<LlmRetrievalHitDraft>,
    pub context_pack: Option<LlmContextPackSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPolicy {
    pub policy_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveLlmCandidatesCommand {
    pub query: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmRetrieverResult {
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmIndexReceipt {
    pub record_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageModelCommand {
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddingCommand {
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RerankMemoryHitsCommand {
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RerankMemoryHitsResult {
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmImportCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmImportResult {
    pub imported_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmExportCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmExportResult {
    pub exported_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmDeleteCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmDeleteReceipt {
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmShadowReadCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlmShadowReadResult {
    pub comparable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssembleLlmContextCommand {
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmContextPackDraft {
    pub record_ids: Vec<String>,
    pub context_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunLlmEvalCommand {
    pub eval_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmEvalRunResult {
    pub eval_type: String,
}

#[async_trait]
pub trait LlmRecordStorePort: Send + Sync {
    async fn create(&self, command: CreateLlmRecordCommand) -> LlmSpiResult<LlmRecord>;

    async fn retrieve(
        &self,
        query: RetrieveLlmRecordQuery,
    ) -> LlmSpiResult<Option<LlmRecord>>;

    async fn mark_deleted(
        &self,
        command: DeleteLlmRecordCommand,
    ) -> LlmSpiResult<LlmDeletionReceipt>;
}

#[async_trait]
pub trait LlmEventStorePort: Send + Sync {
    async fn append(&self, command: AppendLlmEventCommand) -> LlmSpiResult<LlmEvent>;

    async fn retrieve(
        &self,
        query: RetrieveLlmEventQuery,
    ) -> LlmSpiResult<Option<LlmEvent>>;
}

#[async_trait]
pub trait LlmAuditStorePort: Send + Sync {
    async fn append(&self, command: AppendLlmAuditCommand)
        -> LlmSpiResult<LlmAuditRecord>;

    async fn retrieve(
        &self,
        query: RetrieveLlmAuditQuery,
    ) -> LlmSpiResult<Option<LlmAuditRecord>>;
}

#[async_trait]
pub trait LlmOutboxStorePort: Send + Sync {
    async fn append(
        &self,
        command: AppendLlmOutboxCommand,
    ) -> LlmSpiResult<LlmOutboxEvent>;

    async fn retrieve(
        &self,
        query: RetrieveLlmOutboxQuery,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>>;

    async fn list_pending(
        &self,
        query: ListPendingLlmOutboxQuery,
    ) -> LlmSpiResult<Vec<LlmOutboxEvent>>;

    async fn mark_published(
        &self,
        command: MarkLlmOutboxPublishedCommand,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>>;

    async fn mark_failed(
        &self,
        command: MarkLlmOutboxFailedCommand,
    ) -> LlmSpiResult<Option<LlmOutboxEvent>>;
}

#[async_trait]
pub trait LlmCandidateStorePort: Send + Sync {
    async fn create(
        &self,
        command: CreateLlmCandidateCommand,
    ) -> LlmSpiResult<LlmCandidate>;

    async fn retrieve(
        &self,
        query: RetrieveLlmCandidateQuery,
    ) -> LlmSpiResult<Option<LlmCandidate>>;

    async fn approve(
        &self,
        command: ApproveLlmCandidateCommand,
    ) -> LlmSpiResult<Option<LlmCandidate>>;

    async fn reject(
        &self,
        command: RejectLlmCandidateCommand,
    ) -> LlmSpiResult<Option<LlmCandidate>>;
}

#[async_trait]
pub trait LlmHabitStorePort: Send + Sync {
    async fn upsert(&self, command: UpsertLlmHabitCommand) -> LlmSpiResult<LlmHabit>;

    async fn retrieve(
        &self,
        query: RetrieveLlmHabitQuery,
    ) -> LlmSpiResult<Option<LlmHabit>>;

    async fn promote(
        &self,
        command: PromoteLlmHabitCommand,
    ) -> LlmSpiResult<Option<LlmHabit>>;

    async fn decay(&self, command: DecayLlmHabitCommand)
        -> LlmSpiResult<Option<LlmHabit>>;
}

#[async_trait]
pub trait LlmRetrievalTraceStorePort: Send + Sync {
    async fn append(
        &self,
        command: AppendLlmRetrievalTraceCommand,
    ) -> LlmSpiResult<LlmRetrievalTrace>;

    async fn retrieve(
        &self,
        query: RetrieveLlmRetrievalTraceQuery,
    ) -> LlmSpiResult<Option<LlmRetrievalTrace>>;

    async fn list_recent(
        &self,
        query: ListLlmRetrievalTracesQuery,
    ) -> LlmSpiResult<Vec<LlmRetrievalTrace>>;
}

#[async_trait]
pub trait MemoryPolicyStorePort: Send + Sync {
    async fn resolve_policy(&self, policy_code: String) -> LlmSpiResult<MemoryPolicy>;
}

#[async_trait]
pub trait LlmRetrieverPort: Send + Sync {
    fn retriever_code(&self) -> &str;

    async fn retrieve(
        &self,
        command: RetrieveLlmCandidatesCommand,
    ) -> LlmSpiResult<LlmRetrieverResult>;
}

#[async_trait]
pub trait LlmIndexPort: Send + Sync {
    fn index_kind(&self) -> &str;

    async fn index(&self, record_id: String) -> LlmSpiResult<LlmIndexReceipt>;
}

#[async_trait]
pub trait LanguageModelPort: Send + Sync {
    fn provider_code(&self) -> &str;

    async fn generate(&self, command: LanguageModelCommand) -> LlmSpiResult<String>;
}

#[async_trait]
pub trait EmbeddingModelPort: Send + Sync {
    fn provider_code(&self) -> &str;

    fn dimensions(&self) -> usize;

    async fn embed(&self, command: EmbeddingCommand) -> LlmSpiResult<Vec<f32>>;
}

#[async_trait]
pub trait RerankModelPort: Send + Sync {
    fn provider_code(&self) -> &str;

    async fn rerank(
        &self,
        command: RerankMemoryHitsCommand,
    ) -> LlmSpiResult<RerankMemoryHitsResult>;
}

#[async_trait]
pub trait ExternalLlmBridgePort: Send + Sync {
    fn provider_code(&self) -> &str;

    async fn import(
        &self,
        command: ExternalLlmImportCommand,
    ) -> LlmSpiResult<ExternalLlmImportResult>;

    async fn export(
        &self,
        command: ExternalLlmExportCommand,
    ) -> LlmSpiResult<ExternalLlmExportResult>;

    async fn delete(
        &self,
        command: ExternalLlmDeleteCommand,
    ) -> LlmSpiResult<ExternalLlmDeleteReceipt>;

    async fn shadow_read(
        &self,
        command: ExternalLlmShadowReadCommand,
    ) -> LlmSpiResult<ExternalLlmShadowReadResult>;
}

#[async_trait]
pub trait LlmContextAssemblerPort: Send + Sync {
    async fn assemble(
        &self,
        command: AssembleLlmContextCommand,
    ) -> LlmSpiResult<LlmContextPackDraft>;
}

#[async_trait]
pub trait LlmEvaluationPort: Send + Sync {
    async fn run(&self, command: RunLlmEvalCommand) -> LlmSpiResult<LlmEvalRunResult>;
}
