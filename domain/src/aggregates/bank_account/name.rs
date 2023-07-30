use crate::error::DomainError;

use serde::{Deserialize, Serialize};

/// 口座名を表す型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountName {
    // 名
    first_name: String,
    // 姓
    last_name: String,
}

impl AccountName {
    pub fn from_primitives(first_name: String, last_name: String) -> Result<Self, DomainError> {
        Ok(Self {
            first_name,
            last_name,
        })
    }
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}

impl From<AccountName> for String {
    fn from(value: AccountName) -> Self {
        format!("{} {}", value.first_name, value.last_name)
    }
}

impl TryFrom<String> for AccountName {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let names = value.split_whitespace().collect::<Vec<_>>();
        if names.len() == 2 {
            Ok(Self {
                first_name: names[0].to_string(),
                last_name: names[1].to_string(),
            })
        } else {
            Err(DomainError::DomainParseError(
                "Invalid account name.".to_string(),
            ))
        }
    }
}

/// sea-ormのトレイトに関する部分(deriveマクロにできる)
#[cfg(feature = "orm")]
mod sea_orm {
    use super::AccountName;

    use sea_orm::TryGetable;
    use sea_query::{value::Nullable, ValueType};

    impl From<AccountName> for sea_query::Value {
        fn from(value: AccountName) -> Self {
            sea_query::Value::String(Some(Box::new(value.into())))
        }
    }

    impl TryGetable for AccountName {
        fn try_get_by<I: sea_orm::ColIdx>(
            res: &sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm::TryGetError> {
            let account_name_str: String = res.try_get_by(index)?;

            Ok(TryInto::<AccountName>::try_into(account_name_str)
                .map_err(|e| sea_orm::DbErr::Custom(e.to_string()))?)
        }
    }

    impl ValueType for AccountName {
        fn try_from(v: sea_query::Value) -> Result<Self, sea_query::ValueTypeErr> {
            match v {
                sea_query::Value::String(Some(account_name_str)) => {
                    TryInto::<AccountName>::try_into(*account_name_str)
                        .map_err(|_| sea_query::ValueTypeErr)
                }
                _ => Err(sea_query::ValueTypeErr),
            }
        }
        fn type_name() -> String {
            <String as ValueType>::type_name()
        }
        fn array_type() -> sea_query::ArrayType {
            <String as ValueType>::array_type()
        }
        fn column_type() -> sea_query::ColumnType {
            <String as ValueType>::column_type()
        }
    }

    impl Nullable for AccountName {
        fn null() -> sea_query::Value {
            <String as Nullable>::null()
        }
    }
}
