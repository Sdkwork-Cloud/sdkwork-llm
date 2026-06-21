use async_trait::async_trait;
use sdkwork_llm_contract::ListSpacesQuery;
use sdkwork_llm_contract::{
    ListAuditLogsQuery, ListLlmCandidatesQuery, ListEventsQuery, ListHabitsQuery, ListLlmRecordsQuery,
    ListRetrievalTracesQuery, LlmAppApi, LlmAppRequestContext, LlmBackendApi,
    LlmBackendRequestContext, LlmCandidate, LlmCandidateList, LlmEventList,
    LlmExportJob, LlmExportRequest, LlmExtractionRequest, LlmForgetJob,
    LlmForgetRequest, LlmHabit, LlmHabitList, LlmHabitRequest, LlmLearningJob,
    LlmLearningSettings, LlmLearningSettingsPatch, LlmOpenApi, LlmPageInfo,
    LlmProviderHealth, LlmRecordList, LlmRecordSource, LlmRecordSourceList,
    LlmRetrievalTrace, LlmRetrievalTraceList, LlmReviewRequest, LlmServiceError,
    LlmServiceResult, LlmSpace, LlmSpaceList, LlmSpaceRequest,
};
use sdkwork_llm_plugin_native_sql::{
    NativeSqlAuditLogRow, NativeSqlCandidateRow, NativeSqlCreateSpaceCommand, NativeSqlHabitRow,
    NativeSqlLlmSpaceRow, NativeSqlRecordSourceRow, NativeSqlRetrievalTraceSummaryRow,
};
use sdkwork_llm_spi::{
    DecayLlmHabitCommand, LlmScopeContext, PromoteLlmHabitCommand,
    RejectLlmCandidateCommand, UpsertLlmHabitCommand,
};

use crate::open_api::OpenLlmService;
use crate::platform;

impl OpenLlmService {
    pub(crate) fn map_space(row: NativeSqlLlmSpaceRow) -> LlmServiceResult<LlmSpace> {
        Ok(LlmSpace {
            space_id: u64::try_from(row.space_id.max(0))
                .map_err(|_| LlmServiceError::storage("space id must be non-negative"))?,
            uuid: Some(row.uuid),
            tenant_id: u64::try_from(row.tenant_id.max(0)).unwrap_or(0),
            organization_id: None,
            owner_subject_type: row.owner_subject_type,
            owner_subject_id: row.owner_subject_id,
            space_type: row.space_type,
            display_name: row.display_name,
            default_scope: row.default_scope,
            lifecycle_status: row.lifecycle_status,
            metadata: None,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: u64::try_from(row.version.max(0)).unwrap_or(0),
        })
    }

