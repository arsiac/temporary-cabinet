use std::borrow::Cow;

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

impl crate::error::I18nError for CabinetError {
    fn i18n_message(&self, locale: super::Locale) -> Cow<'static, str> {
        let locale_text = locale.to_string();
        match self {
            CabinetError::StatusNotSupport(status) => {
                rust_i18n::t!(
                    "error.cabinet.status_not_support",
                    locale = locale_text,
                    status = status
                )
            }
            CabinetError::NoAvailableCabinet => {
                rust_i18n::t!("error.cabinet.no_available_cabinet", locale = locale_text)
            }
            CabinetError::InvalidItemCategory(category) => {
                rust_i18n::t!(
                    "error.cabinet.invalid_item_category",
                    locale = locale_text,
                    category = category
                )
            }
            CabinetError::NotFound => {
                rust_i18n::t!("error.cabinet.not_found", locale = locale_text)
            }
            CabinetError::NoEmptyCabinet => {
                rust_i18n::t!("error.cabinet.no_empty_cabinet", locale = locale_text)
            }
            CabinetError::ItemContentMustNotEmpty => {
                rust_i18n::t!(
                    "error.cabinet.item_content_must_not_empty",
                    locale = locale_text
                )
            }
            CabinetError::CabinetItemNotFound => {
                rust_i18n::t!("error.cabinet.cabinet_item_not_found", locale = locale_text)
            }
            CabinetError::InvalidTextSize(size) => {
                rust_i18n::t!(
                    "error.cabinet.invalid_text_size",
                    locale = locale_text,
                    size = size
                )
            }
            CabinetError::InvalidFileSize(filename, size) => {
                rust_i18n::t!(
                    "error.cabinet.invalid_file_size",
                    locale = locale_text,
                    filename = filename,
                    size = size
                )
            }
            CabinetError::InvalidTotalSize(size) => {
                rust_i18n::t!(
                    "error.cabinet.invalid_total_size",
                    locale = locale_text,
                    size = size
                )
            }
            CabinetError::InvalidNumberString(text) => {
                rust_i18n::t!(
                    "error.cabinet.invalid_number_string",
                    locale = locale_text,
                    text = text
                )
            }
            CabinetError::InvalidHours(hours) => {
                rust_i18n::t!(
                    "error.cabinet.invalid_hours",
                    locale = locale_text,
                    hours = hours
                )
            }
            CabinetError::PasswordRequired => {
                rust_i18n::t!("error.cabinet.password_required", locale = locale_text)
            }
            CabinetError::ExpireTimeRequired => {
                rust_i18n::t!("error.cabinet.expire_time_required", locale = locale_text)
            }
            CabinetError::HoldTokenRequired => {
                rust_i18n::t!("error.cabinet.hold_token_required", locale = locale_text)
            }
            CabinetError::PublicKeyRequired => {
                rust_i18n::t!("error.cabinet.public_key_required", locale = locale_text)
            }
            CabinetError::NotYourHoldCabinet(code) => {
                rust_i18n::t!(
                    "error.cabinet.not_your_hold_cabinet",
                    locale = locale_text,
                    code = code
                )
            }
            CabinetError::InvalidPassword => {
                rust_i18n::t!("error.cabinet.invalid_password", locale = locale_text)
            }
            CabinetError::InvalidItemContent => {
                rust_i18n::t!("error.cabinet.invalid_item_content", locale = locale_text)
            }
            CabinetError::ItemNotSupportMode(mode) => {
                rust_i18n::t!(
                    "error.cabinet.item_not_support_mode",
                    locale = locale_text,
                    mode = mode
                )
            }
        }
    }
}

impl std::error::Error for CabinetError {}

impl std::fmt::Display for CabinetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl From<CabinetError> for super::DomainError {
    fn from(value: CabinetError) -> Self {
        super::DomainError::CabinetError(value)
    }
}
