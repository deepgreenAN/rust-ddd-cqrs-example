pub mod aggregates;
pub mod global;

#[cfg(feature = "server")]
pub mod repositories;

#[cfg(feature = "server")]
pub mod services;

#[cfg(feature = "server")]
pub mod events;

mod error;
mod id;
mod macros;

pub use error::DomainError;
pub use id::Id;