    pub(crate) fn map_candidate(
        row: NativeSqlCandidateRow,
    ) -> LlmServiceResult<LlmCandidate> {
        Ok(LlmCandidate {
            candidate_id: row.candidate_id.parse().unwrap_or(0),
            space_id: u64::try_from(row.space_id.max(0)).unwrap_or(0),
            candidate_type: row.candidate_type,
            record_type: OpenLlmService::record_type_from_db(&row.record_type),
            proposed_text: row.proposed_text,
            confidence: row.confidence,
            decision_state: row.decision_state,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub(crate) fn map_trace_summary(
        row: NativeSqlRetrievalTraceSummaryRow,
    ) -> LlmServiceResult<LlmRetrievalTrace> {
        Ok(LlmRetrievalTrace {
            trace_id: row.trace_id.parse().unwrap_or(0),
            space_id: Some(u64::try_from(row.space_id.max(0)).unwrap_or(0)),
            retrieval_profile_id: None,
            actor_id: None,
            query_text: row.query_text,
            query_hash: row.query_hash,
            result_count: row.result_count as i32,
            degraded: row.degraded,
            created_at: row.created_at,
        })
    }

    fn normalize_habit_stage(stage: &str) -> String {
        match stage {
            "promoted" => "confirmed".to_string(),
            "decayed" => "decaying".to_string(),
            "candidate" => "emerging".to_string(),
            other => other.to_string(),
        }
    }

    fn map_habit(row: NativeSqlHabitRow) -> LlmServiceResult<LlmHabit> {
        Ok(LlmHabit {
            habit_id: row.habit_id.parse().unwrap_or(0),
            space_id: u64::try_from(row.space_id.max(0)).unwrap_or(0),
            user_id: u64::try_from(row.user_id.max(0)).unwrap_or(0),
            habit_key: row.habit_key,
            habit_type: row.habit_type,
            description: row.description,
            stage: Self::normalize_habit_stage(&row.stage),
            strength: row.strength,
            confidence: row.confidence,
            support_count: row.support_count as i32,
            last_signal_at: row.last_signal_at,
            promoted_record_id: row
                .promoted_memory_uuid
                .and_then(|value| value.parse().ok()),
            decay_after: row.decay_after,
            metadata: row
                .metadata_json
                .as_deref()
                .and_then(|value| serde_json::from_str(value).ok()),
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: u64::try_from(row.version.max(0)).unwrap_or(0),
        })
    }

    pub(crate) fn map_record_source(
        row: NativeSqlRecordSourceRow,
    ) -> LlmServiceResult<LlmRecordSource> {
        let source_id = OpenLlmService::parse_id(&row.source_uuid)
            .ok_or_else(|| LlmServiceError::storage("source id must be numeric"))?;
        let record_id = OpenLlmService::parse_id(&row.memory_uuid)
            .ok_or_else(|| LlmServiceError::storage("llm record id must be numeric"))?;
        let event_id = OpenLlmService::parse_id(&row.event_uuid)
            .ok_or_else(|| LlmServiceError::storage("event id must be numeric"))?;

        Ok(LlmRecordSource {
            source_id,
            record_id,
            event_id,
            source_role: row.source_role,
            confidence_delta: row.confidence_delta,
            created_at: row.created_at,
        })
    }

    async fn load_habit_row(
        &self,
        tenant_id: i64,
        habit_id: u64,
    ) -> LlmServiceResult<NativeSqlHabitRow> {
        self.store
            .retrieve_habit_for_tenant(tenant_id, &habit_id.to_string())
            .await
            .map_err(OpenLlmService::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("habit not found"))
    }

    pub(crate) fn governance_scope(tenant_id: i64) -> LlmScopeContext {
        LlmScopeContext {
            tenant_id,
            space_id: 1,
            organization_id: None,
            user_id: None,
        }
    }

    pub(crate) async fn persist_governance_job<T: serde::Serialize>(
        &self,
        tenant_id: i64,
        job_id: u64,
        resource_type: &str,
        action: &str,
        job: &T,
    ) -> LlmServiceResult<()> {
        let metadata = serde_json::to_string(job).map_err(|error| {
            LlmServiceError::storage(format!("governance job metadata encode failed: {error}"))
        })?;
        self.store
            .append_audit_with_metadata(
                &Self::governance_scope(tenant_id),
                &job_id.to_string(),
                action,
                resource_type,
                &job_id.to_string(),
                "accepted",
                &metadata,
            )
            .await
            .map_err(OpenLlmService::map_store_error)?;
        Ok(())
    }

    pub(crate) async fn load_governance_job<T: serde::de::DeserializeOwned>(
        &self,
        tenant_id: i64,
        job_id: u64,
        resource_type: &str,
    ) -> LlmServiceResult<T> {
        let row = self
            .store
            .retrieve_governance_job_for_tenant(tenant_id, &job_id.to_string(), resource_type)
            .await
            .map_err(OpenLlmService::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("governance job not found"))?;
        let metadata = row
            .metadata_json
            .ok_or_else(|| LlmServiceError::storage("governance job metadata is missing"))?;
        serde_json::from_str(&metadata).map_err(|error| {
            LlmServiceError::storage(format!("governance job metadata decode failed: {error}"))
        })
    }
}

#[async_trait]
impl LlmAppApi for OpenLlmService {
    async fn list_spaces(
        &self,
        context: LlmAppRequestContext,
        query: ListSpacesQuery,
    ) -> LlmServiceResult<LlmSpaceList> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_spaces_for_tenant(tenant_id, page_size)
            .await
            .map_err(OpenLlmService::map_store_error)?;
        let items = rows
            .into_iter()
            .map(Self::map_space)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LlmSpaceList {
            items,
            page_info: LlmPageInfo {
                next_cursor: None,
                has_more: false,
                page_size: Some(page_size),
            },
        })
    }

    async fn create_space(
        &self,
        context: LlmAppRequestContext,
        request: LlmSpaceRequest,
    ) -> LlmServiceResult<LlmSpace> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let space_id = i64::try_from(self.next_id()?)
            .map_err(|_| LlmServiceError::storage("generated space id out of range"))?;
        self.store
            .create_space_record(
                tenant_id,
                space_id,
                &NativeSqlCreateSpaceCommand {
                    organization_id: context.organization_id.map(|value| value as i64),
                    owner_subject_type: request.owner_subject_type,
                    owner_subject_id: request.owner_subject_id,
                    space_type: request.space_type,
                    display_name: request.display_name,
                    default_scope: request.default_scope.unwrap_or_else(|| "user".to_string()),
                },
            )
            .await
            .map_err(OpenLlmService::map_store_error)?;
        match self
            .store
            .retrieve_space_for_tenant(tenant_id, space_id)
            .await
            .map_err(OpenLlmService::map_store_error)?
        {
            Some(row) => Self::map_space(row),
            None => Err(LlmServiceError::storage("space not found")),
        }
    }

