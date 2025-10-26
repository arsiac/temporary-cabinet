use crate::error::InterfaceError;
use crate::extract::AcceptLanguage;
use crate::handler::ServerState;
use axum::extract::{Json, Path, Query, State};
use chrono::{DateTime, Local};
use domain::entity::cabinet::{
    Cabinet, CabinetItem, CabinetItemCategory, CabinetStatus, CabinetUsage,
};
use domain::error::DomainError;
use domain::error::cabinet::CabinetError;
use infrastructure::service::cabinet::create_cabinet_service;
use infrastructure::service::crypto::create_sm2_crypto_service;

/// Cabinet router
pub(crate) fn router() -> axum::Router<ServerState> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/apply", post(apply))
        .route("/usage", get(usage))
        .route(
            "/{cabinet_code}",
            get(get_by_code).post(save).delete(delete_cabinet),
        )
        .route("/{cabinet_code}/items", post(items))
        .route(
            "/{cabinet_code}/item/{item_id}/content",
            post(get_item_content),
        )
}

/// Apply for a cabinet
#[axum::debug_handler]
pub(crate) async fn apply(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
) -> Result<Json<CabinetView>, InterfaceError> {
    let service = create_cabinet_service(
        state.connection,
        &state.data_folder,
        state.max_cabinet_number,
    );
    let cabinet = service
        .apply()
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(Json(CabinetView::from(cabinet)))
}

/// Get cabinet usage status
#[axum::debug_handler]
pub(crate) async fn usage(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
) -> Result<Json<CabinetUsage>, InterfaceError> {
    let service = create_cabinet_service(
        state.connection,
        &state.data_folder,
        state.max_cabinet_number,
    );
    let status = service
        .usage()
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(Json(status))
}

/// Get cabinet by code
#[axum::debug_handler]
pub(crate) async fn get_by_code(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
    Path(cabinet_code): Path<i64>,
) -> Result<Json<CabinetView>, InterfaceError> {
    let service = create_cabinet_service(
        state.connection,
        &state.data_folder,
        state.max_cabinet_number,
    );
    let cabinet = service
        .get_nonnone_by_code(cabinet_code)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(Json(CabinetView::from(cabinet)))
}

