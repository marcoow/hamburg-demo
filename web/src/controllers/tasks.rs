use crate::{error::Error, state::SharedAppState};
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use hamburg_demo_db::entities;
use tracing::info;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create(
    State(app_state): State<SharedAppState>,
    Json(task): Json<entities::tasks::TaskChangeset>,
) -> Result<(StatusCode, Json<entities::tasks::Task>), Error> {
    let task = entities::tasks::create(task, &app_state.db_pool).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

#[axum::debug_handler]
pub async fn read_all(
    State(app_state): State<SharedAppState>,
) -> Result<Json<Vec<entities::tasks::Task>>, Error> {
    let tasks = entities::tasks::load_all(&app_state.db_pool).await?;

    info!("responding with {:?}", tasks);

    Ok(Json(tasks))
}
