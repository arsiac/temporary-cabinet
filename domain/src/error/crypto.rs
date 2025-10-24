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

impl From<CryptoError> for super::DomainError {
    fn from(value: CryptoError) -> Self {
        super::DomainError::CryptoError(value)
    }
}
