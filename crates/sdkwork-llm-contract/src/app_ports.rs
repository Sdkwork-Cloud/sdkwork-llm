use async_trait::async_trait;

use crate::dto::{
    ListLlmCandidatesQuery, ListHabitsQuery, ListLlmRecordsQuery, LlmCandidate, LlmCandidateList,
    LlmContextPack, LlmContextPackRequest, LlmEvent, LlmEventRequest, LlmExportJob,
    LlmExportRequest, LlmExtractionRequest, MemoryFeedback, LlmFeedbackRequest,
    LlmForgetJob, LlmForgetRequest, LlmHabit, LlmHabitList, LlmHabitRequest,
    LlmLearningJob, LlmLearningSettings, LlmLearningSettingsPatch, LlmRecord,
    LlmRecordList, LlmRecordPatch, LlmRecordRequest, LlmRecordSourceList,
    LlmRetrievalRequest, LlmRetrievalResult, LlmReviewRequest,
};
use crate::ports::{LlmServiceError, LlmServiceResult};
use crate::space::{ListSpacesQuery, LlmSpace, LlmSpaceList, LlmSpaceRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmAppRequestContext {
    pub tenant_id: u64,
    pub actor_id: Option<u64>,
    pub organization_id: Option<u64>,
    pub session_id: Option<String>,
}

macro_rules! app_not_implemented {
    ($name:literal, $ret:ty) => {
        Err(LlmServiceError::not_implemented($name)) as LlmServiceResult<$ret>
    };
}

#[async_trait]
pub trait LlmAppApi: Send + Sync + 'static {
    async fn list_spaces(
        &self,
        _context: LlmAppRequestContext,
        _query: ListSpacesQuery,
    ) -> LlmServiceResult<LlmSpaceList> {
        app_not_implemented!("spaces.list", LlmSpaceList)
    }

    async fn create_space(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmSpaceRequest,
    ) -> LlmServiceResult<LlmSpace> {
        app_not_implemented!("spaces.create", LlmSpace)
    }

    async fn retrieve_space(
        &self,
        _context: LlmAppRequestContext,
        _space_id: u64,
    ) -> LlmServiceResult<LlmSpace> {
        app_not_implemented!("spaces.retrieve", LlmSpace)
    }

    async fn update_space(
        &self,
        _context: LlmAppRequestContext,
        _space_id: u64,
        _request: LlmSpaceRequest,
    ) -> LlmServiceResult<LlmSpace> {
        app_not_implemented!("spaces.update", LlmSpace)
    }

    async fn create_event(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmEventRequest,
    ) -> LlmServiceResult<LlmEvent> {
        app_not_implemented!("events.create", LlmEvent)
    }

    async fn retrieve_event(
        &self,
        _context: LlmAppRequestContext,
        _event_id: u64,
    ) -> LlmServiceResult<LlmEvent> {
        app_not_implemented!("events.retrieve", LlmEvent)
    }

    async fn list_memories(
        &self,
        _context: LlmAppRequestContext,
        _query: ListLlmRecordsQuery,
    ) -> LlmServiceResult<LlmRecordList> {
        app_not_implemented!("records.list", LlmRecordList)
    }

    async fn create_memory(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmRecordRequest,
    ) -> LlmServiceResult<LlmRecord> {
        app_not_implemented!("records.create", LlmRecord)
    }

    async fn retrieve_memory(
        &self,
        _context: LlmAppRequestContext,
        _record_id: u64,
    ) -> LlmServiceResult<LlmRecord> {
        app_not_implemented!("records.retrieve", LlmRecord)
    }

    async fn update_memory(
        &self,
        _context: LlmAppRequestContext,
        _record_id: u64,
        _patch: LlmRecordPatch,
    ) -> LlmServiceResult<LlmRecord> {
        app_not_implemented!("records.update", LlmRecord)
    }

    async fn delete_memory(
        &self,
        _context: LlmAppRequestContext,
        _record_id: u64,
    ) -> LlmServiceResult<()> {
        app_not_implemented!("records.delete", ())
    }

    async fn list_memory_sources(
        &self,
        _context: LlmAppRequestContext,
        _record_id: u64,
    ) -> LlmServiceResult<LlmRecordSourceList> {
        app_not_implemented!("records.sources.list", LlmRecordSourceList)
    }

    async fn create_forget_request(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmForgetRequest,
    ) -> LlmServiceResult<LlmForgetJob> {
        app_not_implemented!("forgetRequests.create", LlmForgetJob)
    }

    async fn retrieve_forget_request(
        &self,
        _context: LlmAppRequestContext,
        _forget_request_id: u64,
    ) -> LlmServiceResult<LlmForgetJob> {
        app_not_implemented!("forgetRequests.retrieve", LlmForgetJob)
    }

    async fn create_extraction(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmExtractionRequest,
    ) -> LlmServiceResult<LlmLearningJob> {
        app_not_implemented!("extractions.create", LlmLearningJob)
    }

    async fn list_candidates(
        &self,
        _context: LlmAppRequestContext,
        _query: ListLlmCandidatesQuery,
    ) -> LlmServiceResult<LlmCandidateList> {
        app_not_implemented!("candidates.list", LlmCandidateList)
    }

    async fn retrieve_candidate(
        &self,
        _context: LlmAppRequestContext,
        _candidate_id: u64,
    ) -> LlmServiceResult<LlmCandidate> {
        app_not_implemented!("candidates.retrieve", LlmCandidate)
    }

    async fn approve_candidate(
        &self,
        _context: LlmAppRequestContext,
        _candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        app_not_implemented!("candidates.approve", LlmCandidate)
    }

    async fn reject_candidate(
        &self,
        _context: LlmAppRequestContext,
        _candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        app_not_implemented!("candidates.reject", LlmCandidate)
    }

    async fn list_habits(
        &self,
        _context: LlmAppRequestContext,
        _query: ListHabitsQuery,
    ) -> LlmServiceResult<LlmHabitList> {
        app_not_implemented!("habits.list", LlmHabitList)
    }

    async fn retrieve_habit(
        &self,
        _context: LlmAppRequestContext,
        _habit_id: u64,
    ) -> LlmServiceResult<LlmHabit> {
        app_not_implemented!("habits.retrieve", LlmHabit)
    }

    async fn update_habit(
        &self,
        _context: LlmAppRequestContext,
        _habit_id: u64,
        _request: LlmHabitRequest,
    ) -> LlmServiceResult<LlmHabit> {
        app_not_implemented!("habits.update", LlmHabit)
    }

    async fn confirm_habit(
        &self,
        _context: LlmAppRequestContext,
        _habit_id: u64,
        _request: LlmReviewRequest,
    ) -> LlmServiceResult<LlmHabit> {
        app_not_implemented!("habits.confirm", LlmHabit)
    }

    async fn reject_habit(
        &self,
        _context: LlmAppRequestContext,
        _habit_id: u64,
        _request: LlmReviewRequest,
    ) -> LlmServiceResult<LlmHabit> {
        app_not_implemented!("habits.reject", LlmHabit)
    }

    async fn create_retrieval(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmRetrievalRequest,
    ) -> LlmServiceResult<LlmRetrievalResult> {
        app_not_implemented!("retrievals.create", LlmRetrievalResult)
    }

    async fn retrieve_retrieval(
        &self,
        _context: LlmAppRequestContext,
        _retrieval_id: u64,
    ) -> LlmServiceResult<LlmRetrievalResult> {
        app_not_implemented!("retrievals.retrieve", LlmRetrievalResult)
    }

    async fn create_context_pack(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmContextPackRequest,
    ) -> LlmServiceResult<LlmContextPack> {
        app_not_implemented!("contextPacks.create", LlmContextPack)
    }

    async fn retrieve_context_pack(
        &self,
        _context: LlmAppRequestContext,
        _context_pack_id: u64,
    ) -> LlmServiceResult<LlmContextPack> {
        app_not_implemented!("contextPacks.retrieve", LlmContextPack)
    }

    async fn create_feedback(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmFeedbackRequest,
    ) -> LlmServiceResult<MemoryFeedback> {
        app_not_implemented!("feedback.create", MemoryFeedback)
    }

    async fn create_export_job(
        &self,
        _context: LlmAppRequestContext,
        _request: LlmExportRequest,
    ) -> LlmServiceResult<LlmExportJob> {
        app_not_implemented!("exportJobs.create", LlmExportJob)
    }

    async fn retrieve_export_job(
        &self,
        _context: LlmAppRequestContext,
        _export_job_id: u64,
    ) -> LlmServiceResult<LlmExportJob> {
        app_not_implemented!("exportJobs.retrieve", LlmExportJob)
    }

    async fn retrieve_learning_settings(
        &self,
        _context: LlmAppRequestContext,
    ) -> LlmServiceResult<LlmLearningSettings> {
        app_not_implemented!("learningSettings.retrieve", LlmLearningSettings)
    }

    async fn update_learning_settings(
        &self,
        _context: LlmAppRequestContext,
        _patch: LlmLearningSettingsPatch,
    ) -> LlmServiceResult<LlmLearningSettings> {
        app_not_implemented!("learningSettings.update", LlmLearningSettings)
    }
}
