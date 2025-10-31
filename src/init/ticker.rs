
/// Initialize public key clean ticker
pub(crate) fn initialize_public_key_clean_ticker(
    state: &interface::ServerState,
    cancel_token: &tokio_util::sync::CancellationToken,
) {
    use infrastructure::service::crypto::create_sm2_crypto_service;
    use tokio::time::Duration;

    // Public key clean ticker
    log::info!("Starting public key clean ticker...");
    let mut interval = tokio::time::interval(Duration::from_secs(5 * 60));
    let crypto_service =
        create_sm2_crypto_service(state.connection.clone(), state.max_keypair_number);
    let cancel_token = cancel_token.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match crypto_service.delete_expired().await {
                        Ok(count) => {
                            if log::log_enabled!(log::Level::Debug) {
                                log::debug!("Deleted {} expired public keys", count);
                            } else if count > 0 {
                                log::info!("Deleted {} expired public keys", count);
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to delete expired public keys: {e}");
                        }
                    }
                },

                _ = cancel_token.cancelled() => {
                    log::info!("Stopping public key clean ticker...");
                    break;
                }
            }
        }
    });
}


/// Initialize cabinets clean ticker
pub(crate) fn initialize_cabinet_clean_ticker(
    state: &interface::ServerState,
    cancel_token: &tokio_util::sync::CancellationToken,
) {
    use infrastructure::service::cabinet::create_cabinet_service;
    use tokio::time::Duration;

    log::info!("Starting cabinet clean ticker...");
    let mut interval = tokio::time::interval(Duration::from_secs(5 * 60));
    let cabinet_service = create_cabinet_service(
        state.connection.clone(),
        &state.data_folder,
        state.max_cabinet_number,
    );
    let cancel_token = cancel_token.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = interval.tick() => {
                     match cabinet_service.delete_expired().await {
                        Ok(count) => {
                            if log::log_enabled!(log::Level::Debug) {
                                log::debug!("Deleted {} expired cabinets", count);
                            } else if count > 0 {
                                log::info!("Deleted {} expired cabinets", count);
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to delete expired cabinets: {e}")
                        }
                    }
                },
                _ = cancel_token.cancelled() => {
                    log::info!("Stopping cabinet clean ticker...");
                    break;
                }

            }
        }
    });
}