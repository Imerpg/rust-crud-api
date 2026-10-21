use std::sync::Arc;

use diesel::{SqliteConnection, r2d2};
use crate::{AppState, r2d2::PooledConnection};

use crate::error::{map_diesel_err, ApiError, ApiResult};

pub type DbConn = PooledConnection<r2d2::ConnectionManager<SqliteConnection>>;

pub async fn run_db<T, F>(data: &Arc<AppState>, f: F) -> ApiResult<T>
where
    F: FnOnce(&mut DbConn) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let pool = data.db.clone();

    tokio::task::spawn_blocking(move || {
        let mut conn = pool
            .get()
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("pool: {e}")))?;
        f(&mut conn).map_err(map_diesel_err)
    })
    .await
    .map_err(|e| ApiError::Internal(e.into()))?
}