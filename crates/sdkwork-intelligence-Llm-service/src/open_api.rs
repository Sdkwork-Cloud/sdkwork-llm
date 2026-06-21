use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_llm_contract::{
    ListLlmCandidatesQuery, ListLlmRecordsQuery, LlmAppRequestContext, LlmBackendRequestContext,
    LlmCandidate, LlmCandidateList, LlmCapabilities, LlmContextPack,
    LlmContextPackRequest, LlmEvent, LlmEventRequest, LlmExtractionRequest,
    MemoryFeedback, LlmFeedbackRequest, LlmImplementationKind, LlmLearningJob,
    LlmOpenApi, LlmOpenApiRequestContext, LlmPageInfo, LlmProviderHealth,
    LlmProviderHealthStatus, LlmProviderInterface, LlmRecord, LlmRecordList,
    LlmRecordPatch, LlmRecordRequest, LlmRetrievalHit, LlmRetrievalRequest,
    LlmRetrievalResult, LlmRetrievalTrace, LlmRetrieverKind, LlmServiceError,
    LlmServiceResult, LlmRecordType,
};
use sdkwork_llm_core::{
    build_context_pack_from_hits, fuse_retrieval_candidates, keyword_match_score,
    RetrievalCandidate,
};
use sdkwork_llm_plugin_native_sql::{
    NativeSqlLlmRecordDetail, NativeSqlLlmStore, NativeSqlOpenApiEventRow,
};
use sdkwork_llm_spi::{
    AppendLlmRetrievalTraceCommand, CreateLlmCandidateCommand, LlmRetrievalHitDraft,
    LlmScopeContext,
};

use crate::platform;

pub struct OpenLlmService {
    pub(crate) store: Arc<NativeSqlLlmStore>,
}

impl OpenLlmService {
    pub fn new(store: NativeSqlLlmStore) -> Self {
        Self {
            store: Arc::new(store),
        }
    }

    pub(crate) fn to_open_context(app: &LlmAppRequestContext) -> LlmOpenApiRequestContext {
        LlmOpenApiRequestContext {
            api_key_id: app
                .session_id
                .clone()
                .unwrap_or_else(|| format!("app-{}", app.actor_id.unwrap_or(0))),
            tenant_id: app.tenant_id,
            actor_id: app.actor_id,
        }
    }

    pub(crate) fn to_open_context_backend(
        backend: &LlmBackendRequestContext,
    ) -> LlmOpenApiRequestContext {
        LlmOpenApiRequestContext {
            api_key_id: format!("backend-{}", backend.operator_id.unwrap_or(0)),
            tenant_id: backend.tenant_id,
            actor_id: backend.operator_id,
        }
    }

    pub(crate) fn next_id(&self) -> LlmServiceResult<u64> {
        platform::next_numeric_id()
    }

    fn scope(context: &LlmOpenApiRequestContext, space_id: u64) -> LlmScopeContext {
        LlmScopeContext {
            tenant_id: i64::try_from(context.tenant_id).unwrap_or(i64::MAX),
            space_id: i64::try_from(space_id).unwrap_or(i64::MAX),
            organization_id: None,
            user_id: context.actor_id.map(|value| value as i64),
        }
    }

    pub(crate) fn map_store_error(
        error: sdkwork_llm_plugin_native_sql::NativeSqlStoreError,
    ) -> LlmServiceError {
        if let sdkwork_llm_plugin_native_sql::NativeSqlStoreError::EventConflict { .. } = error {
            return LlmServiceError::conflict(error.to_string());
        }
        LlmServiceError::storage(error.to_string())
    }

    pub(crate) fn parse_id(value: &str) -> Option<u64> {
        platform::parse_numeric_id(value)
    }

