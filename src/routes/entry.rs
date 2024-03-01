use crate::ApiState;
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum_macros::debug_handler;
use chrono::Utc;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Entry {
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    pub state: String,
    pub area: String,
    #[serde(rename = "utc_timestamp")]
    pub timestamp: chrono::DateTime<Utc>,
}

#[debug_handler]
pub async fn post_entry(Extension(state): Extension<ApiState>, Json(data): Json<Entry>) -> StatusCode {
    let id = format!("{}.{}", data.area, data.entity_id);
    
    tracing::debug!("Creating entry: {:?}", data);

    sqlx::query_as!(
        Entry,
        r#"
        INSERT INTO entries (entity_id, state, area, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        data.state,
        data.area,
        data.timestamp,
    )
        .execute(&state.db)
        .await
        .unwrap();

    tracing::info!("Entry created: {}", id);

    StatusCode::CREATED
}

pub async fn get_all_entries(Extension(state): Extension<ApiState>) -> Json<Vec<Entry>> {
    let entries = sqlx::query_as!(
        Entry,
        r#"
        SELECT entity_id, state, area, timestamp
        FROM entries
        "#,
    )
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(entries)
}
