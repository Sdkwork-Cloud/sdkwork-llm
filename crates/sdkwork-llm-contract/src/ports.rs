use async_trait::async_trait;

use crate::dto::{
    ListLlmCandidatesQuery, ListLlmRecordsQuery, LlmCandidate, LlmCandidateList,
    LlmCapabilities, LlmContextPack, LlmContextPackRequest, LlmEvent,
    LlmEventRequest, LlmExtractionRequest, MemoryFeedback, LlmFeedbackRequest,
    LlmLearningJob, LlmProviderHealth, LlmRecord, LlmRecordList, LlmRecordPatch,
    LlmRecordRequest, LlmRetrievalRequest, LlmRetrievalResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmOpenApiRequestContext {
    pub api_key_id: String,
    pub tenant_id: u64,
    pub actor_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmServiceErrorKind {
    NotFound,
    Conflict,
    Validation,
    Storage,
    NotImplemented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmServiceError {
    pub kind: LlmServiceErrorKind,
    pub code: String,
    pub detail: String,
}

impl LlmServiceError {
    pub fn not_found(detail: impl Into<String>) -> Self {
        Self {
            kind: LlmServiceErrorKind::NotFound,
            code: "not_found".to_string(),
            detail: detail.into(),
        }
    }

    pub fn conflict(detail: impl Into<String>) -> Self {
        Self {
            kind: LlmServiceErrorKind::Conflict,
            code: "conflict".to_string(),
            detail: detail.into(),
        }
    }

    pub fn validation(detail: impl Into<String>) -> Self {
        Self {
            kind: LlmServiceErrorKind::Validation,
            code: "validation_error".to_string(),
            detail: detail.into(),
        }
    }

    pub fn storage(detail: impl Into<String>) -> Self {
        Self {
            kind: LlmServiceErrorKind::Storage,
            code: "storage_error".to_string(),
            detail: detail.into(),
        }
    }

    pub fn not_implemented(operation_id: &'static str) -> Self {
        Self {
            kind: LlmServiceErrorKind::NotImplemented,
            code: "operation_not_implemented".to_string(),
            detail: format!("operation is not implemented: {operation_id}"),
        }
    }
}

pub type LlmServiceResult<T> = Result<T, LlmServiceError>;

#[async_trait]
pub trait LlmOpenApi: Send + Sync + 'static {
    async fn retrieve_capabilities(
        &self,
        _context: LlmOpenApiRequestContext,
    ) -> LlmServiceResult<LlmCapabilities> {
        Err(LlmServiceError::not_implemented("capabilities.retrieve"))
    }

    async fn create_event(
        &self,
        _context: LlmOpenApiRequestContext,
        _request: LlmEventRequest,
    ) -> LlmServiceResult<LlmEvent> {
        Err(LlmServiceError::not_implemented("events.create"))
    }

    async fn retrieve_event(
        &self,
        _context: LlmOpenApiRequestContext,
        _event_id: u64,
    ) -> LlmServiceResult<LlmEvent> {
        Err(LlmServiceError::not_implemented("events.retrieve"))
    }

    async fn list_memories(
        &self,
        _context: LlmOpenApiRequestContext,
        _query: ListLlmRecordsQuery,
    ) -> LlmServiceResult<LlmRecordList> {
        Err(LlmServiceError::not_implemented("records.list"))
    }

    async fn create_memory(
        &self,
        _context: LlmOpenApiRequestContext,
        _request: LlmRecordRequest,
    ) -> LlmServiceResult<LlmRecord> {
        Err(LlmServiceError::not_implemented("records.create"))
    }

    async fn retrieve_memory(
        &self,
        _context: LlmOpenApiRequestContext,
        _record_id: u64,
    ) -> LlmServiceResult<LlmRecord> {
        Err(LlmServiceError::not_implemented("records.retrieve"))
    }

    async fn update_memory(
        &self,
        _context: LlmOpenApiRequestContext,
        _record_id: u64,
        _patch: LlmRecordPatch,
    ) -> LlmServiceResult<LlmRecord> {
        Err(LlmServiceError::not_implemented("records.update"))
    }

    async fn delete_memory(
        &self,
        _context: LlmOpenApiRequestContext,
        _record_id: u64,
    ) -> LlmServiceResult<()> {
        Err(LlmServiceError::not_implemented("records.delete"))
    }

    async fn create_retrieval(
        &self,
        _context: LlmOpenApiRequestContext,
        _request: LlmRetrievalRequest,
    ) -> LlmServiceResult<LlmRetrievalResult> {
        Err(LlmServiceError::not_implemented("retrievals.create"))
    }

    async fn retrieve_retrieval(
        &self,
        _context: LlmOpenApiRequestContext,
        _retrieval_id: u64,
    ) -> LlmServiceResult<LlmRetrievalResult> {
        Err(LlmServiceError::not_implemented("retrievals.retrieve"))
    }

    async fn create_context_pack(
        &self,
        _context: LlmOpenApiRequestContext,
        _request: LlmContextPackRequest,
    ) -> LlmServiceResult<LlmContextPack> {
        Err(LlmServiceError::not_implemented("contextPacks.create"))
    }

    async fn retrieve_context_pack(
        &self,
        _context: LlmOpenApiRequestContext,
        _context_pack_id: u64,
    ) -> LlmServiceResult<LlmContextPack> {
        Err(LlmServiceError::not_implemented("contextPacks.retrieve"))
    }

    async fn create_feedback(
        &self,
        _context: LlmOpenApiRequestContext,
        _request: LlmFeedbackRequest,
    ) -> LlmServiceResult<MemoryFeedback> {
        Err(LlmServiceError::not_implemented("feedback.create"))
    }

    async fn create_extraction(
        &self,
        _context: LlmOpenApiRequestContext,
        _request: LlmExtractionRequest,
    ) -> LlmServiceResult<LlmLearningJob> {
        Err(LlmServiceError::not_implemented("extractions.create"))
    }

    async fn list_candidates(
        &self,
        _context: LlmOpenApiRequestContext,
        _query: ListLlmCandidatesQuery,
    ) -> LlmServiceResult<LlmCandidateList> {
        Err(LlmServiceError::not_implemented("candidates.list"))
    }

    async fn retrieve_candidate(
        &self,
        _context: LlmOpenApiRequestContext,
        _candidate_id: u64,
    ) -> LlmServiceResult<LlmCandidate> {
        Err(LlmServiceError::not_implemented("candidates.retrieve"))
    }

    async fn retrieve_provider_health(
        &self,
        _context: LlmOpenApiRequestContext,
    ) -> LlmServiceResult<LlmProviderHealth> {
        Err(LlmServiceError::not_implemented(
            "providerHealth.retrieve",
        ))
    }
}
