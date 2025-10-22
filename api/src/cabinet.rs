use crate::ServerState;
use axum::Json;
use axum::extract::State;
use domain::entity::cabinet::Cabinet;
use domain::error::DomainError;
use infrastructure::service::cabinet::create_cabinet_service;
use rand::seq::IndexedRandom;

/// Cabinet router
pub(crate) fn router() -> axum::Router<ServerState> {
    axum::Router::new().route("/random", axum::routing::get(random_unused))
}

/// Get a random unused cabinet
#[axum::debug_handler]
pub(crate) async fn random_unused(
    State(state): State<ServerState>,
) -> Result<Json<Cabinet>, DomainError> {
    let service = create_cabinet_service(&state.connection, &state.data_folder);
    let cabinets = service.list_unused_cabinets().await?;
    if cabinets.is_empty() {
        return Err(DomainError::NoEmptyCabinet);
    }
    let cabinet = cabinets.choose(&mut rand::rng());
    Ok(Json(cabinet.unwrap().clone()))
}
