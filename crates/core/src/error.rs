//! Error types for Immortal Engine
//!
//! Provides a unified error handling system across all engine components.

use thiserror::Error;

/// Result type alias for engine operations
pub type EngineResult<T> = Result<T, EngineError>;

/// Core error type for Immortal Engine
#[derive(Error, Debug)]
pub enum EngineError {
    // ========== Graph/IR Errors ==========
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Edge not found: {0}")]
    EdgeNotFound(String),

    #[error("Port not found: {node_id}.{port_id}")]
    PortNotFound { node_id: String, port_id: String },

    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Cycle detected in graph: {0}")]
    CycleDetected(String),

    // ========== Component Errors ==========
    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Component already registered: {0}")]
    ComponentAlreadyExists(String),

    #[error("Invalid component configuration: {0}")]
    InvalidComponentConfig(String),

    #[error("Missing required field: {component}.{field}")]
    MissingRequiredField { component: String, field: String },

    // ========== Validation Errors ==========
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Schema validation error: {0}")]
    SchemaValidation(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    // ========== Code Generation Errors ==========
    #[error("Code generation failed: {0}")]
    CodeGeneration(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Unsupported target: {0}")]
    UnsupportedTarget(String),

    // ========== IO Errors ==========
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    // ========== Project Errors ==========
    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Project already exists: {0}")]
    ProjectAlreadyExists(String),

    #[error("Invalid project structure: {0}")]
    InvalidProjectStructure(String),

    // ========== Generic Errors ==========
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl EngineError {
    /// Create a custom error with the given message
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }

    /// Create an internal error with the given message
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::ValidationFailed(_)
                | Self::InvalidComponentConfig(_)
                | Self::MissingRequiredField { .. }
                | Self::TypeMismatch { .. }
        )
    }
}

// Conversion from serde_json errors
impl From<serde_json::Error> for EngineError {
    fn from(err: serde_json::Error) -> Self {
        EngineError::Serialization(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = EngineError::NodeNotFound("node_123".to_string());
        assert_eq!(err.to_string(), "Node not found: node_123");
    }

    #[test]
    fn test_error_is_recoverable() {
        let recoverable = EngineError::ValidationFailed("test".to_string());
        let not_recoverable = EngineError::Internal("test".to_string());

        assert!(recoverable.is_recoverable());
        assert!(!not_recoverable.is_recoverable());
    }

    #[test]
    fn test_custom_error() {
        let err = EngineError::custom("Something went wrong");
        assert_eq!(err.to_string(), "Something went wrong");
    }
}
