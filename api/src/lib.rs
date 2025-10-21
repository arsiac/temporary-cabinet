mod test;

/// Backend router
pub fn router() -> axum::Router<ServerState> {
    axum::Router::new().nest("/test", test::router())
}

/// Backend server state
#[derive(Clone)]
pub struct ServerState;
