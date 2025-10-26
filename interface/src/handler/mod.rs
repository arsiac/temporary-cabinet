mod cabinet;
mod crypto;

/// Backend router
pub fn router() -> axum::Router<ServerState> {
    axum::Router::new()
        .route("/ping", axum::routing::get(ping))
        .nest("/cabinet", cabinet::router())
        .nest("/crypto", crypto::router())
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
    pub max_cabinet_number: u64,
    pub max_keypair_number: u64,
}

impl ServerState {
    pub fn new(
        connection: sea_orm::DatabaseConnection,
        data_folder: std::path::PathBuf,
        max_cabinet_number: u64,
    ) -> Self {
        Self {
            connection,
            data_folder,
            max_cabinet_number,
            max_keypair_number: max_cabinet_number * 10,
        }
    }
}
