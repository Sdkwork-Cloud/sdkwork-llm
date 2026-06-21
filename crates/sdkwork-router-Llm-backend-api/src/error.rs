use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sdkwork_llm_contract::{LlmServiceError, LlmServiceErrorKind, ProblemDetails};

pub type BackendApiResult<T> = Result<T, BackendApiError>;

#[derive(Debug, Clone)]
pub struct BackendApiError {
    status: StatusCode,
    code: String,
    detail: String,
}

impl BackendApiError {
    pub fn new(status: StatusCode, code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            status,
            code: code.into(),
            detail: detail.into(),
        }
    }

    pub fn not_implemented(operation_id: &'static str) -> Self {
        Self::new(
            StatusCode::NOT_IMPLEMENTED,
            "operation_not_implemented",
            format!("operation is not implemented: {operation_id}"),
        )
    }
}

impl From<LlmServiceError> for BackendApiError {
    fn from(error: LlmServiceError) -> Self {
        let status = match error.kind {
            LlmServiceErrorKind::NotFound => StatusCode::NOT_FOUND,
            LlmServiceErrorKind::Conflict => StatusCode::CONFLICT,
            LlmServiceErrorKind::Validation => StatusCode::BAD_REQUEST,
            LlmServiceErrorKind::Storage => StatusCode::INTERNAL_SERVER_ERROR,
            LlmServiceErrorKind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
        };
        Self::new(status, error.code, error.detail)
    }
}

#[derive(Debug, Clone)]
pub struct BackendApiProblem {
    status: StatusCode,
    problem: Box<ProblemDetails>,
}

impl BackendApiProblem {
    pub fn new(status: StatusCode, code: impl Into<String>, detail: impl Into<String>) -> Self {
        let title = status
            .canonical_reason()
            .unwrap_or("HTTP Error")
            .to_string();
        Self {
            status,
            problem: Box::new(ProblemDetails {
                r#type: "about:blank".to_string(),
                title,
                status: status.as_u16(),
                detail: Some(detail.into()),
                instance: None,
                code: Some(code.into()),
            }),
        }
    }
}

impl From<BackendApiError> for BackendApiProblem {
    fn from(error: BackendApiError) -> Self {
        Self::new(error.status, error.code, error.detail)
    }
}

impl From<LlmServiceError> for BackendApiProblem {
    fn from(error: LlmServiceError) -> Self {
        BackendApiError::from(error).into()
    }
}

impl IntoResponse for BackendApiProblem {
    fn into_response(self) -> Response {
        let mut response = (self.status, Json(*self.problem)).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}
