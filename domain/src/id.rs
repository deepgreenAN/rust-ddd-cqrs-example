use crate::error::{DomainError, GenericParseError};

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ジェネリックなUUID
#[derive(Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
#[cfg_attr(feature = "orm", derive(sea_orm_newtype::DeriveNewType))]
#[cfg_attr(feature = "orm", sea_orm_newtype(from_into = "Uuid", primary_key))]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> Id<T> {
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

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::generate()
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Id<T> {
        Id::from_uuid(id)
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(value: Id<T>) -> Self {
        value.to_uuid()
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Id<T>, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(Into::<GenericParseError>::into)?;
        Ok(id.into())
    }
}

impl<T> From<Id<T>> for String {
    fn from(value: Id<T>) -> Self {
        value.0.hyphenated().to_string()
    }
}

// -------------------------------------------------------------------------------------------------
// impl Dummy

#[cfg(any(test, feature = "fake"))]
impl<T> fake::Dummy<fake::Faker> for Id<T> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use fake::{Fake, Faker};

        let rand_id = Faker.fake_with_rng::<uuid::Uuid, R>(rng);
        Self(rand_id, PhantomData)
    }
}