    async fn retrieve_space(
        &self,
        context: LlmAppRequestContext,
        space_id: u64,
    ) -> LlmServiceResult<LlmSpace> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .retrieve_space_for_tenant(tenant_id, space_id as i64)
            .await
            .map_err(OpenLlmService::map_store_error)?
        {
            Some(row) => Self::map_space(row),
            None => Err(LlmServiceError::not_found("space not found")),
        }
    }

    async fn update_space(
        &self,
        context: LlmAppRequestContext,
        space_id: u64,
        request: LlmSpaceRequest,
    ) -> LlmServiceResult<LlmSpace> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .update_space_record(
                tenant_id,
                space_id as i64,
                Some(&request.display_name),
                request.default_scope.as_deref(),
            )
            .await
            .map_err(OpenLlmService::map_store_error)?
        {
            Some(row) => Self::map_space(row),
            None => Err(LlmServiceError::not_found("space not found")),
        }
    }

    async fn create_event(
        &self,
        context: LlmAppRequestContext,
        request: sdkwork_llm_contract::LlmEventRequest,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmEvent> {
        LlmOpenApi::create_event(self, Self::to_open_context(&context), request).await
    }

    async fn retrieve_event(
        &self,
        context: LlmAppRequestContext,
        event_id: u64,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmEvent> {
        LlmOpenApi::retrieve_event(self, Self::to_open_context(&context), event_id).await
    }

    async fn list_memories(
        &self,
        context: LlmAppRequestContext,
        query: ListLlmRecordsQuery,
    ) -> LlmServiceResult<LlmRecordList> {
        LlmOpenApi::list_memories(self, Self::to_open_context(&context), query).await
    }

    async fn create_memory(
        &self,
        context: LlmAppRequestContext,
        request: sdkwork_llm_contract::LlmRecordRequest,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRecord> {
        LlmOpenApi::create_memory(self, Self::to_open_context(&context), request).await
    }

    async fn retrieve_memory(
        &self,
        context: LlmAppRequestContext,
        record_id: u64,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRecord> {
        LlmOpenApi::retrieve_memory(self, Self::to_open_context(&context), record_id).await
    }

    async fn update_memory(
        &self,
        context: LlmAppRequestContext,
        record_id: u64,
        patch: sdkwork_llm_contract::LlmRecordPatch,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRecord> {
        LlmOpenApi::update_memory(self, Self::to_open_context(&context), record_id, patch).await
    }

    async fn delete_memory(
        &self,
        context: LlmAppRequestContext,
        record_id: u64,
    ) -> LlmServiceResult<()> {
        LlmOpenApi::delete_memory(self, Self::to_open_context(&context), record_id).await
    }

    async fn list_memory_sources(
        &self,
        context: LlmAppRequestContext,
        record_id: u64,
    ) -> LlmServiceResult<LlmRecordSourceList> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let memory_uuid = record_id.to_string();
        self.store
            .retrieve_record_detail_for_tenant(tenant_id, &memory_uuid)
            .await
            .map_err(OpenLlmService::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("memory not found"))?;

        let page_size = 50_i32;
        let rows = self
            .store
            .list_record_sources_for_memory(tenant_id, &memory_uuid, page_size)
            .await
            .map_err(OpenLlmService::map_store_error)?;
        let has_more = rows.len() > page_size as usize;
        let items = rows
            .into_iter()
            .take(page_size as usize)
            .map(Self::map_record_source)
            .collect::<Result<Vec<_>, _>>()?;
        let next_cursor = items.last().map(|source| source.source_id.to_string());

        Ok(LlmRecordSourceList {
            items,
            page_info: LlmPageInfo {
                next_cursor: if has_more { next_cursor } else { None },
                has_more,
                page_size: Some(page_size),
            },
        })
    }

    async fn create_forget_request(
        &self,
        context: LlmAppRequestContext,
        request: LlmForgetRequest,
    ) -> LlmServiceResult<LlmForgetJob> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let job_id = self.next_id()?;
        let mut deleted_count = 0_u32;

        match request.scope.as_str() {
            "memory" => {
                let record_ids = request.record_ids.as_ref().ok_or_else(|| {
                    LlmServiceError::validation("recordIds is required when scope is memory")
                })?;
                for record_id in record_ids {
                    if LlmAppApi::delete_memory(self, context.clone(), *record_id)
                        .await
                        .is_ok()
                    {
                        deleted_count += 1;
                    }
                }
            }
            "space" => {
                let space_id = request.space_id.ok_or_else(|| {
                    LlmServiceError::validation("spaceId is required when scope is space")
                })?;
                let scope = LlmScopeContext {
                    tenant_id,
                    space_id: space_id as i64,
                    organization_id: context.organization_id.map(|value| value as i64),
                    user_id: context.actor_id.map(|value| value as i64),
                };
                let rows = self
                    .store
                    .list_record_details(&scope, None, 100, None)
                    .await
                    .map_err(OpenLlmService::map_store_error)?;
                for row in rows {
                    if self
                        .store
                        .mark_record_deleted(&scope, &row.record_id)
                        .await
                        .map_err(OpenLlmService::map_store_error)
                        .is_ok()
                    {
                        deleted_count += 1;
                    }
                }
            }
            "user" | "query" => {}
            _ => {
                return Err(LlmServiceError::validation(
                    "scope must be one of memory, space, user, or query",
                ));
            }
        }

        let job = LlmForgetJob {
            forget_request_id: job_id,
            state: "succeeded".to_string(),
            result: Some(serde_json::json!({
                "deletedCount": deleted_count,
                "scope": request.scope,
                "reason": request.reason,
            })),
            created_at: platform::current_timestamp(),
            updated_at: platform::current_timestamp(),
        };
        self.persist_governance_job(
            tenant_id,
            job_id,
            "forget_job",
            "forget.request.create",
            &job,
        )
        .await?;
        Ok(job)
    }

    async fn retrieve_forget_request(
        &self,
        context: LlmAppRequestContext,
        forget_request_id: u64,
    ) -> LlmServiceResult<LlmForgetJob> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        Self::load_governance_job(self, tenant_id, forget_request_id, "forget_job").await
    }

    async fn create_export_job(
        &self,
        context: LlmAppRequestContext,
        request: LlmExportRequest,
    ) -> LlmServiceResult<LlmExportJob> {
        if request.space_ids.is_empty() {
            return Err(LlmServiceError::validation("spaceIds must not be empty"));
        }
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let job_id = self.next_id()?;
        let mut exported_records = 0_u32;
        let mut exported_events = 0_u32;

        for space_id in &request.space_ids {
            let scope = LlmScopeContext {
                tenant_id,
                space_id: *space_id as i64,
                organization_id: context.organization_id.map(|value| value as i64),
                user_id: context.actor_id.map(|value| value as i64),
            };
            exported_records += self
                .store
                .list_record_details(&scope, None, 100, None)
                .await
                .map_err(OpenLlmService::map_store_error)?
                .len() as u32;
            if request.include_events.unwrap_or(false) {
                exported_events += self
                    .store
                    .list_open_api_events_for_tenant(tenant_id, Some(*space_id as i64), 100)
                    .await
                    .map_err(OpenLlmService::map_store_error)?
                    .len() as u32;
            }
        }

        let job = LlmExportJob {
            export_job_id: job_id,
            state: "succeeded".to_string(),
            format: request.format.clone(),
            drive_object_ref: request.drive_target_ref.clone(),
            result: Some(serde_json::json!({
                "exportedRecords": exported_records,
                "exportedEvents": exported_events,
                "spaceIds": request.space_ids,
            })),
            created_at: platform::current_timestamp(),
            updated_at: platform::current_timestamp(),
        };
        self.persist_governance_job(tenant_id, job_id, "export_job", "export.job.create", &job)
            .await?;
        Ok(job)
    }

    async fn retrieve_export_job(
        &self,
        context: LlmAppRequestContext,
        export_job_id: u64,
    ) -> LlmServiceResult<LlmExportJob> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        Self::load_governance_job(self, tenant_id, export_job_id, "export_job").await
    }

    async fn list_candidates(
        &self,
        context: LlmAppRequestContext,
        query: ListLlmCandidatesQuery,
    ) -> LlmServiceResult<LlmCandidateList> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_candidates_for_tenant(
                tenant_id,
                query.space_id.map(|value| value as i64),
                page_size,
            )
            .await
            .map_err(OpenLlmService::map_store_error)?;
        let items = rows
            .into_iter()
            .map(Self::map_candidate)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LlmCandidateList {
            items,
            page_info: LlmPageInfo {
                next_cursor: None,
                has_more: false,
                page_size: Some(page_size),
            },
        })
    }

    async fn retrieve_candidate(
        &self,
        context: LlmAppRequestContext,
        candidate_id: u64,
    ) -> LlmServiceResult<LlmCandidate> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .retrieve_candidate_for_tenant(tenant_id, &candidate_id.to_string())
            .await
            .map_err(OpenLlmService::map_store_error)?
        {
            Some(row) => Self::map_candidate(row),
            None => Err(LlmServiceError::not_found("candidate not found")),
        }
    }

    async fn create_retrieval(
        &self,
        context: LlmAppRequestContext,
        request: sdkwork_llm_contract::LlmRetrievalRequest,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRetrievalResult> {
        LlmOpenApi::create_retrieval(self, Self::to_open_context(&context), request).await
    }

    async fn retrieve_retrieval(
        &self,
        context: LlmAppRequestContext,
        retrieval_id: u64,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRetrievalResult> {
        LlmOpenApi::retrieve_retrieval(self, Self::to_open_context(&context), retrieval_id).await
    }

    async fn create_extraction(
        &self,
        context: LlmAppRequestContext,
        request: LlmExtractionRequest,
    ) -> LlmServiceResult<LlmLearningJob> {
        LlmOpenApi::create_extraction(self, Self::to_open_context(&context), request).await
    }

    async fn approve_candidate(
        &self,
        context: LlmAppRequestContext,
        candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        LlmBackendApi::approve_candidate(
            self,
            LlmBackendRequestContext {
                tenant_id: context.tenant_id,
                operator_id: context.actor_id,
            },
            candidate_id,
            serde_json::Value::Null,
        )
        .await
    }

    async fn reject_candidate(
        &self,
        context: LlmAppRequestContext,
        candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        LlmBackendApi::reject_candidate(
            self,
            LlmBackendRequestContext {
                tenant_id: context.tenant_id,
                operator_id: context.actor_id,
            },
            candidate_id,
            serde_json::Value::Null,
        )
        .await
    }

    async fn create_context_pack(
        &self,
        context: LlmAppRequestContext,
        request: sdkwork_llm_contract::LlmContextPackRequest,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmContextPack> {
        LlmOpenApi::create_context_pack(self, Self::to_open_context(&context), request).await
    }

    async fn retrieve_context_pack(
        &self,
        context: LlmAppRequestContext,
        context_pack_id: u64,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmContextPack> {
        LlmOpenApi::retrieve_context_pack(self, Self::to_open_context(&context), context_pack_id)
            .await
    }

    async fn create_feedback(
        &self,
        context: LlmAppRequestContext,
        request: sdkwork_llm_contract::LlmFeedbackRequest,
    ) -> LlmServiceResult<sdkwork_llm_contract::MemoryFeedback> {
        LlmOpenApi::create_feedback(self, Self::to_open_context(&context), request).await
    }

    async fn list_habits(
        &self,
        context: LlmAppRequestContext,
        query: ListHabitsQuery,
    ) -> LlmServiceResult<LlmHabitList> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_habits_for_tenant(
                tenant_id,
                query.space_id.map(|value| value as i64),
                query.stage.as_deref(),
                query.q.as_deref(),
                page_size,
            )
            .await
            .map_err(OpenLlmService::map_store_error)?;
        let items = rows
            .into_iter()
            .map(Self::map_habit)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LlmHabitList {
            items,
            page_info: LlmPageInfo {
                next_cursor: None,
                has_more: false,
                page_size: Some(page_size),
            },
        })
    }

    async fn retrieve_habit(
        &self,
        context: LlmAppRequestContext,
        habit_id: u64,
    ) -> LlmServiceResult<LlmHabit> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .retrieve_habit_for_tenant(tenant_id, &habit_id.to_string())
            .await
            .map_err(OpenLlmService::map_store_error)?
        {
            Some(row) => Self::map_habit(row),
            None => Err(LlmServiceError::not_found("habit not found")),
        }
    }

    async fn update_habit(
        &self,
        context: LlmAppRequestContext,
        habit_id: u64,
        request: LlmHabitRequest,
    ) -> LlmServiceResult<LlmHabit> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self.load_habit_row(tenant_id, habit_id).await?;
        let scope = LlmScopeContext {
            tenant_id,
            space_id: existing.space_id,
            organization_id: context.organization_id.map(|value| value as i64),
            user_id: context.actor_id.map(|value| value as i64),
        };
        let user_id = existing.user_id;
        self.store
            .upsert_habit(&UpsertLlmHabitCommand {
                scope,
                habit_id: existing.habit_id.clone(),
                user_id,
                habit_key: existing.habit_key.clone(),
                habit_type: existing.habit_type.clone(),
                description: request.description.unwrap_or(existing.description),
                stage: request.stage.unwrap_or(existing.stage),
                strength: existing.strength,
                confidence: existing.confidence,
                support_count: existing.support_count,
                metadata_json: request
                    .metadata
                    .map(|value| value.to_string())
                    .or(existing.metadata_json),
            })
            .await
            .map_err(OpenLlmService::map_store_error)?;
        self.retrieve_habit(context, habit_id).await
    }

    async fn confirm_habit(
        &self,
        context: LlmAppRequestContext,
        habit_id: u64,
        _request: LlmReviewRequest,
    ) -> LlmServiceResult<LlmHabit> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self.load_habit_row(tenant_id, habit_id).await?;
        let scope = LlmScopeContext {
            tenant_id,
            space_id: existing.space_id,
            organization_id: context.organization_id.map(|value| value as i64),
            user_id: context.actor_id.map(|value| value as i64),
        };
        self.store
            .promote_habit(&PromoteLlmHabitCommand {
                scope,
                user_id: existing.user_id,
                habit_key: existing.habit_key.clone(),
                promoted_record_id: None,
            })
            .await
            .map_err(OpenLlmService::map_store_error)?;
        self.retrieve_habit(context, habit_id).await
    }

    async fn reject_habit(
        &self,
        context: LlmAppRequestContext,
        habit_id: u64,
        _request: LlmReviewRequest,
    ) -> LlmServiceResult<LlmHabit> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self.load_habit_row(tenant_id, habit_id).await?;
        let scope = LlmScopeContext {
            tenant_id,
            space_id: existing.space_id,
            organization_id: context.organization_id.map(|value| value as i64),
            user_id: context.actor_id.map(|value| value as i64),
        };
        self.store
            .decay_habit(&DecayLlmHabitCommand {
                scope,
                user_id: existing.user_id,
                habit_key: existing.habit_key.clone(),
                strength_delta: existing.strength.max(0.1),
            })
            .await
            .map_err(OpenLlmService::map_store_error)?;
        self.retrieve_habit(context, habit_id).await
    }

    async fn retrieve_learning_settings(
        &self,
        _context: LlmAppRequestContext,
    ) -> LlmServiceResult<LlmLearningSettings> {
        Ok(LlmLearningSettings {
            auto_promote_candidates: false,
            habit_learning_enabled: true,
            updated_at: platform::current_timestamp(),
        })
    }

    async fn update_learning_settings(
        &self,
        _context: LlmAppRequestContext,
        patch: LlmLearningSettingsPatch,
    ) -> LlmServiceResult<LlmLearningSettings> {
        Ok(LlmLearningSettings {
            auto_promote_candidates: patch.auto_promote_candidates.unwrap_or(false),
            habit_learning_enabled: patch.habit_learning_enabled.unwrap_or(true),
            updated_at: platform::current_timestamp(),
        })
    }
}

