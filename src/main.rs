mod config;
mod database;
mod routes;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{response::Html, routing::get, Extension, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::routes::entry::Entry;

#[derive(Clone)]
struct ApiState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::filter::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "hass_proxy=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::debug!("Tracing initiated");

    let config = config::load_config();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Connected to database");

    let app = routes::router()
        .route("/", get(index))
        .route("/hass_dump", post(any_post))
        .layer(Extension(ApiState { db: pool.clone() }))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.address, config.port))
        .await
        .expect("Failed to bind to address");

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn any_post(data: String) -> StatusCode {
    tracing::warn!("Data: {:#?}", data);

    let entry: Entry = serde_json::from_str(&data).unwrap();
    tracing::error!("Entry: {:#?}", entry);
    StatusCode::CREATED
}
