use migration::MigratorTrait;

/// Initialize logger
pub(crate) fn initialize_logger(debug: bool) {
    use simple_logger::SimpleLogger;
    let result = SimpleLogger::new()
        .with_level(if debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .with_colors(true)
        .with_local_timestamps()
        .init();
    if let Err(e) = result {
        eprintln!("Failed to initialize logger: {e}");
    }
}

/// Initialize data folder
pub(crate) fn initialize_data_folder(data_folder: Option<String>) -> std::path::PathBuf {
    use std::path::PathBuf;
    let path = match data_folder {
        Some(path) => PathBuf::from(&path),
        None => {
            let exe_path = std::env::current_exe();
            if let Err(e) = exe_path {
                eprintln!("Failed to get executable path: {e}");
                std::process::exit(1);
            }
            let exe_path = exe_path.unwrap();
            let exe_path = exe_path.parent();
            if exe_path.is_none() {
                eprintln!("Failed to get executable directory");
                std::process::exit(1);
            }
            let exe_path = exe_path.unwrap();
            exe_path.to_path_buf()
        }
    };

    if !path.exists()
        && let Err(e) = std::fs::create_dir_all(&path)
    {
        eprintln!("Failed to create data directory: {e}");
        std::process::exit(1);
    }

    log::debug!("Using data directory '{}'", path.display());
    path
}

/// Connect to database and migrate
pub(crate) async fn initialize_database(
    data_folder: &std::path::Path,
) -> sea_orm::DatabaseConnection {
    use migration::Migrator;
    use sea_orm::{ConnectOptions, Database};

    let database_file = data_folder.join("db.sqlite");
    let database_url = format!("sqlite://{}?mode=rwc", database_file.display());
    let mut connect_opts = ConnectOptions::new(database_url);
    connect_opts
        .max_connections(10)
        .min_connections(2)
        .connect_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(10))
        .sqlx_logging_level(log::LevelFilter::Debug);
    log::debug!("Connecting to database '{}'...", database_file.display());
    let connection = Database::connect(connect_opts).await;
    if let Err(e) = connection {
        eprintln!("Failed to connect to database: {e}");
        std::process::exit(1);
    }
    let connection = connection.unwrap();

    log::debug!("Migrating database...");
    let migrate_result = Migrator::up(&connection, None).await;
    if let Err(e) = migrate_result {
        eprintln!("Failed to migrate database: {e}");
        std::process::exit(1);
    }
    connection
}

/// Initialize tickers
pub(crate) fn initialize_tickers(
    state: &interface::ServerState,
    cancel_token: &tokio_util::sync::CancellationToken,
) {
    use infrastructure::service::cabinet::create_cabinet_service;
    use infrastructure::service::crypto::create_sm2_crypto_service;
    use tokio::time::Duration;

    // Public key clean ticker
    log::info!("Starting public key clean ticker...");
    let mut public_key_clean_interval = tokio::time::interval(Duration::from_secs(5 * 60));
    let crypto_service =
        create_sm2_crypto_service(state.connection.clone(), state.max_keypair_number);
    let public_key_cancel_token = cancel_token.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = public_key_clean_interval.tick() => {
                    match crypto_service.delete_expired().await {
                        Ok(count) => {
                            log::info!("Deleted {} expired public keys", count);
                        }
                        Err(e) => {
                            log::error!("Failed to delete expired public keys: {e}");
                        }
                    }
                },

                _ = public_key_cancel_token.cancelled() => {
                    log::info!("Stopping public key clean ticker...");
                    break;
                }
            }
        }
    });

    // Cabinet clean ticker
    log::info!("Starting cabinet clean ticker...");
    let mut cabinet_clean_interval = tokio::time::interval(Duration::from_secs(5 * 60));
    let cabinet_service = create_cabinet_service(
        state.connection.clone(),
        &state.data_folder,
        state.max_cabinet_number,
    );
    let cabinet_clean_cancel_token = cancel_token.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cabinet_clean_interval.tick() => {
                     match cabinet_service.delete_expired().await {
                        Ok(count) => {
                            log::info!("Deleted {} expired cabinets", count);
                        }
                        Err(e) => {
                            log::error!("Failed to delete expired cabinets: {e}")
                        }
                    }
                },
                _ = cabinet_clean_cancel_token.cancelled() => {
                    log::info!("Stopping cabinet clean ticker...");
                    break;
                }

            }
        }
    });
}