#[async_trait]
impl LlmBackendApi for OpenLlmService {
    async fn list_spaces(
        &self,
        context: LlmBackendRequestContext,
        query: ListSpacesQuery,
    ) -> LlmServiceResult<LlmSpaceList> {
        LlmAppApi::list_spaces(
            self,
            LlmAppRequestContext {
                tenant_id: context.tenant_id,
                actor_id: context.operator_id,
                organization_id: None,
                session_id: None,
            },
            query,
        )
        .await
    }

    async fn retrieve_space(
        &self,
        context: LlmBackendRequestContext,
        space_id: u64,
    ) -> LlmServiceResult<LlmSpace> {
        LlmAppApi::retrieve_space(
            self,
            LlmAppRequestContext {
                tenant_id: context.tenant_id,
                actor_id: context.operator_id,
                organization_id: None,
                session_id: None,
            },
            space_id,
        )
        .await
    }

    async fn update_space(
        &self,
        context: LlmBackendRequestContext,
        space_id: u64,
        request: LlmSpaceRequest,
    ) -> LlmServiceResult<LlmSpace> {
        LlmAppApi::update_space(
            self,
            LlmAppRequestContext {
                tenant_id: context.tenant_id,
                actor_id: context.operator_id,
                organization_id: None,
                session_id: None,
            },
            space_id,
            request,
        )
        .await
    }

