use redis::aio::ConnectionManager;
use sea_orm::DbConn;

pub struct AppState {
    pub redis: ConnectionManager,
    pub db: DbConn,
}

impl AppState {
    pub fn new(redis: ConnectionManager, db: DbConn) -> Self {
        Self { redis, db }
    }
}
