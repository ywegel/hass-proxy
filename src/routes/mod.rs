use axum::Router;
use axum::routing::{get, post};
use crate::routes::entry::{get_all_entities, get_entity_by_id, get_latest_entities, post_entity};

pub mod entry;

pub fn router() -> Router {
    Router::new()
        .route("/api/entity", post(post_entity))
        .route("/api/entity/:area/:entity_id", get(get_entity_by_id))
        .route("/api/entities", get(get_all_entities))
        .route("/api/entities/latest", get(get_latest_entities))
}