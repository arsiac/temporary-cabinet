pub mod cabinet;
pub mod crypto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    InternalError,
    CabinetError(cabinet::CabinetError),
    CryptoError(crypto::CryptoError),
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
