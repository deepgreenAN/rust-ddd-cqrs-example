use domain::DomainError;

use serde::{Deserialize, Serialize};

/// アプリケーション層に共通するエラー
#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum ApplicationError {
    /// ドメインに由来するエラー
    #[error("ApplicationError::DomainError: {0}")]
    DomainError(#[from] DomainError),

    /// レコードが見つからないときのエラー
    #[error("ApplicationError::RecordNotFoundError: {0}")]
    RecordNotFound(String),

    /// その他のインフラに関するエラー
    #[error("ApplicationError::OtherInfraError: {0}")]
    OtherInfraError(String),

    /// APIハンドラのJsonRejectionエラー
    #[error("ApplicationError::JsonRejectionError: {0}")]
    JsonRejectionError(String),

    /// serdeのシリアライズ・デシリアライズに関するエラー
    #[error("ApplicationError::SerdeError: {0}")]
    SerdeError(String),

    /// リクエストに関するエラー
    #[error("ApplicationError::FetchError: {0}")]
    FetchError(String),
}

#[cfg(feature = "server")]
mod server {
    use super::ApplicationError;
    use axum::{http::StatusCode, response::IntoResponse, Json};
    use infrastructure::InfraError;

    // -------------------------------------------------------------------------------------------------
    // From<> for ApplicationError

    impl From<InfraError> for ApplicationError {
        fn from(value: InfraError) -> Self {
            match value {
                e @ InfraError::RecordNotFoundError(_) => Self::RecordNotFound(e.to_string()),
                e @ _ => Self::OtherInfraError(e.to_string()),
            }
        }
    }

    impl From<axum::extract::rejection::JsonRejection> for ApplicationError {
        fn from(json_rejection_error: axum::extract::rejection::JsonRejection) -> Self {
            ApplicationError::JsonRejectionError(json_rejection_error.to_string())
        }
    }

    // -------------------------------------------------------------------------------------------------
    // IntoResponse(StatusCode, ApplicationError)

    impl IntoResponse for ApplicationError {
        fn into_response(self) -> axum::response::Response {
            match self {
                Self::JsonRejectionError(_) => {
                    (StatusCode::BAD_REQUEST, Json(self)).into_response()
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response(),
            }
        }
    }
}

#[cfg(feature = "frontend")]
mod frontend {
    use super::ApplicationError;

    // -------------------------------------------------------------------------------------------------
    // From<> for ApplicationError

    impl From<reqwest::Error> for ApplicationError {
        fn from(value: reqwest::Error) -> Self {
            // jsonのデコードに関するエラー
            if value.is_decode() {
                ApplicationError::SerdeError(value.to_string())
            } else {
                // その他のエラー
                ApplicationError::FetchError(value.to_string())
            }
        }
    }
}
