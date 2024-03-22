use crate::ApiState;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::Result;
use axum::http::StatusCode;
use axum_macros::debug_handler;
use chrono::Utc;
use crate::error::ResponseError;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    pub entity_id: String,
    pub state: String,
    pub area: String,
    #[serde(rename(deserialize = "utc_timestamp"))]
    pub timestamp: chrono::DateTime<Utc>,
}

pub struct EntityFromDatabase {
    pub id: i32,
    pub entity_id: String,
    pub state: String,
    pub area: String,
    pub timestamp: chrono::DateTime<Utc>,
}

impl From<EntityFromDatabase> for Entity {
    fn from(value: EntityFromDatabase) -> Self {
        Entity {
            entity_id: value.entity_id,
            state: value.state,
            area: value.area,
            timestamp: value.timestamp,
        }
    }
}

#[debug_handler]
pub async fn post_entity(Extension(state): Extension<ApiState>, Json(data): Json<Entity>) -> StatusCode {
    tracing::debug!("Creating entry: {:?}", data);

    sqlx::query_as!(
        Entity,
        r#"
        INSERT INTO entries (entity_id, state, area, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        data.entity_id,
        data.state,
        data.area,
        data.timestamp,
    )
        .execute(&state.db)
        .await
        .unwrap();

    tracing::info!("Entry created: {}", data.entity_id);

    StatusCode::CREATED
}

pub async fn get_all_entities(Extension(state): Extension<ApiState>) -> Result<Json<Vec<Entity>>, ResponseError> {
    let entries = sqlx::query_as!(
        Entity,
        r#"
        SELECT entity_id, state, area, timestamp
        FROM entries
        "#,
    )
        .fetch_all(&state.db)
        .await?;

    Ok(Json(entries))
}

#[debug_handler]
pub async fn get_latest_entities(Extension(state): Extension<ApiState>) -> Result<Json<Vec<Entity>>, ResponseError> {
    let entries = sqlx::query_as!(
        Entity,
        r#"
        SELECT DISTINCT ON (entity_id, area) entity_id, state, area, timestamp
        FROM entries
        ORDER BY entity_id, area, timestamp DESC;
        "#,
    )
        .fetch_all(&state.db)
        .await?;

    Ok(Json(entries))
}

#[derive(serde::Serialize)]
pub struct EntryWithData {
    pub entity_id: String,
    pub area: String,
    pub data: Vec<EntryData>,
}

#[derive(serde::Serialize)]
pub struct EntryData {
    #[serde(rename(serialize = "y"))]
    pub state: String,
    #[serde(rename(serialize = "x"))]
    pub timestamp: chrono::DateTime<Utc>,
}

pub async fn get_entity_data(Extension(state): Extension<ApiState>, Path((area, entity_id)): Path<(String, String)>) -> Result<Json<EntryWithData>, ResponseError> {
    let data = sqlx::query_as!(
        EntryData,
        r#"
            SELECT state, timestamp
            FROM entries
            WHERE entity_id = $1 AND area = $2;
        "#,
        entity_id,
        area
    )
        .fetch_all(&state.db)
        .await?;

    let entry = EntryWithData {
        entity_id,
        area,
        data,
    };

    Ok(Json(entry))
}
