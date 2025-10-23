use crate::ServerState;
use axum::extract::{Json, Path, State};
use chrono::{DateTime, Local};
use domain::entity::cabinet::{
    Cabinet, CabinetItem, CabinetItemCategory, CabinetStatus, CabinetUsage,
};
use domain::error::DomainError;
use infrastructure::service::cabinet::create_cabinet_service;

/// Cabinet router
pub(crate) fn router() -> axum::Router<ServerState> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/apply", post(apply))
        .route("/usage", get(usage))
        .route("/{cabinet_code}", get(get_by_code).post(save))
        .route("/{cabinet_code}/items", get(items))
}

/// Apply for a cabinet
#[axum::debug_handler]
pub(crate) async fn apply(
    State(state): State<ServerState>,
) -> Result<Json<CabinetView>, DomainError> {
    let service =
        create_cabinet_service(&state.connection, &state.data_folder, state.cabinet_number);
    let cabinet = service.apply().await?;
    Ok(Json(CabinetView::from(cabinet)))
}

/// Get cabinet usage status
#[axum::debug_handler]
pub(crate) async fn usage(
    State(state): State<ServerState>,
) -> Result<Json<CabinetUsage>, DomainError> {
    let service =
        create_cabinet_service(&state.connection, &state.data_folder, state.cabinet_number);
    let status = service.usage().await?;
    Ok(Json(status))
}

#[axum::debug_handler]
pub(crate) async fn get_by_code(
    State(state): State<ServerState>,
    Path(cabinet_code): Path<i64>,
) -> Result<Json<CabinetView>, DomainError> {
    let service =
        create_cabinet_service(&state.connection, &state.data_folder, state.cabinet_number);
    let cabinet = service.get_by_code(cabinet_code).await?;
    Ok(Json(CabinetView::from(cabinet)))
}

#[axum::debug_handler]
pub(crate) async fn save(
    State(state): State<ServerState>,
    Path(cabinet_code): Path<i64>,
    mut multipart: axum::extract::Multipart,
) -> Result<(), DomainError> {
    const MAX_MSG_SIZE: usize = 2000;
    const MAX_FILE_SIZE: usize = 2 * 1024 * 1024;
    const MAX_TOTAL_SIZE: usize = 10 * 1024 * 1024;

    let mut cabinet = Cabinet::new(cabinet_code, None, None, CabinetStatus::Hold, None, None);
    let mut items = Vec::new();
    let mut order = 1;
    let mut total_size = 0;
    while let Ok(Some(mut field)) = multipart.next_field().await {
        if field.name() == Some("password") {
            match field.text().await {
                Ok(password) => {
                    cabinet.password = Some(password);
                }
                Err(e) => {
                    log::error!("Failed to read password: {:?}", e);
                    return Err(DomainError::InternalError);
                }
            }
            continue;
        }
        if field.name() == Some("hours") {
            match field.text().await {
                Ok(text) => {
                    let hour = text.parse::<i32>().map_err(|e| {
                        log::error!("Failed to read hours: {:?}", e);
                        DomainError::InvalidNumberString(text)
                    })?;
                    if !(0..=24).contains(&hour) {
                        return Err(DomainError::InvalidHours(hour));
                    }
                    cabinet.expire_at = Some(Local::now() + chrono::Duration::hours(hour as i64));
                }
                Err(e) => {
                    log::error!("Failed to read hours: {:?}", e);
                    return Err(DomainError::InternalError);
                }
            }
            continue;
        }
        if field.name() == Some("hold_token") {
            match field.text().await {
                Ok(text) => {
                    cabinet.hold_token = Some(text);
                }
                Err(e) => {
                    log::error!("Failed to read hold_token: {:?}", e);
                    return Err(DomainError::InternalError);
                }
            }
            continue;
        }
        if field.name() == Some("message") {
            let bytes = field.text().await.unwrap().into_bytes();
            let text_size = bytes.len();
            if text_size > MAX_MSG_SIZE {
                return Err(DomainError::InvalidTextSize(text_size));
            }

            let text_item = CabinetItem::new(
                cabinet_code * 10 + order,
                cabinet_code,
                CabinetItemCategory::Text,
                String::from("message.txt"),
                Some(bytes),
                order as i32,
            );
            items.push(text_item);
            order += 1;
            total_size += text_size;
            continue;
        }

        if field.name() == Some("files") {
            let Some(filename) = field.file_name() else {
                continue;
            };

            let filename = filename.rsplit('/').next().unwrap_or("unknown");
            let filename = filename.to_string();
            let bytes = field.chunk().await.map_err(|e| {
                log::error!("Failed to read chunk of file '{}': {:?}", filename, e);
                DomainError::InternalError
            })?;

            if let Some(bytes) = bytes {
                let file_size = bytes.len();
                if file_size > MAX_FILE_SIZE {
                    return Err(DomainError::InvalidFileSize(filename, file_size));
                }
                let file_item = CabinetItem::new(
                    cabinet_code * 10 + order,
                    cabinet_code,
                    CabinetItemCategory::File,
                    filename,
                    Some(bytes.to_vec()),
                    order as i32,
                );
                items.push(file_item);
                order += 1;
                total_size += file_size;
            }
        }
    }

    if total_size > MAX_TOTAL_SIZE {
        return Err(DomainError::InvalidTotalSize(total_size));
    }

    // Set expire_at if not set
    if cabinet.expire_at.is_none() {
        cabinet.expire_at = Some(Local::now() + chrono::Duration::hours(1));
    }

    let transaction = infrastructure::database::begin_transaction(&state.connection).await?;
    let service =
        create_cabinet_service(&state.connection, &state.data_folder, state.cabinet_number);
    if let Err(e) = service.save(cabinet, items).await {
        transaction.rollback().await?;
        return Err(e);
    } else {
        transaction.commit().await?;
    }
    Ok(())
}

/// Get cabinet items
#[axum::debug_handler]
pub(crate) async fn items(
    State(state): State<ServerState>,
    Path(cabinet_code): Path<i64>,
) -> Result<Json<Vec<CabinetItem>>, DomainError> {
    let service =
        create_cabinet_service(&state.connection, &state.data_folder, state.cabinet_number);
    let items = service.list_items_by_cabinet_code(cabinet_code).await?;
    Ok(Json(items))
}

/// Cabinet struct for view
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CabinetView {
    pub code: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: CabinetStatus,
    pub hold_token: Option<String>,
    pub expire_at: Option<DateTime<Local>>,
}

impl From<Cabinet> for CabinetView {
    fn from(value: Cabinet) -> Self {
        Self {
            code: value.code,
            name: value.name,
            description: value.description,
            status: value.status,
            hold_token: value.hold_token,
            expire_at: value.expire_at,
        }
    }
}
