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
