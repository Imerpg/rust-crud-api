use std::sync::Arc;

use dotenv::dotenv;
use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2;
use crate::route::create_router;

mod handlers {
    pub mod game;
}

mod db;
mod error;
mod models;
mod route;
mod schema;

pub struct AppState {
    db: r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");

    let _conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erreur de connexion à {}", database_url));

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Échec de la création du pool");

    let app = create_router(Arc::new(AppState { db: pool }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}