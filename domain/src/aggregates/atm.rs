mod atm_location;

use crate::error::DomainError;
use crate::events::atm_events::AtmEvents;
use crate::id::Id;
use atm_location::AtmLocation;

use ddd_cqrs_core::{Aggregate, DomainEventList};

use serde::{Deserialize, Serialize};

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
    events_list: DomainEventList<AtmEvents>,
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
}

impl Aggregate for Atm {
    type Event = AtmEvents;
    fn domain_events(&self) -> &DomainEventList<Self::Event> {
        &self.events_list
    }
    fn domain_events_mut(&mut self) -> &mut DomainEventList<Self::Event> {
        &mut self.events_list
    }
}
