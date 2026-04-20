use axum::extract::Request;
use finance_common::X_REQUEST_ID;

pub fn get_request_id_from_headers(req: &Request) -> String {
    req.headers()
        .get(X_REQUEST_ID)
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.into())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
}
