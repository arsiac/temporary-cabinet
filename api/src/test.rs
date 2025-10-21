/// Test router
pub(crate) fn router() -> axum::Router<crate::ServerState> {
    use axum::routing::get;
    axum::Router::new().route("/ping", get(ping))
}

/// Ping
pub(crate) async fn ping() -> String {
    "pong".to_string()
}
