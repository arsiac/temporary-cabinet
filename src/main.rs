mod arg;
mod init;

/// Application entrypoint
/// - Initialization Log (simple_logger)
/// - Build routes
/// - Start the server and listen for the ports in the configuration
#[tokio::main]
async fn main() {
    let args = arg::parse();
    init::initialize_logger(args.debug);
    let data_folder = init::initialize_data_folder(args.data_dir);
    let connection = init::initialize_database(&data_folder).await;
    let state = api::ServerState::new(connection, data_folder, args.cabinet_number);
    let serv_addr = format!("{}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(&serv_addr).await;
    if let Err(e) = listener {
        log::error!("Failed to bind to {}: {}", &serv_addr, e);
        return;
    }
    log::info!("Serving on {}", &serv_addr);
    axum::serve(listener.unwrap(), router(state)).await.unwrap();
}

/// Merge front-end and back-end routes and configure middleware
fn router(state: api::ServerState) -> axum::Router {
    use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

    let static_service = axum_embed::ServeEmbed::<web::WebAssets>::with_parameters(
        Some("/".to_string()),
        axum_embed::FallbackBehavior::Ok,
        Some("index.html".to_string()),
    );
    axum::Router::new()
        .nest("/api", api::router().with_state(state))
        .fallback_service(static_service)
        .layer(
            tower::ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
}
