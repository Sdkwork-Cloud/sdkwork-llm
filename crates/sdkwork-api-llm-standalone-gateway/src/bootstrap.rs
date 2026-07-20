use axum::Router;
use sdkwork_api_llm_assembly::assemble_api_router;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

pub async fn build_router() -> Result<Router, String> {
    let assembly = assemble_api_router().await?;
    Ok(service_router(
        assembly.router,
        ServiceRouterConfig::default().with_always_ready(),
    ))
}
