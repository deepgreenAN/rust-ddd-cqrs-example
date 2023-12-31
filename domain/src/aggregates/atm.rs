mod atm_location;

use crate::error::{AtmError, DomainError};
use crate::events::atm_events::AtmEvent;
use crate::id::Id;

pub use atm_location::AtmLocation;

use ddd_cqrs_core::{Aggregate, DomainEventList};

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// -------------------------------------------------------------------------------------------------
// AtmId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtmIdType;

pub type AtmId = Id<AtmIdType>;

// -------------------------------------------------------------------------------------------------
// Atm

/// Atmを表すアグリゲイト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Atm {
    id: AtmId,
    location: AtmLocation,
    total_cash: f64,
    #[serde(skip)]
    events_list: DomainEventList<AtmEvent>,
}

impl Atm {
    pub fn from_domains(location: AtmLocation, total_cash: f64) -> Self {
        Atm {
            id: AtmId::generate(),
            location,
            total_cash,
            events_list: DomainEventList::new(),
        }
    }
    pub fn from_primitives(location: String, total_cash: f64) -> Result<Self, DomainError> {
        Ok(Atm {
            id: AtmId::generate(),
            location: AtmLocation::new(location),
            total_cash,
            events_list: DomainEventList::new(),
        })
    }
    pub fn id(&self) -> AtmId {
        self.id
    }
    pub fn location(&self) -> &AtmLocation {
        &self.location
    }
    pub fn total_cash(&self) -> f64 {
        self.total_cash
    }
    // -------------------------------------------------------------------------------------------------
    // 以下がドメインロジック

    /// Atmに現金をチャージ
    pub fn charge_cash(&mut self, amount: f64) -> Result<(), DomainError> {
        self.total_cash += amount;
        Ok(())
    }
    /// Atmから現金を引き出す
    pub fn withdraw(&mut self, amount: f64) -> Result<(), DomainError> {
        if amount < self.total_cash {
            self.total_cash -= amount;
            Ok(())
        } else {
            Err(AtmError::CannotWithdrawError {
                total_cash: self.total_cash,
                withdraw_amount: amount,
            }
            .into())
        }
    }
}

impl Aggregate for Atm {
    type Event = AtmEvent;
    type IntoId = AtmId;
    fn id(&self) -> Self::IntoId {
        self.id
    }
    fn domain_events(&self) -> &DomainEventList<Self::Event> {
        &self.events_list
    }
    fn domain_events_mut(&mut self) -> &mut DomainEventList<Self::Event> {
        &mut self.events_list
    }
}

// -------------------------------------------------------------------------------------------------
// sea_orm用Model

#[cfg(feature = "orm")]
pub mod orm {
    use super::*;
    use sea_orm::entity::prelude::*;
    use serde::Serialize;

    /// Atmに対するORMモデル．
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "atm")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, unique)]
        id: AtmId,
        location: AtmLocation,
        total_cash: f64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

    /// 双方のFromを実装することで，フィールド対応のバグを減らすことができる．
    impl From<Model> for Atm {
        fn from(value: Model) -> Self {
            let Model {
                id,
                location,
                total_cash,
            } = value;
            Self {
                id,
                location,
                total_cash,
                events_list: Default::default(),
            }
        }
    }

    impl From<Atm> for Model {
        fn from(value: Atm) -> Self {
            let Atm {
                id,
                location,
                total_cash,
                events_list: _,
            } = value;
            Self {
                id,
                location,
                total_cash,
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// impl Dummy

#[cfg(any(test, feature = "fake"))]
impl fake::Dummy<fake::Faker> for Atm {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use fake::{Fake, Faker};

        Self {
            id: Faker.fake_with_rng(rng),
            location: Faker.fake_with_rng(rng),
            total_cash: Faker.fake_with_rng(rng),
            events_list: DomainEventList::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// test

#[cfg(test)]
mod test {
    use super::{orm, Atm};

    #[cfg(feature = "orm")]
    mod orm_test {
        use super::{orm, Atm};
        use fake::{Fake, Faker};

        #[test]
        fn aggregate_model_serde() {
            let atm: Atm = Faker.fake();

            let json_from_model =
                serde_json::to_string(&Into::<orm::Model>::into(atm.clone())).unwrap();

            assert_eq!(atm, serde_json::from_str(&json_from_model).unwrap())
        }
    }
}
