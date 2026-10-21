use std::sync::Arc;

use axum::{routing::{delete, post}, Router};
use crate::{AppState, handlers::game::{
    create_game_handler, delete_game_handler, game_list_handler, update_game_handler,
}};

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_game_handler).get(game_list_handler))
        .route("/{id}", delete(delete_game_handler).put(update_game_handler))
}