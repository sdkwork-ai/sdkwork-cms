use std::sync::Arc;

use sdkwork_api_cms_assembly::assemble_api_router;
use sdkwork_api_cms_standalone_gateway::CmsPostgresReadinessCheck;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let assembly = assemble_api_router().await?;
    let router = assembly
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_CMS_ENVIRONMENT"],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
        ))
        .layer(TraceLayer::new_for_http());
    let app = service_router(
        router,
        ServiceRouterConfig::default().with_readiness_check(Arc::new(
            CmsPostgresReadinessCheck::new(assembly.readiness_pool),
        )),
    );

    let bind_address = std::env::var("SDKWORK_CMS_APPLICATION_PUBLIC_INGRESS_BIND")
        .expect("SDKWORK_CMS_APPLICATION_PUBLIC_INGRESS_BIND must come from a topology profile");
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!(%bind_address, "sdkwork-api-cms-standalone-gateway listening");
    axum::serve(listener, app).await?;
    Ok(())
}