    fn record_type_to_db(value: LlmRecordType) -> &'static str {
        match value {
            LlmRecordType::Working => "working",
            LlmRecordType::Session => "session",
            LlmRecordType::Semantic => "semantic",
            LlmRecordType::Episodic => "episodic",
            LlmRecordType::Procedural => "procedural",
            LlmRecordType::Habit => "habit",
            LlmRecordType::Relationship => "relationship",
            LlmRecordType::DomainKnowledge => "domain_knowledge",
        }
    }

    pub(crate) fn record_type_from_db(value: &str) -> LlmRecordType {
        match value {
            "working" => LlmRecordType::Working,
            "session" => LlmRecordType::Session,
            "episodic" => LlmRecordType::Episodic,
            "procedural" => LlmRecordType::Procedural,
            "habit" => LlmRecordType::Habit,
            "relationship" => LlmRecordType::Relationship,
            "domain_knowledge" => LlmRecordType::DomainKnowledge,
            _ => LlmRecordType::Semantic,
        }
    }

    fn map_record(detail: NativeSqlLlmRecordDetail) -> LlmServiceResult<LlmRecord> {
        let record_id = Self::parse_id(&detail.record_id)
            .ok_or_else(|| LlmServiceError::storage("llm record id must be numeric"))?;
        let space_id = u64::try_from(detail.space_id)
            .map_err(|_| LlmServiceError::storage("space id must be non-negative"))?;
        let version = u64::try_from(detail.version.max(0))
            .map_err(|_| LlmServiceError::storage("version must be non-negative"))?;

        Ok(LlmRecord {
            record_id,
            uuid: Some(detail.record_id),
            space_id,
            user_id: None,
            scope: detail.scope,
            record_type: Self::record_type_from_db(&detail.record_type),
            subject: detail.subject,
            predicate: detail.predicate,
            object_text: Some(detail.object_text),
            canonical_text: detail.canonical_text,
            summary_text: None,
            confidence: detail.confidence,
            evidence_count: Some(1),
            contradiction_count: Some(0),
            status: detail.status,
            created_at: detail.created_at,
            updated_at: detail.updated_at,
            version,
        })
    }

    pub(crate) fn map_event(row: NativeSqlOpenApiEventRow) -> LlmServiceResult<LlmEvent> {
        let event_id = Self::parse_id(&row.event_id)
            .ok_or_else(|| LlmServiceError::storage("event id must be numeric"))?;
        let space_id = u64::try_from(row.space_id)
            .map_err(|_| LlmServiceError::storage("space id must be non-negative"))?;

        Ok(LlmEvent {
            event_id,
            uuid: Some(row.event_id),
            space_id,
            user_id: None,
            actor_type: None,
            actor_id: None,
            event_type: row.event_type,
            source_type: row.source_type,
            event_time: row.event_time,
            payload: Some(row.payload),
            payload_hash: row.payload_hash,
            sensitivity_level: None,
            ingestion_status: row.ingestion_status,
            created_at: row.created_at,
        })
    }
}

#[async_trait]
impl LlmOpenApi for OpenLlmService {
    async fn retrieve_capabilities(
        &self,
        _context: LlmOpenApiRequestContext,
    ) -> LlmServiceResult<LlmCapabilities> {
        Ok(LlmCapabilities {
            embedding_optional: true,
            retrievers: vec![
                LlmRetrieverKind::Sql,
                LlmRetrieverKind::Keyword,
                LlmRetrieverKind::Dictionary,
                LlmRetrieverKind::Time,
                LlmRetrieverKind::Event,
            ],
            provider_interfaces: vec![
                LlmProviderInterface::Memory,
                LlmProviderInterface::Search,
            ],
            implementation_kinds: vec![LlmImplementationKind::NativeSql],
            open_api_prefix: "/llm/v3/api".to_string(),
            sdk_family: "sdkwork-llm-sdk".to_string(),
            checked_at: platform::current_timestamp(),
            metadata: None,
        })
    }

    async fn create_event(
        &self,
        context: LlmOpenApiRequestContext,
        request: LlmEventRequest,
    ) -> LlmServiceResult<LlmEvent> {
        let scope = Self::scope(&context, request.space_id);
        let event_id = self.next_id()?.to_string();
        self.store
            .append_open_api_event(
                &scope,
                &event_id,
                &request.event_type,
                &request.source_type,
                &request.event_time,
                &request.payload,
            )
            .await
            .map_err(Self::map_store_error)?;

        self.store
            .retrieve_open_api_event(&scope, &event_id)
            .await
            .map_err(Self::map_store_error)?
            .map(Self::map_event)
            .transpose()?
            .ok_or_else(|| LlmServiceError::storage("created event could not be loaded"))
    }

