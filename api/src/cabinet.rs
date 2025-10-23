use crate::ServerState;
use axum::extract::{Json, Path, State};
use domain::entity::cabinet::{Cabinet, CabinetItem, CabinetStatus};
use domain::error::DomainError;
use infrastructure::service::cabinet::create_cabinet_service;
use rand::seq::IndexedRandom;

/// Cabinet router
pub(crate) fn router() -> axum::Router<ServerState> {
    use axum::routing::get;
    axum::Router::new()
        .route("/random", get(random_unused))
        .route("/status", get(status))
        .route("/{cabinet_code}/items", get(items))
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

/// Get cabinet status
#[axum::debug_handler]
pub(crate) async fn status(
    State(state): State<ServerState>,
) -> Result<Json<CabinetStatus>, DomainError> {
    let service = create_cabinet_service(&state.connection, &state.data_folder);
    let status = service.status().await?;
    Ok(Json(status))
}

/// Get cabinet items
#[axum::debug_handler]
pub(crate) async fn items(
    State(state): State<ServerState>,
    Path(cabinet_code): Path<i64>,
) -> Result<Json<Vec<CabinetItem>>, DomainError> {
    let service = create_cabinet_service(&state.connection, &state.data_folder);
    let items = service.list_items_by_cabinet_code(cabinet_code).await?;
    Ok(Json(items))
}
