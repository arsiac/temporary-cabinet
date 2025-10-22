pub mod entity;
pub mod repository;
pub mod service;

pub mod error {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DomainError {
        InternalError,
        InvalidCabinetItemCategory(String),
        CabinetNotFound,
        NoEmptyCabinet,
        CabinetItemContentMustNotEmpty,
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
