use std::str::FromStr;

use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::header::ACCEPT_LANGUAGE;
use axum::http::request::Parts;
use domain::error::Locale;

/// Extract user language from request header 'Accept-Language'
pub struct AcceptLanguage(pub Locale);

impl<S> FromRequestParts<S> for AcceptLanguage
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let al_value = parts.headers.get(ACCEPT_LANGUAGE);
        if al_value.is_none() {
            log::debug!("No 'Accept-Language' header found, use default locale");
            return Ok(Self(Locale::default()));
        }
        let al_header_value = al_value.unwrap();
        let al_str = al_header_value.to_str();
        if let Err(e) = al_str {
            log::error!("Invalid 'Accept-Language' header value '{al_header_value:?}': {e}");
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid 'Accept-Language' header value",
            ));
        }

        // zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6
        let al_str = al_str.unwrap();
        let languages: Vec<&str> = al_str
            .split(',')
            .map(|seg| {
                if seg.contains(";") {
                    seg.split(";").next().unwrap().trim()
                } else {
                    seg.trim()
                }
            })
            .collect();

        for language in languages {
            match Locale::from_str(language) {
                Ok(locale) => {
                    log::debug!("Pick language '{language}' from 'Accept-Language' header");
                    return Ok(Self(locale));
                }
                Err(e) => {
                    log::debug!(
                        "Language '{language}' in 'Accept-Language' header is not supported: {e}"
                    );
                }
            }
        }

        log::debug!("No supported language found in 'Accept-Language' header, use default locale");
        Ok(Self(Locale::default()))
    }
}
