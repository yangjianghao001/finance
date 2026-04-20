mod util;

use axum::extract::Request;
use tracing::instrument;

#[instrument(
    fields(request_id = %util::get_request_id_from_headers(&req)),
    skip(req))]
pub async fn hello_finance(req: Request) -> String {
    "HelloFinance".into()
}