/// Save cabinet items and update cabinet status to `Occupied`
#[axum::debug_handler]
pub(crate) async fn save(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
    Path(cabinet_code): Path<i64>,
    mut multipart: axum::extract::Multipart,
) -> Result<Json<CabinetView>, InterfaceError> {
    const MAX_MSG_SIZE: usize = 2000;
    const MAX_FILE_SIZE: usize = 2 * 1024 * 1024;
    const MAX_TOTAL_SIZE: usize = 10 * 1024 * 1024;

    let mut public_key = None;
    let mut cabinet = Cabinet::new(cabinet_code, None, None, CabinetStatus::Hold, None, None);
    let mut items = Vec::new();
    let mut order = 1;
    let mut total_size = 0;
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => {
                log::error!("Failed to read multipart field: {e:?}");
                return Err(InterfaceError::new(language, DomainError::InternalError));
            }
        };

        let field_name = field.name();
        if field_name.is_none() {
            continue;
        }
        let field_name = field_name.unwrap().to_string();
        log::debug!("Cabinet '{cabinet_code}' save with field '{field_name}'.");
        match field_name.as_str() {
            "password" => match field.text().await {
                Ok(password) => {
                    cabinet.password = Some(password);
                }
                Err(e) => {
                    log::error!("Failed to read password: {e:?}");

                    return Err(InterfaceError::new(language, DomainError::InternalError));
                }
            },
            "public_key" => match field.text().await {
                Ok(pk) => {
                    public_key = Some(pk);
                }
                Err(e) => {
                    log::error!("Failed to read pk: {e:?}");
                    return Err(InterfaceError::new(language, DomainError::InternalError));
                }
            },

            "hours" => match field.text().await {
                Ok(text) => {
                    let hour = text.parse::<i32>().map_err(|e| {
                        log::error!("Failed to read hours: {e:?}");
                        InterfaceError::new(
                            language,
                            CabinetError::InvalidNumberString(text).into(),
                        )
                    })?;
                    if !(0..=24).contains(&hour) {
                        return Err(InterfaceError::new(language, DomainError::InternalError));
                    }
                    cabinet.expire_at = Some(Local::now() + chrono::Duration::hours(hour as i64));
                }
                Err(e) => {
                    log::error!("Failed to read hours: {e:?}");
                    return Err(InterfaceError::new(language, DomainError::InternalError));
                }
            },
            "hold_token" => match field.text().await {
                Ok(text) => {
                    cabinet.hold_token = Some(text);
                }
                Err(e) => {
                    log::error!("Failed to read hold_token: {e:?}");
                    return Err(InterfaceError::new(language, DomainError::InternalError));
                }
            },
            "message" => {
                let bytes = field.text().await.unwrap().into_bytes();
                let text_size = bytes.len();
                if text_size > MAX_MSG_SIZE {
                    return Err(InterfaceError::new(
                        language,
                        CabinetError::InvalidTextSize(text_size).into(),
                    ))?;
                }

                let text_item = CabinetItem::new(
                    cabinet_code * 10 + order,
                    cabinet_code,
                    CabinetItemCategory::Text,
                    String::from("message.txt"),
                    bytes,
                    order as i32,
                );
                log::debug!(
                    "Cabinet '{}' add message item '{}' ({}).",
                    cabinet_code,
                    &text_item.name,
                    text_size
                );
                items.push(text_item);
                order += 1;
                total_size += text_size;
            }
            "files" => {
                let filename = field.file_name().unwrap_or("unknown");
                let filename = filename.rsplit('/').next().unwrap_or("unknown");
                let filename = filename.to_string();
                let bytes = field.bytes().await.map_err(|e| {
                    log::error!("Failed to read file '{filename}': {e:?}");
                    InterfaceError::new(language, DomainError::InternalError)
                })?;
                let file_size = bytes.len();
                if file_size > MAX_FILE_SIZE {
                    return Err(InterfaceError::new(
                        language,
                        CabinetError::InvalidFileSize(filename, file_size).into(),
                    ))?;
                }
                let file_item = CabinetItem::new(
                    cabinet_code * 10 + order,
                    cabinet_code,
                    CabinetItemCategory::File,
                    filename,
                    bytes.to_vec(),
                    order as i32,
                );
                log::debug!(
                    "Cabinet '{}' add file item '{}' ({}).",
                    cabinet_code,
                    &file_item.name,
                    file_size
                );
                items.push(file_item);
                order += 1;
                total_size += file_size;
            }
            _ => {
                log::warn!("Unknown field: {field_name}");
            }
        }
    }

    if total_size > MAX_TOTAL_SIZE {
        return Err(InterfaceError::new(
            language,
            CabinetError::InvalidTotalSize(total_size).into(),
        ));
    }

    // Set expire_at if not set
    if cabinet.expire_at.is_none() {
        cabinet.expire_at = Some(Local::now() + chrono::Duration::hours(1));
    }

    if public_key.is_none() {
        return Err(InterfaceError::new(
            language,
            CabinetError::PublicKeyRequired.into(),
        ));
    }

    if cabinet.password.is_none() {
        return Err(InterfaceError::new(
            language,
            CabinetError::PasswordRequired.into(),
        ));
    }

    let transaction = infrastructure::database::begin_transaction(&state.connection)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    let public_key = public_key.unwrap();
    let crypto_service =
        create_sm2_crypto_service(state.connection.clone(), state.max_keypair_number);
    let keypair = crypto_service
        .get_effective_by_public_key(&public_key)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    let secret_key = domain::service::crypto::hex2sk(&keypair.secret_key)
        .map_err(|e| InterfaceError::new(language, e.into()))?;
    let password = domain::service::crypto::decrypt_hex_to_plaintext(
        &secret_key,
        cabinet.password.as_ref().unwrap(),
    )
    .map_err(|e| InterfaceError::new(language, e.into()))?;
    cabinet.password = Some(password);
    crypto_service
        .delete_by_id(keypair.id.unwrap())
        .await
        .map_err(|e| InterfaceError::new(language, e))?;

    let cabinet_service = create_cabinet_service(
        state.connection,
        &state.data_folder,
        state.max_cabinet_number,
    );
    let cabinet = cabinet_service
        .save(cabinet, items)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    transaction
        .commit()
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(Json(CabinetView::from(cabinet)))
}

/// Delete cabinet and items
#[axum::debug_handler]
pub(crate) async fn delete_cabinet(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
    Path(cabinet_code): Path<i64>,
    Json(credential): Json<CabinetCredential>,
) -> Result<Json<bool>, InterfaceError> {
    let transaction = infrastructure::database::begin_transaction(&state.connection)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    let _ = validate_cabinet_permission(&state, cabinet_code, credential)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    let cabinet_service = create_cabinet_service(
        state.connection.clone(),
        &state.data_folder,
        state.max_cabinet_number,
    );

    cabinet_service
        .delete_by_code(cabinet_code)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    transaction
        .commit()
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(Json(true))
}

