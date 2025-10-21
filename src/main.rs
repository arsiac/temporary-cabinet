mod arg;
mod init;

static INDEX_FILE: &str = "index.html";

#[tokio::main]
async fn main() {
    let args = arg::parse();
    init::initialize_logger(args.debug);

    let state = api::ServerState;
    let static_service = axum_embed::ServeEmbed::<web::WebAssets>::with_parameters(
        Some(INDEX_FILE.to_string()),
        axum_embed::FallbackBehavior::Redirect,
        Some(INDEX_FILE.to_string()),
        );
    let app = axum::Router::new()
    .nest("/api", api::router())
    .fallback_service(static_service)
    .with_state(state);

    let serv_addr = format!("{}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(&serv_addr).await;
    if let Err(e) = listener {
        log::error!("Failed to bind to {}: {}", &serv_addr, e);
        return;
    }
    log::info!("Serving on {}", &serv_addr);
    axum::serve(listener.unwrap(), app).await.unwrap();
}
