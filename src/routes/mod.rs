use axum::Router;
use axum::routing::{get, post};
use crate::routes::entry::{get_all_entries, post_entry};

pub mod entry;

pub fn router() -> Router {
    Router::new()
        .route("/api/entry", post(post_entry))
        .route("/api/entries", get(get_all_entries))
}