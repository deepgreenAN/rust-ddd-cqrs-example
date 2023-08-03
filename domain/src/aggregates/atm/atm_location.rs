use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// ATMのある場所を示すエンティティ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "orm", derive(sea_orm_newtype::DeriveNewType))]
#[cfg_attr(feature = "orm", sea_orm_newtype(from_into = "String"))]
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
