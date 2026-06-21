use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use sdkwork_llm_contract::{
    ListLlmCandidatesQuery, ListLlmRecordsQuery, LlmContextPackRequest, LlmEventRequest,
    LlmExtractionRequest, LlmFeedbackRequest, LlmOpenApi, LlmOpenApiRequestContext,
    LlmRecordPatch, LlmRecordRequest, LlmRetrievalRequest, LlmServiceResult,
};
use std::sync::Arc;

use crate::{auth::require_context, paths, ApiProblem};

#[derive(Clone)]
struct OpenState {
    api: Arc<dyn LlmOpenApi>,
}

pub fn build_router_with_open_api<A>(api: A) -> Router
where
    A: LlmOpenApi,
{
    build_router_with_shared_open_api(Arc::new(api))
}

pub fn build_router_with_shared_open_api(api: Arc<dyn LlmOpenApi>) -> Router {
    Router::new()
        .route(paths::CAPABILITIES, get(retrieve_capabilities))
        .route(paths::EVENTS, post(create_event))
        .route(paths::EVENT, get(retrieve_event))
        .route(paths::RECORDS, get(list_memories).post(create_memory))
        .route(
            paths::RECORD,
            get(retrieve_memory)
                .patch(update_memory)
                .delete(delete_memory),
        )
        .route(paths::RETRIEVALS, post(create_retrieval))
        .route(paths::RETRIEVAL, get(retrieve_retrieval))
        .route(paths::CONTEXT_PACKS, post(create_context_pack))
        .route(paths::CONTEXT_PACK, get(retrieve_context_pack))
        .route(paths::FEEDBACK, post(create_feedback))
        .route(paths::EXTRACTIONS, post(create_extraction))
        .route(paths::CANDIDATES, get(list_candidates))
        .route(paths::CANDIDATE, get(retrieve_candidate))
        .route(paths::PROVIDER_HEALTH, get(retrieve_provider_health))
        .with_state(OpenState { api })
}

async fn retrieve_capabilities(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.retrieve_capabilities(context).await)
}

async fn create_event(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Json(request): Json<LlmEventRequest>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    created_json(state.api.create_event(context, request).await)
}

async fn retrieve_event(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(event_id): Path<u64>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.retrieve_event(context, event_id).await)
}

async fn list_memories(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Query(query): Query<ListLlmRecordsQuery>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.list_memories(context, query).await)
}

async fn create_memory(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Json(request): Json<LlmRecordRequest>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    created_json(state.api.create_memory(context, request).await)
}

async fn retrieve_memory(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(record_id): Path<u64>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.retrieve_memory(context, record_id).await)
}

async fn update_memory(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(record_id): Path<u64>,
    Json(patch): Json<LlmRecordPatch>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.update_memory(context, record_id, patch).await)
}

async fn delete_memory(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(record_id): Path<u64>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    no_content(state.api.delete_memory(context, record_id).await)
}

async fn create_retrieval(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Json(request): Json<LlmRetrievalRequest>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    created_json(state.api.create_retrieval(context, request).await)
}

async fn retrieve_retrieval(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(retrieval_id): Path<u64>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.retrieve_retrieval(context, retrieval_id).await)
}

async fn create_context_pack(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Json(request): Json<LlmContextPackRequest>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    created_json(state.api.create_context_pack(context, request).await)
}

async fn retrieve_context_pack(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(context_pack_id): Path<u64>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(
        state
            .api
            .retrieve_context_pack(context, context_pack_id)
            .await,
    )
}

async fn create_feedback(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Json(request): Json<LlmFeedbackRequest>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    created_json(state.api.create_feedback(context, request).await)
}

async fn create_extraction(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Json(request): Json<LlmExtractionRequest>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    created_json(state.api.create_extraction(context, request).await)
}

async fn list_candidates(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Query(query): Query<ListLlmCandidatesQuery>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.list_candidates(context, query).await)
}

async fn retrieve_candidate(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
    Path(candidate_id): Path<u64>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.retrieve_candidate(context, candidate_id).await)
}

async fn retrieve_provider_health(
    State(state): State<OpenState>,
    context: Option<Extension<LlmOpenApiRequestContext>>,
) -> Result<Response, ApiProblem> {
    let context = require_context(context)?;
    ok_json(state.api.retrieve_provider_health(context).await)
}

fn ok_json<T>(result: LlmServiceResult<T>) -> Result<Response, ApiProblem>
where
    T: serde::Serialize,
{
    match result {
        Ok(value) => Ok((StatusCode::OK, Json(value)).into_response()),
        Err(error) => Err(error.into()),
    }
}

fn created_json<T>(result: LlmServiceResult<T>) -> Result<Response, ApiProblem>
where
    T: serde::Serialize,
{
    match result {
        Ok(value) => Ok((StatusCode::CREATED, Json(value)).into_response()),
        Err(error) => Err(error.into()),
    }
}

fn no_content(result: LlmServiceResult<()>) -> Result<Response, ApiProblem> {
    match result {
        Ok(()) => Ok(StatusCode::NO_CONTENT.into_response()),
        Err(error) => Err(error.into()),
    }
}
