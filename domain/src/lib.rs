pub mod aggregates;
pub mod commands;
pub mod events;
pub mod global;
pub mod services;

mod error;
mod id;
mod macros;

pub use error::DomainError;
