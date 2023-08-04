use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// ATMのある場所を示すエンティティ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "orm", derive(sea_orm_newtype::DeriveNewType))]
#[cfg_attr(feature = "orm", sea_orm_newtype(from_into = "String"))] // transparentでもよい
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

#[cfg(feature = "orm")]
impl From<&AtmLocation> for sea_orm::Value {
    fn from(value: &AtmLocation) -> Self {
        value.as_str().into()
    }
}

// -------------------------------------------------------------------------------------------------
// impl Dummy

#[cfg(any(test, feature = "fake"))]
impl fake::Dummy<fake::Faker> for AtmLocation {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use fake::Fake;

        let location: String = fake::faker::address::en::CityName().fake_with_rng(rng);
        location.into()
    }
}

// -------------------------------------------------------------------------------------------------
// test

#[cfg(test)]
mod test {
    use super::AtmLocation;

    #[cfg(feature = "orm")]
    mod orm_test {
        use super::AtmLocation;

        use fake::{Fake, Faker};
        use sea_orm::Value;

        #[test]
        fn ref_into_value_eq_into_value() {
            let atm_location = Faker.fake::<AtmLocation>();

            assert_eq!(
                Into::<Value>::into(&atm_location),
                Into::<Value>::into(atm_location)
            );
        }
    }
}
