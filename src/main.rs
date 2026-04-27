use std::sync::Arc;
use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


mod app_state;
mod config;
mod db;
mod discovery;
mod groups;
mod targets;
mod labels;

use app_state::AppState;
use config::Config;
use discovery::service::DiscoveryService;
use groups::service::GroupService;
use targets::service::TargetService;
use labels::service::LabelService;

#[derive(OpenApi)]
#[openapi(
    paths(
        discovery::handlers::get_sd_configs,
    ),
    components(
        schemas(discovery::models::NarthexSDResp)
    ),
    tags(
        (name = "discovery", description = "Prometheus SD API endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "narthex=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Load configuration
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize database pool
    let pool = match db::init_pool(&config.database_url()).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Initialize AppState
    let state = AppState {
        discovery_service: Arc::new(DiscoveryService::new(pool.clone())),
        group_service: Arc::new(GroupService::new(pool.clone())),
        target_service: Arc::new(TargetService::new(pool.clone())),
        label_service: Arc::new(LabelService::new(pool.clone())),
    };

    // Build the application
    let app = Router::new()
        .nest("/api/v1", discovery::router())
        .nest("/api/v1/groups", groups::router())
        .nest("/api/v1/groups", targets::router()) // Handles /groups/:group_id/targets
        .nest("/api/v1/groups", labels::router()) // Handles /groups/:group_id/labels
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    // Bind and start the server
    let addr = format!("{}:{}", config.app_host, config.app_port);
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
