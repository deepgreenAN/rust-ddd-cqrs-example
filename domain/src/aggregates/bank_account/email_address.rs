use std::{fmt::Debug, str::FromStr};

use crate::error::{DomainError, GenericParseError};

use email_address::EmailAddress as InnerEmailAddress;
use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------------------------------

/// メールアドレスを表す型
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EmailAddress(InnerEmailAddress);

impl EmailAddress {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(EmailAddress(
            InnerEmailAddress::from_str(&value).map_err(Into::<GenericParseError>::into)?,
        ))
    }
}

impl From<EmailAddress> for String {
    fn from(value: EmailAddress) -> Self {
        value.0.to_string()
    }
}

impl Debug for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(EmailAddress))
            .field(&self.0)
            .finish()
    }
}

/// sea-ormのトレイトに関する部分(参照からの変換以外はderiveマクロにできる)
#[cfg(feature = "orm")]
mod sea_orm {
    use super::EmailAddress;

    use sea_orm::{
        sea_query::{value::Nullable, ValueType},
        TryGetable,
    };

    impl From<EmailAddress> for sea_orm::sea_query::Value {
        fn from(value: EmailAddress) -> Self {
            sea_orm::sea_query::Value::String(Some(Box::new(value.into())))
        }
    }

    impl From<&EmailAddress> for sea_orm::sea_query::Value {
        fn from(value: &EmailAddress) -> Self {
            value.as_str().into()
        }
    }

    impl TryGetable for EmailAddress {
        fn try_get_by<I: sea_orm::ColIdx>(
            res: &sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm::TryGetError> {
            let email_address_str: String = res.try_get_by(index)?;

            Ok(TryInto::<EmailAddress>::try_into(email_address_str)
                .map_err(|e| sea_orm::DbErr::Custom(e.to_string()))?)
        }
    }

    impl ValueType for EmailAddress {
        fn try_from(
            v: sea_orm::sea_query::Value,
        ) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
            match v {
                sea_orm::sea_query::Value::String(Some(email_address)) => {
                    TryInto::<EmailAddress>::try_into(*email_address)
                        .map_err(|_| sea_orm::sea_query::ValueTypeErr)
                }
                _ => Err(sea_orm::sea_query::ValueTypeErr),
            }
        }
        fn type_name() -> String {
            <String as ValueType>::type_name()
        }
        fn array_type() -> sea_orm::sea_query::ArrayType {
            <String as ValueType>::array_type()
        }
        fn column_type() -> sea_orm::sea_query::ColumnType {
            <String as ValueType>::column_type()
        }
    }

    impl Nullable for EmailAddress {
        fn null() -> sea_orm::sea_query::Value {
            <String as Nullable>::null()
        }
    }
}
