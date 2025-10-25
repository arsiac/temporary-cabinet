use std::borrow::Cow;

pub mod cabinet;
pub mod crypto;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Locale {
    ZhCn,
    ZhTw,
    ZhHk,
    #[default]
    EnUs,
    EnGb,
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Locale::ZhCn => write!(f, "zh-CN"),
            Locale::ZhTw => write!(f, "zh-TW"),
            Locale::ZhHk => write!(f, "zh-HK"),
            Locale::EnUs => write!(f, "en-US"),
            Locale::EnGb => write!(f, "en-GB"),
        }
    }
}

impl std::str::FromStr for Locale {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zh-CN" | "zh" => Ok(Locale::ZhCn),
            "zh-TW" => Ok(Locale::ZhTw),
            "zh-HK" => Ok(Locale::ZhHk),
            "en-US" | "en" => Ok(Locale::EnUs),
            "en-GB" => Ok(Locale::EnGb),
            _ => Err(DomainError::LocaleNotSupported),
        }
    }
}

pub trait I18nError: std::error::Error {
    /// Get the error message in the specified locale
    fn i18n_message(&self, locale: Locale) -> Cow<'static, str>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    InternalError,
    LocaleNotSupported,
    CabinetError(cabinet::CabinetError),
    CryptoError(crypto::CryptoError),
}

impl I18nError for DomainError {
    fn i18n_message(&self, locale: Locale) -> Cow<'static, str> {
        let locale_text = locale.to_string();
        match self {
            DomainError::InternalError => {
                rust_i18n::t!("error:internal_error", locale = &locale_text)
            }
            DomainError::LocaleNotSupported => {
                rust_i18n::t!("error:locale_not_supported", locale = &locale_text)
            }
            DomainError::CabinetError(e) => e.i18n_message(locale),
            DomainError::CryptoError(e) => e.i18n_message(locale),
        }
    }
}

impl std::error::Error for DomainError {}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::InternalError | DomainError::LocaleNotSupported => write!(f, "{self:?}"),
            DomainError::CabinetError(e) => write!(f, "Cabinet::{e}"),
            DomainError::CryptoError(e) => write!(f, "Crypto::{e}"),
        }
    }
}
