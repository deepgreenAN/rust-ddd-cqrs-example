use std::{fmt::Debug, str::FromStr};

use crate::error::{DomainError, GenericParseError};

use email_address::EmailAddress as InnerEmailAddress;
use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------------------------------

/// メールアドレスを表す型
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
#[cfg_attr(feature = "orm", derive(sea_orm_newtype::DeriveNewType))]
#[cfg_attr(feature = "orm", sea_orm_newtype(try_from_into = "String"))]
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

#[cfg(feature = "orm")]
impl From<&EmailAddress> for sea_orm::Value {
    fn from(value: &EmailAddress) -> Self {
        value.as_str().into()
    }
}

// -------------------------------------------------------------------------------------------------
// impl Dummy

#[cfg(any(test, feature = "fake"))]
impl fake::Dummy<fake::Faker> for EmailAddress {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use fake::Fake;

        let email_address = fake::faker::internet::en::FreeEmail().fake_with_rng::<String, R>(rng);
        email_address.try_into().unwrap()
    }
}