    async fn list_memories(
        &self,
        context: LlmBackendRequestContext,
        query: ListLlmRecordsQuery,
    ) -> LlmServiceResult<LlmRecordList> {
        LlmOpenApi::list_memories(self, Self::to_open_context_backend(&context), query).await
    }

    async fn retrieve_memory(
        &self,
        context: LlmBackendRequestContext,
        record_id: u64,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRecord> {
        LlmOpenApi::retrieve_memory(self, Self::to_open_context_backend(&context), record_id)
            .await
    }

    async fn update_memory(
        &self,
        context: LlmBackendRequestContext,
        record_id: u64,
        patch: sdkwork_llm_contract::LlmRecordPatch,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRecord> {
        LlmOpenApi::update_memory(
            self,
            Self::to_open_context_backend(&context),
            record_id,
            patch,
        )
        .await
    }

    async fn list_events(
        &self,
        context: LlmBackendRequestContext,
        query: ListEventsQuery,
    ) -> LlmServiceResult<LlmEventList> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_open_api_events_for_tenant(
                tenant_id,
                query.space_id.map(|value| value as i64),
                page_size,
            )
            .await
            .map_err(OpenLlmService::map_store_error)?;
        let items = rows
            .into_iter()
            .map(OpenLlmService::map_event)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LlmEventList {
            items,
            page_info: LlmPageInfo {
                next_cursor: None,
                has_more: false,
                page_size: Some(page_size),
            },
        })
    }

