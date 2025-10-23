pub mod entity;
pub mod repository;
pub mod service;

pub mod error {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DomainError {
        InternalError,
        CabinetStatusNotSupport(i32),
        NoAvailableCabinet,
        InvalidCabinetItemCategory(String),
        CabinetNotFound,
        NoEmptyCabinet,
        CabinetItemContentMustNotEmpty,
        CabinetItemNotFound,
        InvalidTextSize(usize),
        InvalidFileSize(String, usize),
        InvalidTotalSize(usize),
        InvalidNumberString(String),
        InvalidHours(i32),
        CabinetPasswordRequired,
        CabinetExpireTimeRequired,
        CabinetHoldTokenRequired,
        NotYourHoldCabinet(i64),
    }

    impl axum::response::IntoResponse for DomainError {
        fn into_response(self) -> axum::response::Response {
            use axum::Json;
            use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
            let mut headers = HeaderMap::new();
            headers.append(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                Json(serde_json::json!({
                    "code": 500,
                    "message": format!("{:?}", &self),
                })),
            )
                .into_response()
        }
    }
}
