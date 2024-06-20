use clap::Parser;
use rust_todo_app_axum::{config::Config, infrastructure::database};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes;

pub async fn create_server() {
    // This returns an error if the "env" file is not found
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_todo_app_axum=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let config = Config::parse();

    let pool = database::get_postgres_pool(&config.database_url.as_str())
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to connect to Postgres with provided URL: {}",
                &config.database_url
            )
        });

    info!("Database connection successfully made");

    database::migrate(&pool).await; // Only for development

    info!("Database migration successfully completed");

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    let addr = listener.local_addr().unwrap();

    info!("Listening on {}", addr);

    let router = routes::build_routes(pool, config);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
