pub(crate) mod extract;
mod handler;

pub use handler::{ServerState, router};

pub mod error {

    use domain::error::{DomainError, I18nError, Locale};

    /// Server api error
    pub struct InterfaceError(Locale, DomainError);

    impl InterfaceError {
        pub fn new(locale: Locale, domain_error: DomainError) -> Self {
            Self(locale, domain_error)
        }
    }

    impl From<(Locale, DomainError)> for InterfaceError {
        fn from(value: (Locale, DomainError)) -> Self {
            Self(value.0, value.1)
        }
    }

    impl axum::response::IntoResponse for InterfaceError {
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
                    "code": self.1.to_string(),
                    "message": self.1.i18n_message(self.0),
                })),
            )
                .into_response()
        }
    }
}
