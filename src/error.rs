use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::error;

/// Erreur API centralisée : tous les handlers retournent `ApiResult<T>`.
#[derive(Debug)]
pub enum ApiError {
    /// Ressource introuvable → 404
    NotFound(&'static str),
    /// Conflit (déjà existant, FK violée) → 409
    Conflict(&'static str),
    /// Données invalides → 422
    Unprocessable(String),
    /// Tout le reste → 500 (message générique au client, détail loggé)
    Internal(anyhow::Error),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m.to_string()),
            ApiError::Conflict(m) => (StatusCode::CONFLICT, m.to_string()),
            ApiError::Unprocessable(m) => (StatusCode::UNPROCESSABLE_ENTITY, m),
            ApiError::Internal(e) => {
                // Le détail reste côté serveur, jamais exposé au client
                error!("Erreur interne: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
            }
        };

        (status, Json(serde_json::json!({
            "status": "error",
            "message": message
        })))
        .into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError::Internal(e)
    }
}

/// Traduit une erreur Diesel en erreur API, sans matching de chaîne SQL.
pub fn map_diesel_err(e: diesel::result::Error) -> ApiError {
    use diesel::result::{DatabaseErrorKind, Error};

    match e {
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            ApiError::Conflict("Record already exists")
        }
        Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => {
            ApiError::Conflict("Referenced record does not exist")
        }
        other => ApiError::Internal(other.into()),
    }
}