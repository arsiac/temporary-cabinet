use crate::ServerState;
use axum::extract::State;
use domain::error::DomainError;

/// Crypto router
pub(crate) fn router() -> axum::Router<ServerState> {
    use axum::routing::get;
    axum::Router::new().route("/pk", get(public_key))
}

/// Get public key
#[axum::debug_handler]
pub(crate) async fn public_key(State(state): State<ServerState>) -> Result<String, DomainError> {
    let service = infrastructure::service::crypto::create_sm2_crypto_service(state.connection);
    let keypair = service.generate_keypair().await?;
    Ok(keypair.public_key)
}
