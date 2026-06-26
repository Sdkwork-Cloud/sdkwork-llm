use axum::{http::StatusCode, Extension};

use sdkwork_llm_contract::LlmBackendRequestContext;

use crate::BackendApiProblem;

pub fn require_backend_context(
    context: Option<Extension<LlmBackendRequestContext>>,
) -> Result<LlmBackendRequestContext, BackendApiProblem> {
    context.map(|Extension(context)| context).ok_or_else(|| {
        BackendApiProblem::new(
            StatusCode::UNAUTHORIZED,
            "missing_backend_request_context",
            "authenticated backend request context is required",
        )
    })
}
