use sdkwork_llm_standalone_gateway::build_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let bind_address = std::env::var("SDKWORK_LLM_APPLICATION_PUBLIC_INGRESS_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
    let app = build_router()
        .await
        .expect("llm standalone-gateway bootstrap failed");
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind llm standalone-gateway listener failed");
    tracing::info!("sdkwork-llm-standalone-gateway listening on {bind_address}");
    axum::serve(listener, app)
        .await
        .expect("serve llm standalone-gateway failed");
}
