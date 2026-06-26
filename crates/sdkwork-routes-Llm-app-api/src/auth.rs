use axum::{http::StatusCode, Extension};

use sdkwork_llm_contract::LlmAppRequestContext;

use crate::ApiProblem;

pub fn require_app_context(
    context: Option<Extension<LlmAppRequestContext>>,
) -> Result<LlmAppRequestContext, ApiProblem> {
    context.map(|Extension(context)| context).ok_or_else(|| {
        ApiProblem::new(
            StatusCode::UNAUTHORIZED,
            "missing_app_request_context",
            "authenticated app request context is required",
        )
    })
}
