/// Backend router
pub fn router<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().route("/ping", axum::routing::get(ping))
}

/// ping the server
/// GET /api/ping
pub(crate) async fn ping() -> String {
    "pong".to_string()
}

/// Backend server state
#[derive(Clone)]
pub struct ServerState {
    pub connection: sea_orm::DatabaseConnection,
}

impl ServerState {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
