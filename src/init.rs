
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
