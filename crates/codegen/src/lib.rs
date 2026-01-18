//! Immortal Engine Code Generation
//!
//! This crate provides code generation capabilities for Immortal Engine.
//! It transforms the IR (ProjectGraph) into actual Rust code.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │              ProjectGraph (IR)              │
//! └─────────────────────────────────────────────┘
//!                      │
//!                      ▼
//! ┌─────────────────────────────────────────────┐
//! │             CodeGenerator                   │
//! │  ┌───────────────────────────────────────┐  │
//! │  │     Domain-specific Generators       │  │
//! │  │  - EntityGenerator                   │  │
//! │  │  - ApiGenerator                      │  │
//! │  │  - AuthGenerator                     │  │
//! │  │  - StorageGenerator                  │  │
//! │  └───────────────────────────────────────┘  │
//! └─────────────────────────────────────────────┘
//!                      │
//!                      ▼
//! ┌─────────────────────────────────────────────┐
//! │           Generated Rust Project           │
//! │  - src/models/*.rs                         │
//! │  - src/handlers/*.rs                       │
//! │  - src/main.rs                             │
//! │  - Cargo.toml                              │
//! │  - migrations/*.sql                        │
//! └─────────────────────────────────────────────┘
//! ```

pub mod generator;
pub mod rust;
pub mod templates;

pub use generator::{CodeGenerator, GeneratorConfig, GeneratedProject};

/// Prelude for convenient imports
pub mod prelude {
    pub use super::generator::{CodeGenerator, GeneratorConfig, GeneratedProject};
}

/// Current version of the code generator
pub const CODEGEN_VERSION: &str = "1.0.0";
