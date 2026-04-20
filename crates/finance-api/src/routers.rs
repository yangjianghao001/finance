use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

async fn finance_detail() -> impl IntoResponse {
    Json("finance detail")
}

pub fn routes() -> Router {
    Router::new().route("/finance/detail", get(finance_detail))
}
