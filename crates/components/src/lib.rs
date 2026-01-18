//! Immortal Engine Components
//!
//! This crate provides the component system for Immortal Engine. Components are
//! the building blocks that users drag and drop onto the canvas to create applications.
//!
//! # Component Types
//!
//! Components are organized into categories:
//! - **Auth**: Login, Register, OAuth
//! - **Data**: Entity, Collection, Query
//! - **API**: REST Endpoint, GraphQL, WebSocket
//! - **Storage**: Database, Cache, FileStore
//! - **UI**: Form, Table, Page (future)
//! - **Logic**: Validator, Transformer, Condition
//! - **Embedded**: GPIO, Sensor, I2C (future)
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │           ComponentRegistry                 │
//! │  ┌───────────────────────────────────────┐  │
//! │  │     ComponentDefinition (template)    │  │
//! │  │  - default fields, ports, config     │  │
//! │  └───────────────────────────────────────┘  │
//! └─────────────────────────────────────────────┘
//!                      │
//!                      │ instantiate
//!                      ▼
//! ┌─────────────────────────────────────────────┐
//! │              Node (instance)                │
//! │  - actual fields, config values            │
//! │  - position on canvas                      │
//! └─────────────────────────────────────────────┘
//! ```

pub mod definition;
pub mod registry;
pub mod traits;

pub mod definitions {
    //! Built-in component definitions
    pub mod auth;
    pub mod data;
    pub mod api;
    pub mod storage;
    pub mod logic;
}

// Re-export main types
pub use definition::{
    ComponentDefinition,
    FieldDefinition,
    PortDefinition,
    ConfigOption,
    ConfigType,
};
pub use registry::ComponentRegistry;
pub use traits::{Component, ComponentFactory};

// Re-export core types commonly used with components
pub use imortal_core::{
    ComponentCategory,
    DataType,
    ConfigValue,
    PortDirection,
    PortKind,
    Validation,
    UiHints,
};

pub use imortal_ir::{
    Node,
    Port,
    Field,
};

/// Prelude for convenient imports
pub mod prelude {
    pub use super::{
        ComponentDefinition,
        FieldDefinition,
        PortDefinition,
        ConfigOption,
        ConfigType,
        ComponentRegistry,
        Component,
        ComponentFactory,
    };

    pub use imortal_core::prelude::*;
}

/// Current version of the component system
pub const COMPONENT_VERSION: &str = "1.0.0";
