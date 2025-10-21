/// Backend router
pub fn router<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().route("/ping", axum::routing::get(ping))
}

/// Backend server state
#[derive(Clone)]
pub struct ServerState;

/// ping the server
/// GET /api/ping
pub(crate) async fn ping() -> String {
    "pong".to_string()
}