    async fn retrieve_event(
        &self,
        context: LlmBackendRequestContext,
        event_id: u64,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmEvent> {
        LlmOpenApi::retrieve_event(self, Self::to_open_context_backend(&context), event_id).await
    }

    async fn list_candidates(
        &self,
        context: LlmBackendRequestContext,
        query: ListLlmCandidatesQuery,
    ) -> LlmServiceResult<LlmCandidateList> {
        LlmAppApi::list_candidates(
            self,
            LlmAppRequestContext {
                tenant_id: context.tenant_id,
                actor_id: context.operator_id,
                organization_id: None,
                session_id: None,
            },
            query,
        )
        .await
    }

    async fn approve_candidate(
        &self,
        context: LlmBackendRequestContext,
        candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self
            .store
            .retrieve_candidate_detail_for_tenant(tenant_id, &candidate_id.to_string())
            .await
            .map_err(OpenLlmService::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("candidate not found"))?;
        let scope = LlmScopeContext {
            tenant_id,
            space_id: existing.space_id,
            organization_id: None,
            user_id: context.operator_id.map(|value| value as i64),
        };
        self.approve_candidate_with_promotion(tenant_id, scope, candidate_id, context.operator_id)
            .await
    }

    async fn reject_candidate(
        &self,
        context: LlmBackendRequestContext,
        candidate_id: u64,
        _request: serde_json::Value,
    ) -> LlmServiceResult<LlmCandidate> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self
            .store
            .retrieve_candidate_for_tenant(tenant_id, &candidate_id.to_string())
            .await
            .map_err(OpenLlmService::map_store_error)?
            .ok_or_else(|| {
                sdkwork_llm_contract::LlmServiceError::not_found("candidate not found")
            })?;
        let scope = LlmScopeContext {
            tenant_id,
            space_id: existing.space_id,
            organization_id: None,
            user_id: context.operator_id.map(|value| value as i64),
        };
        self.store
            .reject_candidate(&RejectLlmCandidateCommand {
                scope,
                candidate_id: candidate_id.to_string(),
                decision_reason: None,
                decided_by: context.operator_id.map(|value| value as i64),
            })
            .await
            .map_err(OpenLlmService::map_store_error)?;
        match self
            .store
            .retrieve_candidate_for_tenant(tenant_id, &candidate_id.to_string())
            .await
            .map_err(OpenLlmService::map_store_error)?
        {
            Some(row) => Self::map_candidate(row),
            None => Err(LlmServiceError::not_found("candidate not found")),
        }
    }

