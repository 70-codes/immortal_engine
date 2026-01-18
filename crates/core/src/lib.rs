//! Immortal Engine Core
//!
//! Core types, traits, and utilities used across all Immortal Engine crates.

pub mod error;
pub mod types;
pub mod traits;

pub use error::{EngineError, EngineResult};
pub use types::*;
pub use traits::*;

/// Re-export commonly used external types
pub mod prelude {
    pub use super::error::{EngineError, EngineResult};
    pub use super::types::*;
    pub use super::traits::*;
    pub use uuid::Uuid;
}
