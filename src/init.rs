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
        eprintln!("Failed to initialize logger: {}", e);
    }
}

/// Connect to database and migrate
pub(crate) async fn initialize_database(data_dir: Option<String>) -> sea_orm::DatabaseConnection {
    use migration::Migrator;
    use sea_orm::{ConnectOptions, Database};

    let database_file = match data_dir {
        Some(path) => {
            let path = std::path::Path::new(&path);
            let path = path.join("db.sqlite");
            path.to_str().unwrap().to_string()
        }
        None => {
            let exe_path = std::env::current_exe();
            if let Err(e) = exe_path {
                eprintln!("Failed to get executable path: {}", e);
                std::process::exit(1);
            }
            let exe_path = exe_path.unwrap();
            let exe_path = exe_path.parent();
            if exe_path.is_none() {
                eprintln!("Failed to get executable directory");
                std::process::exit(1);
            }
            let exe_path = exe_path.unwrap();
            exe_path.join("db.sqlite").to_str().unwrap().to_string()
        }
    };

    let database_url = format!("sqlite://{}?mode=rwc", &database_file);
    let mut connect_opts = ConnectOptions::new(database_url);
    connect_opts
        .max_connections(10)
        .min_connections(2)
        .connect_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(10));
    log::debug!("Connecting to database '{}'...", &database_file);
    let connection = Database::connect(connect_opts).await;
    if let Err(e) = connection {
        eprintln!("Failed to connect to database: {}", e);
        std::process::exit(1);
    }
    let connection = connection.unwrap();

    log::debug!("Migrating database...");
    let migrate_result = Migrator::up(&connection, None).await;
    if let Err(e) = migrate_result {
        eprintln!("Failed to migrate database: {}", e);
        std::process::exit(1);
    }
    connection
}
