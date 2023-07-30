use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// ATMのある場所を示すエンティティ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AtmLocation(String);

impl AtmLocation {
    pub fn new<'a, S: Into<Cow<'a, str>>>(location: S) -> Self {
        let s: Cow<'a, str> = location.into();
        Self(s.into_owned())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for AtmLocation {
    fn from(value: String) -> Self {
        AtmLocation(value)
    }
}

impl From<AtmLocation> for String {
    fn from(value: AtmLocation) -> Self {
        value.0
    }
}

/// sea-ormのトレイトに関する部分(deriveマクロにできる)
#[cfg(feature = "orm")]
mod sea_orm {
    use super::AtmLocation;

    use sea_orm::TryGetable;
    use sea_query::{value::Nullable, ValueType};

    impl From<AtmLocation> for sea_query::Value {
        fn from(value: AtmLocation) -> Self {
            sea_query::Value::String(Some(Box::new(value.into())))
        }
    }

    impl TryGetable for AtmLocation {
        fn try_get_by<I: sea_orm::ColIdx>(
            res: &sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm::TryGetError> {
            let atm_location_str: String = res.try_get_by(index)?;
            Ok(atm_location_str.into())
        }
    }

    impl ValueType for AtmLocation {
        fn try_from(v: sea_query::Value) -> Result<Self, sea_query::ValueTypeErr> {
            match v {
                sea_query::Value::String(Some(atm_location)) => Ok((*atm_location).into()),
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

    impl Nullable for AtmLocation {
        fn null() -> sea_query::Value {
            <String as Nullable>::null()
        }
    }
}
