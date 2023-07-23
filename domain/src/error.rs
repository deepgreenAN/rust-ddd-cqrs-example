use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------------------------------
// DomainError

/// ドメインに関するエラー
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum DomainError {
    /// プリミティブな型などからドメイン固有型へのパースの際のロジックのエラー．serdeのデシリアライズなどで起こる
    #[error("DomainError::DomainParseError: {0}")]
    DomainParseError(String),
}

// -------------------------------------------------------------------------------------------------
// GenericParseError

/// パース全般に関するジェネリックなエラー
#[derive(thiserror::Error, Debug, Clone)]
pub enum GenericParseError {
    /// UUIDのパースに関するエラー
    #[error("GenericParseError::ParseUuidError: {0}")]
    ParseUuidError(#[from] uuid::Error),
    /// EmailAddressのパースに関するエラー
    #[error("GenericParseError::ParseEmailAddressError: {0}")]
    ParseEmailAddressError(#[from] email_address::Error),
}

impl From<GenericParseError> for DomainError {
    fn from(value: GenericParseError) -> Self {
        DomainError::DomainParseError(value.to_string())
    }
}
