mod routers;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

async fn log_requests(request: Request, next: Next) -> Response {
    tracing::info!("{request:?}");
    let response = next.run(request).await;
    tracing::info!("{response:?}");
    response
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    let _guard = finance_config::log::init();
    let app_port = finance_config::application::get().server.port();
    let ip_addr = finance_config::application::get().server.ip_addr();
    let listener = TcpListener::bind(format!("{}:{}", ip_addr, app_port)).await?;

    let router = axum::Router::new()
        .merge(routers::routes())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(axum::middleware::from_fn(log_requests))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));

    tracing::info!("Server start {:?}", listener);
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

pub fn main() {
    let result = start();
    if let Some(error) = result.err() {
        tracing::error!(?error, "Server start error");
        std::process::exit(1);
    }
}
