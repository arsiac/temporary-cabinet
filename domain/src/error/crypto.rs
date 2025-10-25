use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    KeypairGenerationFailed,
    InvalidSecretKey,
    InvalidPublicKey,
    DecryptionFailed,
    InvalidHexString,
    KeypairExpired,
    NotFound,
}

impl crate::error::I18nError for CryptoError {
    fn i18n_message(&self, locale: super::Locale) -> Cow<'static, str> {
        let locale_text = locale.to_string();
        match self {
            CryptoError::KeypairGenerationFailed => {
                rust_i18n::t!(
                    "error.crypto.keypair_generation_failed",
                    locale = locale_text
                )
            }
            CryptoError::InvalidSecretKey => {
                rust_i18n::t!("error.crypto.invalid_secret_key", locale = locale_text)
            }
            CryptoError::InvalidPublicKey => {
                rust_i18n::t!("error.crypto.invalid_public_key", locale = locale_text)
            }
            CryptoError::DecryptionFailed => {
                rust_i18n::t!("error.crypto.decryption_failed", locale = locale_text)
            }
            CryptoError::InvalidHexString => {
                rust_i18n::t!("error.crypto.invalid_hex_string", locale = locale_text)
            }
            CryptoError::KeypairExpired => {
                rust_i18n::t!("error.crypto.keypair_expired", locale = locale_text)
            }
            CryptoError::NotFound => {
                rust_i18n::t!("error.crypto.not_found", locale = locale_text)
            }
        }
    }
}

impl std::error::Error for CryptoError {}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl From<CryptoError> for super::DomainError {
    fn from(value: CryptoError) -> Self {
        super::DomainError::CryptoError(value)
    }
}