/// Get cabinet items
#[axum::debug_handler]
pub(crate) async fn items(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
    Path(cabinet_code): Path<i64>,
    Json(credential): Json<CabinetCredential>,
) -> Result<Json<Vec<CabinetItemView>>, InterfaceError> {
    let _ = validate_cabinet_permission(&state, cabinet_code, credential)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    let cabinet_service = create_cabinet_service(
        state.connection.clone(),
        &state.data_folder,
        state.max_cabinet_number,
    );
    let items = cabinet_service
        .list_items_by_cabinet_code(cabinet_code)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    Ok(Json(
        items
            .into_iter()
            .map(CabinetItemView::from)
            .collect::<Vec<_>>(),
    ))
}

/// Get cabinet item content
#[axum::debug_handler]
pub(crate) async fn get_item_content(
    State(state): State<ServerState>,
    AcceptLanguage(language): AcceptLanguage,
    Path((cabinet_code, item_id)): Path<(i64, i64)>,
    Query(params): Query<CabinetItemContentParams>,
    Json(credential): Json<CabinetCredential>,
) -> Result<axum::response::Response, InterfaceError> {
    use axum::body::Body;
    use axum::http::header::HeaderValue;
    use axum::response::Response;
    let _ = validate_cabinet_permission(&state, cabinet_code, credential)
        .await
        .map_err(|e| InterfaceError::new(language, e))?;
    let cabinet_service = create_cabinet_service(
        state.connection,
        &state.data_folder,
        state.max_cabinet_number,
    );
    // Get item
    let item = cabinet_service
        .get_item_by_id(item_id, true)
        .await
        .map_err(|e| InterfaceError::new(language, e))?
        .ok_or(InterfaceError::new(
            language,
            CabinetError::CabinetItemNotFound.into(),
        ))?;

    let content = item.content.ok_or(InterfaceError::new(
        language,
        CabinetError::InvalidItemContent.into(),
    ))?;
    match params.mode.as_str() {
        "text" => {
            if item.category != CabinetItemCategory::Text {
                return Err(InterfaceError::new(
                    language,
                    CabinetError::ItemNotSupportMode(params.mode).into(),
                ))?;
            }
            Ok(Response::builder()
                .header(
                    axum::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain"),
                )
                .body(Body::from(content))
                .unwrap())
        }
        "file" => Ok(Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .header(
                axum::http::header::CONTENT_DISPOSITION,
                format!("attachment; filename={}", item.name),
            )
            .body(Body::from(content))
            .unwrap()),
        _ => Err(InterfaceError::new(
            language,
            CabinetError::ItemNotSupportMode(params.mode).into(),
        ))?,
    }
}

/// Validate cabinet permission and return cabinet
async fn validate_cabinet_permission(
    state: &ServerState,
    cabinet_code: i64,
    credential: CabinetCredential,
) -> Result<Cabinet, DomainError> {
    let cabinet_service = create_cabinet_service(
        state.connection.clone(),
        &state.data_folder,
        state.max_cabinet_number,
    );
    let cabinet = cabinet_service.get_by_code(cabinet_code).await?;
    if cabinet.is_none() {
        return Err(CabinetError::NotFound)?;
    }
    let cabinet = cabinet.unwrap();

    // Decrypt password
    let crypto_service =
        create_sm2_crypto_service(state.connection.clone(), state.max_keypair_number);
    let keypair = crypto_service
        .get_effective_by_public_key(&credential.public_key)
        .await?;
    crypto_service.delete_by_id(keypair.id.unwrap()).await?;
    let secret_key = domain::service::crypto::hex2sk(&keypair.secret_key)?;
    let password =
        domain::service::crypto::decrypt_hex_to_plaintext(&secret_key, &credential.password)?;

    // Validate password
    if Some(password) != cabinet.password {
        return Err(CabinetError::InvalidPassword)?;
    }
    Ok(cabinet)
}

/// Cabinet struct for view
#[derive(Debug, serde::Serialize)]
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

#[derive(Debug, serde::Serialize)]
pub struct CabinetItemView {
    pub id: i64,
    pub cabinet_code: i64,
    pub category: CabinetItemCategory,
    pub name: String,
    pub size: i64,
    pub sort_order: i32,
}

impl From<CabinetItem> for CabinetItemView {
    fn from(value: CabinetItem) -> Self {
        Self {
            id: value.id,
            cabinet_code: value.cabinet_code,
            category: value.category,
            name: value.name,
            size: value.size,
            sort_order: value.sort_order,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CabinetCredential {
    pub public_key: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CabinetItemContentParams {
    pub mode: String,
}
