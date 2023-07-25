use ddd_cqrs_core::DomainEvent;

use serde::{Deserialize, Serialize};

/// Atmに関するイベント．
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AtmEvents {}

impl DomainEvent for AtmEvents {
    fn event_type(&self) -> String {
        "AtmEvents".to_string()
    }
    fn event_version() -> String {
        crate::global::EVENT_VERSION.to_string()
    }
}
