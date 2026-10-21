use std::sync::Arc;

use axum::Router;

mod game;
use game::game_routes;

use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1/games", game_routes())
        .with_state(app_state)
}