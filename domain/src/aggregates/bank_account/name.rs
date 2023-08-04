use crate::error::DomainError;

use serde::{Deserialize, Serialize};

/// 口座名を表す型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "orm", derive(sea_orm_newtype::DeriveNewType))]
#[cfg_attr(feature = "orm", sea_orm_newtype(try_from_into = "String"))]
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
    pub fn to_name_string(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl From<AccountName> for String {
    fn from(value: AccountName) -> Self {
        value.to_name_string()
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

#[cfg(feature = "orm")]
impl From<&AccountName> for sea_orm::Value {
    fn from(value: &AccountName) -> Self {
        value.to_name_string().into()
    }
}

#[cfg(any(test, feature = "fake"))]
impl fake::Dummy<fake::Faker> for AccountName {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use fake::Fake;

        let first_name = fake::faker::name::en::FirstName().fake_with_rng::<String, R>(rng);
        let last_name = fake::faker::name::en::LastName().fake_with_rng::<String, R>(rng);

        AccountName::from_primitives(first_name, last_name).unwrap()
    }
}
