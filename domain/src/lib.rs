pub mod aggregates;
pub mod events;

#[cfg(feature = "server")]
pub mod repositories;

mod error;
mod id;
mod macros;

pub use error::DomainError;
pub use id::Id;
