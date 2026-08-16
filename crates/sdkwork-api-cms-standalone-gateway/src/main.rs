use std::sync::Arc;

use sdkwork_api_cms_assembly::assemble_api_router_runtime;
use sdkwork_iam_web_adapter::{
    build_web_framework_builder, iam_web_request_context_resolver_from_database_pool_for_audiences,
    iam_web_request_context_resolver_from_env, IamAuditEmitter, IamSecurityEventEmitter,
};
use sdkwork_web_bootstrap::{infra_public_path_prefixes, ComposedApiAssembly};
use tracing_subscriber::EnvFilter;

const APPLICATION_ID: &str = "sdkwork-cms";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let runtime = assemble_api_router_runtime().await?;
    let assembly = runtime.contribution;
    let environment = std::env::var("SDKWORK_ENVIRONMENT")
        .or_else(|_| std::env::var("SDKWORK_CMS_ENVIRONMENT"))
        .unwrap_or_else(|_| "development".to_owned());
    let production = matches!(
        environment.trim().to_ascii_lowercase().as_str(),
        "prod" | "production"
    );
    let resolver = if production {
        iam_web_request_context_resolver_from_database_pool_for_audiences(
            runtime.database_pool.clone(),
            &[APPLICATION_ID],
        )
        .await?
    } else {
        iam_web_request_context_resolver_from_env().await
    };
    let mut framework = build_web_framework_builder(
        resolver,
        assembly.route_manifest.clone(),
        infra_public_path_prefixes(),
    );
    if production {
        let postgres_pool = runtime
            .database_pool
            .as_postgres()
            .cloned()
            .ok_or("production CMS gateway requires PostgreSQL")?;
        framework = framework
            .audit_emitter(Arc::new(IamAuditEmitter::new(
                postgres_pool.clone(),
                APPLICATION_ID,
                environment.clone(),
            )))
            .security_event_emitter(Arc::new(IamSecurityEventEmitter::new(
                postgres_pool,
                environment,
            )));
    }
    let app = ComposedApiAssembly::try_compose("SDKWork CMS API", vec![assembly])?
        .into_hosted(framework)
        .router;

    let bind_address = std::env::var("SDKWORK_CMS_APPLICATION_PUBLIC_INGRESS_BIND")
        .expect("SDKWORK_CMS_APPLICATION_PUBLIC_INGRESS_BIND must come from a topology profile");
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!(%bind_address, "sdkwork-api-cms-standalone-gateway listening");
    axum::serve(listener, app).await?;
    Ok(())
}
