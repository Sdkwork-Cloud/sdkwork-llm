use crate::serde_int64::{
    deserialize_option_u64_from_string_or_number, deserialize_option_vec_u64_from_string_or_number,
    deserialize_u64_from_string_or_number, deserialize_vec_u64_from_string_or_number,
    serialize_option_u64_as_string, serialize_option_vec_u64_as_string, serialize_u64_as_string,
    serialize_vec_u64_as_string,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmRecordType {
    Working,
    Session,
    Semantic,
    Episodic,
    Procedural,
    Habit,
    Relationship,
    DomainKnowledge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmRetrieverKind {
    Sql,
    Keyword,
    Dictionary,
    Time,
    Event,
    Vector,
    Graph,
    GrepFile,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmProviderInterface {
    Llm,
    Embedding,
    Rerank,
    Tokenizer,
    Graph,
    Search,
    File,
    Memory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmImplementationKind {
    NativeSql,
    EventSourced,
    GraphTemporal,
    SearchFirst,
    LocalEmbedded,
    ExternalProviderBridge,
    HybridPlatform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmProviderHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmPageInfo {
    pub next_cursor: Option<String>,
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmCapabilities {
    pub embedding_optional: bool,
    pub retrievers: Vec<LlmRetrieverKind>,
    pub provider_interfaces: Vec<LlmProviderInterface>,
    pub implementation_kinds: Vec<LlmImplementationKind>,
    pub open_api_prefix: String,
    pub sdk_family: String,
    pub checked_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmEventRequest {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub user_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub event_type: String,
    pub source_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    pub event_time: String,
    pub payload: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmEvent {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub event_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub user_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    pub event_type: String,
    pub source_type: String,
    pub event_time: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    pub payload_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<String>,
    pub ingestion_status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRecordRequest {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub user_id: Option<u64>,
    pub scope: String,
    pub record_type: LlmRecordType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_text: Option<String>,
    pub canonical_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRecordPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRecord {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub record_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub user_id: Option<u64>,
    pub scope: String,
    pub record_type: LlmRecordType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_text: Option<String>,
    pub canonical_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_text: Option<String>,
    pub confidence: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contradiction_count: Option<i32>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub version: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRecordList {
    pub items: Vec<LlmRecord>,
    pub page_info: LlmPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLlmRecordsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRetrievalRequest {
    pub query: String,
    #[serde(
        serialize_with = "serialize_vec_u64_as_string",
        deserialize_with = "deserialize_vec_u64_from_string_or_number"
    )]
    pub space_ids: Vec<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub retrieval_profile_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_types: Option<Vec<LlmRecordType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Value>,
    pub top_k: i32,
    pub context_budget_tokens: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_trace: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRetrievalHit {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub hit_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<LlmRecord>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub record_id: Option<u64>,
    pub retriever_name: String,
    pub result_rank: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fused_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Value>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRetrievalTrace {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub trace_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub space_id: Option<u64>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub retrieval_profile_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    pub query_hash: String,
    pub result_count: i32,
    pub degraded: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRetrievalResult {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub retrieval_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<LlmRetrievalTrace>,
    pub hits: Vec<LlmRetrievalHit>,
    pub degraded: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryProviderBinding {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub provider_binding_id: u64,
    pub provider_kind: String,
    pub provider_code: String,
    pub display_name: String,
    pub capabilities: Vec<String>,
    pub health_state: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub version: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmProviderHealth {
    pub status: LlmProviderHealthStatus,
    pub checked_at: String,
    pub providers: Vec<MemoryProviderBinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmContextPackRequest {
    pub query: String,
    #[serde(
        serialize_with = "serialize_vec_u64_as_string",
        deserialize_with = "deserialize_vec_u64_from_string_or_number"
    )]
    pub space_ids: Vec<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub retrieval_profile_id: Option<u64>,
    pub context_budget_tokens: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_citations: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmContextPack {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub context_pack_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub retrieval_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    pub pack: Value,
    pub estimated_tokens: i32,
    pub truncated: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmFeedbackRequest {
    pub target_type: String,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub target_id: u64,
    pub feedback_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryFeedback {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub feedback_id: u64,
    pub target_type: String,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub target_id: u64,
    pub feedback_type: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmExtractionRequest {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    #[serde(
        serialize_with = "serialize_vec_u64_as_string",
        deserialize_with = "deserialize_vec_u64_from_string_or_number"
    )]
    pub input_events: Vec<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extraction_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmLearningJob {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub job_id: u64,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub space_id: Option<u64>,
    pub job_type: String,
    pub state: String,
    pub priority: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
}

/// Legacy alias kept for app-api extraction responses that mirror learning jobs.
pub type MemoryExtractionJob = LlmLearningJob;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmCandidate {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub candidate_id: u64,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    pub candidate_type: String,
    pub record_type: LlmRecordType,
    pub proposed_text: String,
    pub confidence: f64,
    pub decision_state: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmCandidateList {
    pub items: Vec<LlmCandidate>,
    pub page_info: LlmPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmHabit {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub habit_id: u64,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub space_id: u64,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub user_id: u64,
    pub habit_key: String,
    pub habit_type: String,
    pub description: String,
    pub stage: String,
    pub strength: f64,
    pub confidence: f64,
    pub support_count: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_signal_at: Option<String>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub promoted_record_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decay_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub version: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmHabitList {
    pub items: Vec<LlmHabit>,
    pub page_info: LlmPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmHabitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmReviewRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListHabitsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAuditLogsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLlmCandidatesQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmEventList {
    pub items: Vec<LlmEvent>,
    pub page_info: LlmPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEventsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRetrievalTraceList {
    pub items: Vec<LlmRetrievalTrace>,
    pub page_info: LlmPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRetrievalTracesQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmLearningSettings {
    pub auto_promote_candidates: bool,
    pub habit_learning_enabled: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmLearningSettingsPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_promote_candidates: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub habit_learning_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRecordSource {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub source_id: u64,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub record_id: u64,
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub event_id: u64,
    pub source_role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_delta: Option<f64>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRecordSourceList {
    pub items: Vec<LlmRecordSource>,
    pub page_info: LlmPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmForgetRequest {
    pub scope: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_vec_u64_as_string",
        deserialize_with = "deserialize_option_vec_u64_from_string_or_number"
    )]
    pub record_ids: Option<Vec<u64>>,
    #[serde(
        default,
        serialize_with = "serialize_option_u64_as_string",
        deserialize_with = "deserialize_option_u64_from_string_or_number"
    )]
    pub space_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmForgetJob {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub forget_request_id: u64,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmExportRequest {
    #[serde(
        serialize_with = "serialize_vec_u64_as_string",
        deserialize_with = "deserialize_vec_u64_from_string_or_number"
    )]
    pub space_ids: Vec<u64>,
    pub format: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_events: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drive_target_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmExportJob {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_u64_from_string_or_number"
    )]
    pub export_job_id: u64,
    pub state: String,
    pub format: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drive_object_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
}
