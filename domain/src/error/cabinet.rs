#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CabinetError {
    StatusNotSupport(i32),
    NoAvailableCabinet,
    InvalidItemCategory(String),
    NotFound,
    NoEmptyCabinet,
    ItemContentMustNotEmpty,
    CabinetItemNotFound,
    InvalidTextSize(usize),
    InvalidFileSize(String, usize),
    InvalidTotalSize(usize),
    InvalidNumberString(String),
    InvalidHours(i32),
    PasswordRequired,
    ExpireTimeRequired,
    HoldTokenRequired,
    PublicKeyRequired,
    NotYourHoldCabinet(i64),
    InvalidPassword,
    InvalidItemContent,
    ItemNotSupportMode(String),
}

impl From<CabinetError> for super::DomainError {
    fn from(value: CabinetError) -> Self {
        super::DomainError::CabinetError(value)
    }
}
