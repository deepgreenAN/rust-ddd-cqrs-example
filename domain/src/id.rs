use crate::error::{DomainError, GenericParseError};

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ジェネリックなUUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Id<T: Clone>(Uuid, PhantomData<T>);

impl<T: Clone> Id<T> {
    /// 新しくIDを生成
    pub fn generate() -> Id<T> {
        Id(Uuid::new_v4(), PhantomData)
    }
    /// Uuid -> Id<T>
    pub fn from_uuid(id: Uuid) -> Id<T> {
        Id(id, PhantomData)
    }
    /// Id<T> -> Uuid
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl<T: Clone> Default for Id<T> {
    fn default() -> Self {
        Self::generate()
    }
}

impl<T: Clone> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Id<T> {
        Id::from_uuid(id)
    }
}

impl<T: Clone> From<Id<T>> for Uuid {
    fn from(value: Id<T>) -> Self {
        value.to_uuid()
    }
}

impl<T: Clone> TryFrom<String> for Id<T> {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Id<T>, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(Into::<GenericParseError>::into)?;
        Ok(id.into())
    }
}

impl<T: Clone> From<Id<T>> for String {
    fn from(value: Id<T>) -> Self {
        value.0.hyphenated().to_string()
    }
}

/// sea-ormのトレイトに関する部分(deriveマクロにできる)
#[cfg(feature = "server")]
mod sea_orm {
    use super::Id;

    use sea_orm::{TryFromU64, TryGetable, Value};
    use sea_query::{value::Nullable, ValueType};
    use uuid::Uuid;

    impl<T: Clone> From<Id<T>> for Value {
        fn from(value: Id<T>) -> Self {
            Value::Uuid(Some(Box::new(value.into())))
        }
    }

    impl<T: Clone> TryGetable for Id<T> {
        fn try_get_by<I: sea_orm::ColIdx>(
            res: &sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm::TryGetError> {
            let id: Uuid = res.try_get_by(index)?;
            Ok(id.into())
        }
    }

    impl<T: Clone> ValueType for Id<T> {
        fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
            match v {
                Value::Uuid(Some(id)) => Ok((*id).into()),
                _ => Err(sea_query::ValueTypeErr),
            }
        }
        fn type_name() -> String {
            <Uuid as ValueType>::type_name()
        }
        fn array_type() -> sea_query::ArrayType {
            <Uuid as ValueType>::array_type()
        }
        fn column_type() -> sea_query::ColumnType {
            <Uuid as ValueType>::column_type()
        }
    }

    impl<T: Clone> Nullable for Id<T> {
        fn null() -> Value {
            <Uuid as Nullable>::null()
        }
    }

    impl<T: Clone> TryFromU64 for Id<T> {
        fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
            let id = <Uuid as TryFromU64>::try_from_u64(n)?;
            Ok(id.into())
        }
    }
}
