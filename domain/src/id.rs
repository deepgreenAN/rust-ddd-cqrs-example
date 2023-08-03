use crate::error::{DomainError, GenericParseError};

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ジェネリックなUUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
#[cfg_attr(feature = "orm", derive(sea_orm_newtype::DeriveNewType))]
#[cfg_attr(feature = "orm", sea_orm_newtype(from_into = "Uuid", primary_key))]
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
