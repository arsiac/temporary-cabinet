use crate::error::InterfaceError;
use crate::extract::AcceptLanguage;
use crate::handler::ServerState;
use axum::extract::State;

/// Crypto router
pub(crate) fn router() -> axum::Router<ServerState> {
    use axum::routing::get;
    axum::Router::new().route("/pk", get(public_key))
}

/// Get public key
#[axum::debug_handler]
pub(crate) async fn public_key(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
) -> Result<String, InterfaceError> {
    let service = infrastructure::service::crypto::create_sm2_crypto_service(
        state.connection,
        state.max_keypair_number,
    );
    let keypair = service
        .generate_keypair()
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(keypair.public_key)
}
