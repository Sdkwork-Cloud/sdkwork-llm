use async_trait::async_trait;

use crate::dto::{
    ListAuditLogsQuery, ListLlmCandidatesQuery, ListEventsQuery, ListLlmRecordsQuery,
    ListRetrievalTracesQuery, LlmCandidate, LlmCandidateList, LlmEvent, LlmEventList,
    LlmProviderHealth, LlmRecord, LlmRecordList, LlmRecordPatch,
    LlmRetrievalTraceList,
};
use crate::ports::{LlmServiceError, LlmServiceResult};
use crate::space::{ListSpacesQuery, LlmSpace, LlmSpaceList, LlmSpaceRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmBackendRequestContext {
    pub tenant_id: u64,
    pub operator_id: Option<u64>,
}

macro_rules! backend_not_implemented {
    ($name:literal, $ret:ty) => {
        Err(LlmServiceError::not_implemented($name)) as LlmServiceResult<$ret>
    };
}

#[async_trait]
pub trait LlmBackendApi: Send + Sync + 'static {
    async fn list_spaces(
        &self,
        _context: LlmBackendRequestContext,
        _query: ListSpacesQuery,
    ) -> LlmServiceResult<LlmSpaceList> {
        backend_not_implemented!("spaces.list", LlmSpaceList)
    }

    async fn retrieve_space(
        &self,
        _context: LlmBackendRequestContext,
        _space_id: u64,
    ) -> LlmServiceResult<LlmSpace> {
        backend_not_implemented!("spaces.retrieve", LlmSpace)
    }

    async fn update_space(
        &self,
        _context: LlmBackendRequestContext,
        _space_id: u64,
        _request: LlmSpaceRequest,
    ) -> LlmServiceResult<LlmSpace> {
        backend_not_implemented!("spaces.update", LlmSpace)
    }

    async fn list_memories(
        &self,
        _context: LlmBackendRequestContext,
        _query: ListLlmRecordsQuery,
    ) -> LlmServiceResult<LlmRecordList> {
        backend_not_implemented!("records.list", LlmRecordList)
    }

    async fn retrieve_memory(
        &self,
        _context: LlmBackendRequestContext,
        _record_id: u64,
    ) -> LlmServiceResult<LlmRecord> {
        backend_not_implemented!("records.retrieve", LlmRecord)
    }

    async fn update_memory(
        &self,
        _context: LlmBackendRequestContext,
        _record_id: u64,
        _patch: LlmRecordPatch,
    ) -> LlmServiceResult<LlmRecord> {
        backend_not_implemented!("records.update", LlmRecord)
    }

    async fn supersede_memory(
        &self,
        _context: LlmBackendRequestContext,
        _record_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmRecord> {
        backend_not_implemented!("memories.supersede", LlmRecord)
    }

    async fn list_events(
        &self,
        _context: LlmBackendRequestContext,
        _query: ListEventsQuery,
    ) -> LlmServiceResult<LlmEventList> {
        backend_not_implemented!("events.list", LlmEventList)
    }

    async fn retrieve_event(
        &self,
        _context: LlmBackendRequestContext,
        _event_id: u64,
    ) -> LlmServiceResult<LlmEvent> {
        backend_not_implemented!("events.retrieve", LlmEvent)
    }

    async fn list_candidates(
        &self,
        _context: LlmBackendRequestContext,
        _query: ListLlmCandidatesQuery,
    ) -> LlmServiceResult<LlmCandidateList> {
        backend_not_implemented!("candidates.list", LlmCandidateList)
    }

    async fn approve_candidate(
        &self,
        _context: LlmBackendRequestContext,
        _candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        backend_not_implemented!("candidates.approve", LlmCandidate)
    }

    async fn reject_candidate(
        &self,
        _context: LlmBackendRequestContext,
        _candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        backend_not_implemented!("candidates.reject", LlmCandidate)
    }

    async fn create_extraction_job(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("extractionJobs.create", serde_json::Value)
    }

    async fn retrieve_extraction_job(
        &self,
        _context: LlmBackendRequestContext,
        _job_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("extractionJobs.retrieve", serde_json::Value)
    }

    async fn create_consolidation_job(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("consolidationJobs.create", serde_json::Value)
    }

    async fn list_indexes(
        &self,
        _context: LlmBackendRequestContext,
        _query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("indexes.list", serde_json::Value)
    }

    async fn create_index(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("indexes.create", serde_json::Value)
    }

    async fn retrieve_index(
        &self,
        _context: LlmBackendRequestContext,
        _index_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("indexes.retrieve", serde_json::Value)
    }

    async fn update_index(
        &self,
        _context: LlmBackendRequestContext,
        _index_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("indexes.update", serde_json::Value)
    }

    async fn rebuild_index(
        &self,
        _context: LlmBackendRequestContext,
        _index_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("indexes.rebuild", serde_json::Value)
    }

    async fn list_retrieval_profiles(
        &self,
        _context: LlmBackendRequestContext,
        _query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("retrievalProfiles.list", serde_json::Value)
    }

    async fn create_retrieval_profile(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("retrievalProfiles.create", serde_json::Value)
    }

    async fn retrieve_retrieval_profile(
        &self,
        _context: LlmBackendRequestContext,
        _profile_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("retrievalProfiles.retrieve", serde_json::Value)
    }

    async fn update_retrieval_profile(
        &self,
        _context: LlmBackendRequestContext,
        _profile_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("retrievalProfiles.update", serde_json::Value)
    }

    async fn list_implementation_profiles(
        &self,
        _context: LlmBackendRequestContext,
        _query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("implementationProfiles.list", serde_json::Value)
    }

    async fn create_implementation_profile(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("implementationProfiles.create", serde_json::Value)
    }

    async fn retrieve_implementation_profile(
        &self,
        _context: LlmBackendRequestContext,
        _profile_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("implementationProfiles.retrieve", serde_json::Value)
    }

    async fn update_implementation_profile(
        &self,
        _context: LlmBackendRequestContext,
        _profile_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("implementationProfiles.update", serde_json::Value)
    }

    async fn list_provider_bindings(
        &self,
        _context: LlmBackendRequestContext,
        _query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("providerBindings.list", serde_json::Value)
    }

    async fn create_provider_binding(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("providerBindings.create", serde_json::Value)
    }

    async fn update_provider_binding(
        &self,
        _context: LlmBackendRequestContext,
        _provider_binding_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("providerBindings.update", serde_json::Value)
    }

    async fn retrieve_provider_health(
        &self,
        _context: LlmBackendRequestContext,
    ) -> LlmServiceResult<LlmProviderHealth> {
        backend_not_implemented!("providerHealth.retrieve", LlmProviderHealth)
    }

    async fn list_eval_runs(
        &self,
        _context: LlmBackendRequestContext,
        _query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("evalRuns.list", serde_json::Value)
    }

    async fn create_eval_run(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("evalRuns.create", serde_json::Value)
    }

    async fn retrieve_eval_run(
        &self,
        _context: LlmBackendRequestContext,
        _eval_run_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("evalRuns.retrieve", serde_json::Value)
    }

    async fn list_retrieval_traces(
        &self,
        _context: LlmBackendRequestContext,
        _query: ListRetrievalTracesQuery,
    ) -> LlmServiceResult<LlmRetrievalTraceList> {
        backend_not_implemented!("retrievalTraces.list", LlmRetrievalTraceList)
    }

    async fn retrieve_retrieval_trace(
        &self,
        _context: LlmBackendRequestContext,
        _trace_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("retrievalTraces.retrieve", serde_json::Value)
    }

    async fn list_audit_logs(
        &self,
        _context: LlmBackendRequestContext,
        _query: ListAuditLogsQuery,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("auditLogs.list", serde_json::Value)
    }

    async fn create_retention_job(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("retentionJobs.create", serde_json::Value)
    }

    async fn create_migration_job(
        &self,
        _context: LlmBackendRequestContext,
        _request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("migrationJobs.create", serde_json::Value)
    }

    async fn retrieve_migration_job(
        &self,
        _context: LlmBackendRequestContext,
        _migration_job_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        backend_not_implemented!("migrationJobs.retrieve", serde_json::Value)
    }
}
