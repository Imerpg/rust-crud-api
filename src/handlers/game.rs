use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::run_db,
    error::{ApiError, ApiResult},
    models::{GameModel, NewGame},
    schema::games,
    AppState,
};

fn ensure_affected(rows: usize, message: &'static str) -> ApiResult<()> {
    if rows == 0 {
        Err(ApiError::NotFound(message))
    } else {
        Ok(())
    }
}

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(mut body): Json<NewGame>,
) -> ApiResult<impl IntoResponse> {
    body.id = Uuid::new_v4().to_string();
    let to_insert = body.clone();

    run_db(&data, move |conn| {
        diesel::insert_into(games::table)
            .values(&to_insert)
            .execute(conn)
    })
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "data": { "game": body }
        })),
    ))
}

pub async fn game_list_handler(
    State(data): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    let games_list: Vec<GameModel> = run_db(&data, |conn| {
        games::table
            .order(games::name.asc())
            .load::<GameModel>(conn)
    })
    .await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "count": games_list.len(),
        "data": { "games": games_list }
    })))
}

pub async fn delete_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    let id = game_id.to_string();

    let rows = run_db(&data, move |conn| {
        diesel::delete(games::table.filter(games::id.eq(id))).execute(conn)
    })
    .await?;

    ensure_affected(rows, "Game not found")?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Game deleted successfully"
    })))
}

pub async fn update_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<NewGame>,
) -> ApiResult<impl IntoResponse> {
    let id = game_id.to_string();

    let rows = run_db(&data, move |conn| {
        diesel::update(games::table.filter(games::id.eq(id)))
            .set((
                games::name.eq(body.name),
                games::creator.eq(body.creator),
                games::plays.eq(body.plays),
            ))
            .execute(conn)
    })
    .await?;

    ensure_affected(rows, "Game not found")?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Game updated successfully"
    })))
}