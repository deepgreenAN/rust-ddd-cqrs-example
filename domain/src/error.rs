use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------------------------------
// DomainError

/// ドメインに関するエラー
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum DomainError {
    /// プリミティブな型などからドメイン固有型へのパースの際のロジックのエラー．serdeのデシリアライズなどで起こる
    #[error("DomainError::DomainParseError: {0}")]
    DomainParseError(String),
    /// BankAccountに関するエラー．BankAccountに関するロジックで起こる
    #[error("DomainError::BankAccountError: {0}")]
    BankAccountError(#[from] BankAccountError),
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

// -------------------------------------------------------------------------------------------------
// BankAccountError

/// BankAccountに関するエラー
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum BankAccountError {
    #[error(r#"
BankAccountError::DepositExceedLimitError: As the deposit amount is {amount}, the balance is {exceed_balance}, which exceeds the {limit} limit. 
    "#)]
    DepositExceedLimitError {
        limit: f64,
        amount: f64,
        exceed_balance: f64,
    },
    #[error(r#"
BankAccountError::WithdrawExceedBalanceError: Attempts to withdraw amounts {amount} in excess of the deposit balance {balance}.
    "#)]
    WithdrawExceedBalanceError { amount: f64, balance: f64 },
    #[error(r#"
BankAccountError::CheckExceedBalanceError: Attempts to write check amounts {amount} in excess of the deposit balance {balance}.
    "#)]
    CheckExceedBalanceError { amount: f64, balance: f64 },
}
