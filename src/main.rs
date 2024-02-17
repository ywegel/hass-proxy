use axum::{response::Html, routing::get, Router};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let config = load_config();

    let app = Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.address, config.port))
        .await
        .expect("Failed to bind to address");

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    address: String,
    port: u16,
}

fn load_config() -> Config {
    let env_path = dotenvy::dotenv().expect(".env variable should exist in working directory");
    tracing::info!("Loaded .env successfully from {}", env_path.display());

    let config = envy::from_env::<Config>().expect("Failed to deserialize env variables");
    tracing::debug!("Config: {:#?}", config);

    return config;
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