    async fn retrieve_event(
        &self,
        context: LlmOpenApiRequestContext,
        event_id: u64,
    ) -> LlmServiceResult<LlmEvent> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .retrieve_open_api_event_for_tenant(tenant_id, &event_id.to_string())
            .await
            .map_err(Self::map_store_error)?
        {
            Some(row) => Self::map_event(row),
            None => Err(LlmServiceError::not_found("event not found")),
        }
    }

    async fn list_memories(
        &self,
        context: LlmOpenApiRequestContext,
        query: ListLlmRecordsQuery,
    ) -> LlmServiceResult<LlmRecordList> {
        let space_id = query.space_id.unwrap_or(1);
        let scope = Self::scope(&context, space_id);
        let page_size = query.page_size.unwrap_or(20);
        let rows = self
            .store
            .list_record_details(
                &scope,
                query.q.as_deref(),
                page_size,
                query.cursor.as_deref(),
            )
            .await
            .map_err(Self::map_store_error)?;

        let has_more = rows.len() > page_size as usize;
        let items = rows
            .into_iter()
            .take(page_size as usize)
            .map(Self::map_record)
            .collect::<Result<Vec<_>, _>>()?;
        let next_cursor = items.last().map(|record| record.record_id.to_string());

        Ok(LlmRecordList {
            items,
            page_info: LlmPageInfo {
                next_cursor: if has_more { next_cursor } else { None },
                has_more,
                page_size: Some(page_size),
            },
        })
    }

    async fn create_memory(
        &self,
        context: LlmOpenApiRequestContext,
        request: LlmRecordRequest,
    ) -> LlmServiceResult<LlmRecord> {
        let scope = Self::scope(&context, request.space_id);
        let record_id = self.next_id()?.to_string();
        let object_text = request
            .object_text
            .unwrap_or_else(|| request.canonical_text.clone());

        self.store
            .create_record_open_api(
                &scope,
                &record_id,
                &request.scope,
                Self::record_type_to_db(request.record_type),
                request.subject.as_deref(),
                request.predicate.as_deref(),
                &object_text,
                &request.canonical_text,
            )
            .await
            .map_err(Self::map_store_error)?;

        self.store
            .retrieve_record_detail(&scope, &record_id)
            .await
            .map_err(Self::map_store_error)?
            .map(Self::map_record)
            .transpose()?
            .ok_or_else(|| LlmServiceError::storage("created memory could not be loaded"))
    }

    async fn retrieve_memory(
        &self,
        context: LlmOpenApiRequestContext,
        record_id: u64,
    ) -> LlmServiceResult<LlmRecord> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .retrieve_record_detail_for_tenant(tenant_id, &record_id.to_string())
            .await
            .map_err(Self::map_store_error)?
        {
            Some(row) => Self::map_record(row),
            None => Err(LlmServiceError::not_found("memory not found")),
        }
    }

    async fn update_memory(
        &self,
        context: LlmOpenApiRequestContext,
        record_id: u64,
        patch: LlmRecordPatch,
    ) -> LlmServiceResult<LlmRecord> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self
            .store
            .retrieve_record_detail_for_tenant(tenant_id, &record_id.to_string())
            .await
            .map_err(Self::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("memory not found"))?;
        let scope = Self::scope(&context, u64::try_from(existing.space_id).unwrap_or(1));

        match self
            .store
            .update_record_open_api(
                &scope,
                &record_id.to_string(),
                patch.canonical_text.as_deref(),
                patch.subject.as_deref(),
            )
            .await
            .map_err(Self::map_store_error)?
        {
            Some(row) => Self::map_record(row),
            None => Err(LlmServiceError::not_found("memory not found")),
        }
    }

    async fn delete_memory(
        &self,
        context: LlmOpenApiRequestContext,
        record_id: u64,
    ) -> LlmServiceResult<()> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let existing = self
            .store
            .retrieve_record_detail_for_tenant(tenant_id, &record_id.to_string())
            .await
            .map_err(Self::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("memory not found"))?;
        let scope = Self::scope(&context, u64::try_from(existing.space_id).unwrap_or(1));

        self.store
            .mark_record_deleted(&scope, &record_id.to_string())
            .await
            .map_err(Self::map_store_error)?;
        Ok(())
    }

    async fn create_retrieval(
        &self,
        context: LlmOpenApiRequestContext,
        request: LlmRetrievalRequest,
    ) -> LlmServiceResult<LlmRetrievalResult> {
        if request.space_ids.is_empty() {
            return Err(LlmServiceError::validation("spaceIds must not be empty"));
        }

        let mut candidates = Vec::new();
        for space_id in &request.space_ids {
            let scope = Self::scope(&context, *space_id);
            let rows = self
                .store
                .search_record_details_keyword(&scope, &request.query, request.top_k)
                .await
                .map_err(Self::map_store_error)?;

            for row in rows {
                let memory = Self::map_record(row)?;
                let score = keyword_match_score(&request.query, &memory.canonical_text);
                if score > 0.0 {
                    candidates.push(RetrievalCandidate {
                        memory,
                        retriever_name: "keyword".to_string(),
                        raw_score: score,
                        rank: 0,
                    });
                }
            }
        }

        let fused = fuse_retrieval_candidates(candidates, request.top_k as usize);
        let retrieval_id = self.next_id()?;
        let trace_id = retrieval_id.to_string();
        let primary_scope = Self::scope(&context, request.space_ids[0]);
        let hits: Vec<LlmRetrievalHit> = fused
            .iter()
            .enumerate()
            .map(|(_index, hit)| {
                Ok(LlmRetrievalHit {
                    hit_id: self.next_id()?,
                    memory: Some(hit.memory.clone()),
                    record_id: Some(hit.memory.record_id),
                    retriever_name: hit.retriever_name.clone(),
                    result_rank: hit.rank,
                    raw_score: Some(hit.raw_score),
                    fused_score: Some(hit.fused_score),
                    explanation: None,
                    status: "accepted".to_string(),
                })
            })
            .collect::<LlmServiceResult<Vec<_>>>()?;

        let trace_hits: Vec<LlmRetrievalHitDraft> = hits
            .iter()
            .map(|hit| LlmRetrievalHitDraft {
                hit_id: hit.hit_id.to_string(),
                record_id: hit.record_id.map(|value| value.to_string()),
                retriever_name: hit.retriever_name.clone(),
                result_rank: i64::from(hit.result_rank),
                raw_score: hit.raw_score,
                fused_score: hit.fused_score,
                explanation_json: None,
                status: hit.status.clone(),
            })
            .collect();

        let _ = self
            .store
            .append_retrieval_trace(&AppendLlmRetrievalTraceCommand {
                scope: primary_scope,
                trace_id: trace_id.clone(),
                actor_id: request.actor_id.clone(),
                query_text: Some(request.query.clone()),
                query_hash: format!("query:{retrieval_id}"),
                retrievers_json: Some(r#"["keyword","sql"]"#.to_string()),
                latency_ms: Some(1),
                degraded: false,
                metadata_json: None,
                hits: trace_hits,
                context_pack: None,
            })
            .await
            .map_err(Self::map_store_error)?;

        let trace = if request.include_trace.unwrap_or(false) {
            Some(LlmRetrievalTrace {
                trace_id: retrieval_id,
                space_id: Some(request.space_ids[0]),
                retrieval_profile_id: request.retrieval_profile_id,
                actor_id: request.actor_id,
                query_text: Some(request.query),
                query_hash: format!("query:{retrieval_id}"),
                result_count: hits.len() as i32,
                degraded: false,
                created_at: platform::current_timestamp(),
            })
        } else {
            None
        };

        Ok(LlmRetrievalResult {
            retrieval_id,
            trace,
            hits,
            degraded: false,
        })
    }

    async fn retrieve_retrieval(
        &self,
        context: LlmOpenApiRequestContext,
        retrieval_id: u64,
    ) -> LlmServiceResult<LlmRetrievalResult> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let trace = self
            .store
            .retrieve_retrieval_trace_for_tenant(tenant_id, &retrieval_id.to_string())
            .await
            .map_err(Self::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("retrieval not found"))?;

        let mut hits = Vec::new();
        for (index, hit) in trace.hits.iter().enumerate() {
            let memory = if let Some(record_id) = hit.record_id.as_deref() {
                self.store
                    .retrieve_record_detail_for_tenant(tenant_id, record_id)
                    .await
                    .map_err(Self::map_store_error)?
                    .map(Self::map_record)
                    .transpose()?
            } else {
                None
            };

            hits.push(LlmRetrievalHit {
                hit_id: hit
                    .hit_id
                    .parse()
                    .ok()
                    .or_else(|| Self::parse_id(&hit.hit_id))
                    .unwrap_or_else(|| retrieval_id.saturating_add(index as u64 + 1)),
                memory,
                record_id: hit.record_id.as_deref().and_then(Self::parse_id),
                retriever_name: hit.retriever_name.clone(),
                result_rank: i32::try_from(hit.result_rank).unwrap_or(1),
                raw_score: hit.raw_score,
                fused_score: hit.fused_score,
                explanation: hit
                    .explanation_json
                    .as_deref()
                    .and_then(|value| serde_json::from_str(value).ok()),
                status: hit.status.clone(),
            });
        }

        Ok(LlmRetrievalResult {
            retrieval_id,
            trace: Some(LlmRetrievalTrace {
                trace_id: retrieval_id,
                space_id: Some(1),
                retrieval_profile_id: None,
                actor_id: trace.actor_id,
                query_text: trace.query_text,
                query_hash: trace.query_hash,
                result_count: trace.result_count as i32,
                degraded: trace.degraded,
                created_at: platform::current_timestamp(),
            }),
            hits,
            degraded: trace.degraded,
        })
    }

    async fn retrieve_provider_health(
        &self,
        _context: LlmOpenApiRequestContext,
    ) -> LlmServiceResult<LlmProviderHealth> {
        Ok(LlmProviderHealth {
            status: LlmProviderHealthStatus::Healthy,
            checked_at: platform::current_timestamp(),
            providers: Vec::new(),
        })
    }

    async fn create_context_pack(
        &self,
        context: LlmOpenApiRequestContext,
        request: LlmContextPackRequest,
    ) -> LlmServiceResult<LlmContextPack> {
        if request.space_ids.is_empty() {
            return Err(LlmServiceError::validation("spaceIds must not be empty"));
        }

        let retrieval = self
            .create_retrieval(
                context.clone(),
                LlmRetrievalRequest {
                    query: request.query.clone(),
                    space_ids: request.space_ids.clone(),
                    actor_id: request.actor_id.clone(),
                    retrieval_profile_id: request.retrieval_profile_id,
                    record_types: None,
                    filters: request.filters.clone(),
                    top_k: 10,
                    context_budget_tokens: request.context_budget_tokens,
                    include_trace: Some(false),
                },
            )
            .await?;

        let (pack, estimated_tokens, truncated) =
            build_context_pack_from_hits(&retrieval.hits, request.context_budget_tokens);
        let context_pack_id = self.next_id()?;
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let primary_space = request.space_ids[0] as i64;

        self.store
            .insert_context_pack_open_api(
                tenant_id,
                primary_space,
                &context_pack_id.to_string(),
                Some(&retrieval.retrieval_id.to_string()),
                request.actor_id.as_deref(),
                Some(&request.query),
                &pack.to_string(),
                i64::from(estimated_tokens),
                truncated,
            )
            .await
            .map_err(Self::map_store_error)?;

        Ok(LlmContextPack {
            context_pack_id,
            retrieval_id: Some(retrieval.retrieval_id),
            query: Some(request.query),
            pack,
            estimated_tokens,
            truncated,
            created_at: platform::current_timestamp(),
        })
    }

    async fn retrieve_context_pack(
        &self,
        context: LlmOpenApiRequestContext,
        context_pack_id: u64,
    ) -> LlmServiceResult<LlmContextPack> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        let row = self
            .store
            .retrieve_context_pack_for_tenant(tenant_id, &context_pack_id.to_string())
            .await
            .map_err(Self::map_store_error)?
            .ok_or_else(|| LlmServiceError::not_found("context pack not found"))?;

        let pack = serde_json::from_str(&row.pack_json)
            .unwrap_or_else(|_| serde_json::json!({ "fragments": [] }));

        Ok(LlmContextPack {
            context_pack_id,
            retrieval_id: None,
            query: row.query_text,
            pack,
            estimated_tokens: row.estimated_tokens as i32,
            truncated: row.truncated,
            created_at: row.created_at,
        })
    }

    async fn create_feedback(
        &self,
        context: LlmOpenApiRequestContext,
        request: LlmFeedbackRequest,
    ) -> LlmServiceResult<MemoryFeedback> {
        let feedback_id = self.next_id()?;
        let scope = Self::scope(&context, 1);
        self.store
            .append_audit(
                &scope,
                &feedback_id.to_string(),
                "feedback.create",
                &request.target_type,
                &request.target_id.to_string(),
                "accepted",
            )
            .await
            .map_err(Self::map_store_error)?;

        Ok(MemoryFeedback {
            feedback_id,
            target_type: request.target_type,
            target_id: request.target_id,
            feedback_type: request.feedback_type,
            created_at: platform::current_timestamp(),
        })
    }

    async fn create_extraction(
        &self,
        context: LlmOpenApiRequestContext,
        request: LlmExtractionRequest,
    ) -> LlmServiceResult<LlmLearningJob> {
        let job_id = self.next_id()?;
        let scope = Self::scope(&context, request.space_id);
        let mut created_candidates = 0_u32;

        for event_id in &request.input_events {
            if let Some(payload) = self
                .store
                .retrieve_event_payload(&scope, &event_id.to_string())
                .await
                .map_err(Self::map_store_error)?
            {
                let proposed = payload
                    .get("content")
                    .and_then(|value| value.as_str())
                    .unwrap_or("extracted memory candidate")
                    .to_string();
                let candidate_id = self.next_id()?.to_string();
                self.store
                    .create_candidate(&CreateLlmCandidateCommand {
                        scope: scope.clone(),
                        candidate_id,
                        candidate_type: "extraction".to_string(),
                        record_type: "semantic".to_string(),
                        proposed_text: proposed,
                        proposed_payload_json: Some(payload.to_string()),
                        evidence_json: Some(format!(r#"["event:{event_id}"]"#)),
                        confidence: 0.7,
                    })
                    .await
                    .map_err(Self::map_store_error)?;
                created_candidates += 1;
            }
        }

        Ok(LlmLearningJob {
            job_id,
            space_id: Some(request.space_id),
            job_type: "extraction".to_string(),
            state: if created_candidates > 0 {
                "completed".to_string()
            } else {
                "failed".to_string()
            },
            priority: 0,
            result: Some(serde_json::json!({
                "candidateCount": created_candidates,
                "extractionMode": request.extraction_mode.unwrap_or_else(|| "deterministic".to_string()),
            })),
            created_at: platform::current_timestamp(),
            updated_at: platform::current_timestamp(),
        })
    }

    async fn list_candidates(
        &self,
        context: LlmOpenApiRequestContext,
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
            .map_err(Self::map_store_error)?;
        let items = rows
            .into_iter()
            .map(|row| {
                Ok(LlmCandidate {
                    candidate_id: row.candidate_id.parse().unwrap_or(0),
                    space_id: u64::try_from(row.space_id.max(0)).unwrap_or(0),
                    candidate_type: row.candidate_type,
                    record_type: Self::record_type_from_db(&row.record_type),
                    proposed_text: row.proposed_text,
                    confidence: row.confidence,
                    decision_state: row.decision_state,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            })
            .collect::<LlmServiceResult<Vec<_>>>()?;

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
        context: LlmOpenApiRequestContext,
        candidate_id: u64,
    ) -> LlmServiceResult<LlmCandidate> {
        let tenant_id = i64::try_from(context.tenant_id).unwrap_or(i64::MAX);
        match self
            .store
            .retrieve_candidate_for_tenant(tenant_id, &candidate_id.to_string())
            .await
            .map_err(Self::map_store_error)?
        {
            Some(row) => Self::map_candidate(row),
            None => Err(LlmServiceError::not_found("candidate not found")),
        }
    }
}
