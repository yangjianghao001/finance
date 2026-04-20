use axum::response::IntoResponse;
use sea_orm::sqlx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbErr(#[from] sqlx::Error),
    #[error("Internal Server error")]
    InternalServerError(#[from] anyhow::Error),
}
impl From<AppError> for axum::response::Response {
    fn from(err: AppError) -> Self {
        tracing::error!("error: {}", err);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )
            .into_response()
    }
}