    async fn retrieve_provider_health(
        &self,
        context: LlmBackendRequestContext,
    ) -> LlmServiceResult<LlmProviderHealth> {
        LlmOpenApi::retrieve_provider_health(self, Self::to_open_context_backend(&context)).await
    }

    async fn list_retrieval_traces(
        &self,
        context: LlmBackendRequestContext,
        query: ListRetrievalTracesQuery,
    ) -> LlmServiceResult<LlmRetrievalTraceList> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_retrieval_traces_for_tenant(
                tenant_id,
                query.space_id.map(|value| value as i64),
                page_size,
            )
            .await
            .map_err(OpenLlmService::map_store_error)?;
        let items = rows
            .into_iter()
            .map(Self::map_trace_summary)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LlmRetrievalTraceList {
            items,
            page_info: LlmPageInfo {
                next_cursor: None,
                has_more: false,
                page_size: Some(page_size),
            },
        })
    }

    async fn retrieve_retrieval_trace(
        &self,
        context: LlmBackendRequestContext,
        trace_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        let result = LlmOpenApi::retrieve_retrieval(
            self,
            Self::to_open_context_backend(&context),
            trace_id,
        )
        .await?;
        Ok(serde_json::to_value(result).unwrap_or(serde_json::Value::Null))
    }

    async fn list_audit_logs(
        &self,
        context: LlmBackendRequestContext,
        query: ListAuditLogsQuery,
    ) -> LlmServiceResult<serde_json::Value> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_audit_logs_for_tenant(tenant_id, query.action.as_deref(), page_size)
            .await
            .map_err(OpenLlmService::map_store_error)?;
        Ok(serde_json::json!({
            "items": rows.into_iter().map(audit_log_to_json).collect::<Vec<_>>(),
            "pageInfo": {
                "nextCursor": null,
                "hasMore": false,
                "pageSize": page_size,
            }
        }))
    }

    async fn supersede_memory(
        &self,
        context: LlmBackendRequestContext,
        record_id: u64,
        request: serde_json::Value,
    ) -> LlmServiceResult<sdkwork_llm_contract::LlmRecord> {
        self.backend_supersede_memory(context, record_id, request)
            .await
    }

    async fn create_extraction_job(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_extraction_job(context, request).await
    }

    async fn retrieve_extraction_job(
        &self,
        context: LlmBackendRequestContext,
        job_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_retrieve_extraction_job(context, job_id).await
    }

    async fn create_consolidation_job(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_consolidation_job(context, request)
            .await
    }

    async fn list_indexes(
        &self,
        context: LlmBackendRequestContext,
        query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_list_indexes(context, query).await
    }

    async fn create_index(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_index(context, request).await
    }

    async fn retrieve_index(
        &self,
        context: LlmBackendRequestContext,
        index_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_retrieve_index(context, index_id).await
    }

    async fn update_index(
        &self,
        context: LlmBackendRequestContext,
        index_id: u64,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_update_index(context, index_id, request).await
    }

    async fn rebuild_index(
        &self,
        context: LlmBackendRequestContext,
        index_id: u64,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_rebuild_index(context, index_id, request).await
    }

    async fn list_retrieval_profiles(
        &self,
        context: LlmBackendRequestContext,
        query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_list_retrieval_profiles(context, query).await
    }

    async fn create_retrieval_profile(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_retrieval_profile(context, request)
            .await
    }

    async fn retrieve_retrieval_profile(
        &self,
        context: LlmBackendRequestContext,
        profile_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_retrieve_retrieval_profile(context, profile_id)
            .await
    }

    async fn update_retrieval_profile(
        &self,
        context: LlmBackendRequestContext,
        profile_id: u64,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_update_retrieval_profile(context, profile_id, request)
            .await
    }

    async fn list_implementation_profiles(
        &self,
        context: LlmBackendRequestContext,
        query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_list_implementation_profiles(context, query)
            .await
    }

    async fn create_implementation_profile(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_implementation_profile(context, request)
            .await
    }

    async fn retrieve_implementation_profile(
        &self,
        context: LlmBackendRequestContext,
        profile_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_retrieve_implementation_profile(context, profile_id)
            .await
    }

    async fn update_implementation_profile(
        &self,
        context: LlmBackendRequestContext,
        profile_id: u64,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_update_implementation_profile(context, profile_id, request)
            .await
    }

    async fn list_provider_bindings(
        &self,
        context: LlmBackendRequestContext,
        query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_list_provider_bindings(context, query).await
    }

    async fn create_provider_binding(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_provider_binding(context, request).await
    }

    async fn update_provider_binding(
        &self,
        context: LlmBackendRequestContext,
        binding_id: u64,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_update_provider_binding(context, binding_id, request)
            .await
    }

    async fn list_eval_runs(
        &self,
        context: LlmBackendRequestContext,
        query: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_list_eval_runs(context, query).await
    }

    async fn create_eval_run(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_eval_run(context, request).await
    }

    async fn retrieve_eval_run(
        &self,
        context: LlmBackendRequestContext,
        eval_run_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_retrieve_eval_run(context, eval_run_id).await
    }

    async fn create_retention_job(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_retention_job(context, request).await
    }

    async fn create_migration_job(
        &self,
        context: LlmBackendRequestContext,
        request: serde_json::Value,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_create_migration_job(context, request).await
    }

    async fn retrieve_migration_job(
        &self,
        context: LlmBackendRequestContext,
        migration_job_id: u64,
    ) -> LlmServiceResult<serde_json::Value> {
        self.backend_retrieve_migration_job(context, migration_job_id)
            .await
    }
}

fn audit_log_to_json(row: NativeSqlAuditLogRow) -> serde_json::Value {
    serde_json::json!({
        "auditLogId": row.audit_id,
        "action": row.action,
        "resourceType": row.resource_type,
        "resourceId": row.resource_id,
        "result": row.result,
        "createdAt": row.created_at,
    })
}
