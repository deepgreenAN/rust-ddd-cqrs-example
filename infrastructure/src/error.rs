use domain::DomainError;

use serde::{Deserialize, Serialize};

/// インフラに関するエラー
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum InfraError {
    /// ドメインエラーから生成されたエラー
    #[error("InfraError::DomainError: {0}")]
    DomainError(#[from] DomainError),

    /// レコードが見つからなかったときのエラー
    #[error("InfraError::RecordNotFoundError: {0}")]
    RecordNotFoundError(String),

    /// その他のormに関するエラー
    #[error("InfraError::OtherDbError: {0}")]
    OtherDbError(String),
}

impl From<sea_orm::DbErr> for InfraError {
    fn from(value: sea_orm::DbErr) -> Self {
        use sea_orm::DbErr;

        match value {
            e @ DbErr::RecordNotFound(_) => Self::RecordNotFoundError(e.to_string()),
            e @ _ => Self::OtherDbError(e.to_string()),
        }
    }
}
