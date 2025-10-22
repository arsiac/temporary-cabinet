mod cabinet;

/// Backend router
pub fn router() -> axum::Router<ServerState> {
    axum::Router::new()
        .route("/ping", axum::routing::get(ping))
        .nest("/cabinets", cabinet::router())
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
    pub data_folder: std::path::PathBuf,
}

impl ServerState {
    pub fn new(connection: sea_orm::DatabaseConnection, data_folder: std::path::PathBuf) -> Self {
        Self {
            connection,
            data_folder,
        }
    }
}
