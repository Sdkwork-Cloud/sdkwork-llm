//! Gateway bootstrap for sdkwork-llm.

use axum::Router;
use sdkwork_intelligence_llm_repository_sqlx::bootstrap_llm_data_plane_from_env;
use sdkwork_intelligence_llm_service::OpenLlmService;
use sdkwork_routes_llm_app_api::{
    build_router_with_shared_app_api, wrap_router_with_web_framework_from_env as wrap_app_router,
};
use sdkwork_routes_llm_backend_api::{
    build_router_with_shared_backend_api,
    wrap_router_with_web_framework_from_env as wrap_backend_router,
};
use sdkwork_routes_llm_open_api::{
    build_router_with_shared_open_api, wrap_router_with_web_framework_from_env as wrap_open_router,
};
use std::sync::Arc;

pub struct ApplicationAssembly {
    pub router: Router,
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    let data_plane = bootstrap_llm_data_plane_from_env().await?;
    let product = Arc::new(OpenLlmService::new(data_plane.store));

    let open_business_router = build_router_with_shared_open_api(product.clone());
    let app_business_router = build_router_with_shared_app_api(product.clone());
    let backend_business_router = build_router_with_shared_backend_api(product);

    let open_router = wrap_open_router(open_business_router).await;
    let app_router = wrap_app_router(app_business_router).await;
    let backend_router = wrap_backend_router(backend_business_router).await;

    let router = Router::new()
        .merge(open_router)
        .merge(app_router)
        .merge(backend_router);

    Ok(ApplicationAssembly { router })
}
