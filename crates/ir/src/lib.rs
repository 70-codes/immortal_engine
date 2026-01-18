//! Immortal Engine Intermediate Representation (IR)
//!
//! This crate provides the graph-based intermediate representation for Immortal Engine projects.
//! The IR represents visual projects as a directed graph where:
//! - **Nodes** represent components (Login, Entity, API Endpoint, etc.)
//! - **Edges** represent connections between components (data flow, relationships, triggers)
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │              ProjectGraph               │
//! │  ┌─────────────────────────────────┐    │
//! │  │            Nodes                │    │
//! │  │  (Components on the canvas)     │    │
//! │  └─────────────────────────────────┘    │
//! │  ┌─────────────────────────────────┐    │
//! │  │            Edges                │    │
//! │  │  (Connections between nodes)    │    │
//! │  └─────────────────────────────────┘    │
//! │  ┌─────────────────────────────────┐    │
//! │  │           Groups                │    │
//! │  │  (Visual organization)          │    │
//! │  └─────────────────────────────────┘    │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use imortal_ir::{ProjectGraph, Node, Edge, ProjectMeta};
//!
//! // Create a new project
//! let mut project = ProjectGraph::new(ProjectMeta {
//!     name: "my_blog".to_string(),
//!     version: "0.1.0".to_string(),
//!     description: Some("A simple blog application".to_string()),
//!     ..Default::default()
//! });
//!
//! // Add components
//! let user_node = project.add_node(Node::new_entity("User"));
//! let post_node = project.add_node(Node::new_entity("Post"));
//!
//! // Connect them with a relationship
//! project.add_edge(Edge::relationship(
//!     user_node,
//!     post_node,
//!     RelationType::OneToMany,
//! ));
//!
//! // Save the project
//! project.save_to_file("my_blog.imortal")?;
//! ```

pub mod graph;
pub mod node;
pub mod edge;
pub mod port;
pub mod field;
pub mod project;
pub mod group;
pub mod validation;
pub mod serialization;

// Re-export main types at crate root
pub use graph::ProjectGraph;
pub use node::Node;
pub use edge::{Edge, DataMapping};
pub use port::Port;
pub use field::Field;
pub use project::ProjectMeta;
pub use group::Group;
pub use validation::{ValidationError, ValidationResult, Validator};
pub use serialization::{ProjectFormat, load_project, save_project};

// Re-export core types that are commonly used with IR
pub use imortal_core::{
    DataType,
    ConfigValue,
    ComponentCategory,
    ConnectionType,
    RelationType,
    PortDirection,
    PortKind,
    Position,
    Size,
    Rect,
    Validation as FieldValidation,
    UiHints,
    NodeId,
    EdgeId,
    PortId,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use super::{
        ProjectGraph,
        Node,
        Edge,
        DataMapping,
        Port,
        Field,
        ProjectMeta,
        Group,
        ValidationError,
        ValidationResult,
        Validator,
        ProjectFormat,
        load_project,
        save_project,
    };

    pub use imortal_core::prelude::*;
}

/// Current version of the IR format
pub const IR_VERSION: &str = "1.0.0";

/// File extension for Immortal Engine project files
pub const PROJECT_EXTENSION: &str = "imortal";

/// File extension for Immortal Engine component files
pub const COMPONENT_EXTENSION: &str = "icomp";
