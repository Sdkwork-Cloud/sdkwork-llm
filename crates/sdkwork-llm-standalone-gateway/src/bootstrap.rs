use axum::Router;
use sdkwork_llm_gateway_assembly::assemble_application_router;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

pub async fn build_router() -> Result<Router, String> {
    let assembly = assemble_application_router().await?;
    Ok(service_router(
        assembly.router,
        ServiceRouterConfig::default().with_always_ready(),
    ))
}
