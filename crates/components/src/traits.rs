//! Component traits for Immortal Engine
//!
//! This module defines the core traits that components can implement
//! to provide custom behavior, validation, and code generation.

use imortal_core::{EngineResult, ConfigValue};
use imortal_ir::{Node, ProjectGraph};

use crate::definition::ComponentDefinition;

/// Core trait for component behavior
///
/// Implement this trait to provide custom logic for a component type.
/// This is used by the engine to handle component-specific operations
/// like validation, initialization, and event handling.
pub trait Component: Send + Sync {
    /// Get the component type ID (e.g., "auth.login")
    fn type_id(&self) -> &str;

    /// Get the human-readable name
    fn name(&self) -> &str;

    /// Validate a node instance of this component
    fn validate(&self, node: &Node, graph: &ProjectGraph) -> EngineResult<()> {
        let _ = (node, graph);
        Ok(())
    }

    /// Called when a node is first created
    fn on_create(&self, node: &mut Node) {
        let _ = node;
    }

    /// Called when a node is updated
    fn on_update(&self, node: &mut Node, old_node: &Node) {
        let _ = (node, old_node);
    }

    /// Called when a node is deleted
    fn on_delete(&self, node: &Node, graph: &mut ProjectGraph) {
        let _ = (node, graph);
    }

    /// Called when a connection is made to this component
    fn on_connect(&self, node: &mut Node, port_id: &str, source_node: &Node) {
        let _ = (node, port_id, source_node);
    }

    /// Called when a connection is removed from this component
    fn on_disconnect(&self, node: &mut Node, port_id: &str) {
        let _ = (node, port_id);
    }

    /// Get default configuration values
    fn default_config(&self) -> std::collections::HashMap<String, ConfigValue> {
        std::collections::HashMap::new()
    }

    /// Check if a configuration change is valid
    fn validate_config(&self, key: &str, value: &ConfigValue) -> EngineResult<()> {
        let _ = (key, value);
        Ok(())
    }
}

/// Factory trait for creating component instances
///
/// Implement this trait to provide custom instantiation logic
/// for components.
pub trait ComponentFactory: Send + Sync {
    /// Get the component definition
    fn definition(&self) -> &ComponentDefinition;

    /// Create a new node instance from the definition
    fn create_node(&self) -> Node {
        self.definition().instantiate_default()
    }

    /// Create a new node instance with a custom name
    fn create_node_with_name(&self, name: &str) -> Node {
        self.definition().instantiate(name)
    }

    /// Create a node at a specific position
    fn create_node_at(&self, name: &str, x: f32, y: f32) -> Node {
        let mut node = self.create_node_with_name(name);
        node.position.x = x;
        node.position.y = y;
        node
    }
}

/// Extension trait for ComponentDefinition to implement ComponentFactory
impl ComponentFactory for ComponentDefinition {
    fn definition(&self) -> &ComponentDefinition {
        self
    }
}

/// Trait for components that can generate code
pub trait CodeGeneratable: Component {
    /// The output type for code generation
    type Output;

    /// Generate code for this component
    fn generate_code(&self, node: &Node, context: &CodeGenContext) -> EngineResult<Self::Output>;

    /// Get the generator identifier
    fn generator_id(&self) -> &str;
}

/// Context for code generation
#[derive(Debug, Clone)]
pub struct CodeGenContext {
    /// The project graph being generated
    pub graph: ProjectGraph,
    /// Target language (e.g., "rust", "typescript")
    pub target_language: String,
    /// Target framework (e.g., "axum", "actix")
    pub target_framework: Option<String>,
    /// Output directory
    pub output_dir: String,
    /// Additional options
    pub options: std::collections::HashMap<String, ConfigValue>,
}

impl CodeGenContext {
    /// Create a new code generation context
    pub fn new(graph: ProjectGraph) -> Self {
        Self {
            graph,
            target_language: "rust".to_string(),
            target_framework: None,
            output_dir: "generated".to_string(),
            options: std::collections::HashMap::new(),
        }
    }

    /// Set the target language
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.target_language = language.into();
        self
    }

    /// Set the target framework
    pub fn with_framework(mut self, framework: impl Into<String>) -> Self {
        self.target_framework = Some(framework.into());
        self
    }

    /// Set the output directory
    pub fn with_output_dir(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    /// Add an option
    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Get an option value
    pub fn get_option(&self, key: &str) -> Option<&ConfigValue> {
        self.options.get(key)
    }

    /// Get an option as a string
    pub fn get_option_str(&self, key: &str) -> Option<&str> {
        self.options.get(key).and_then(|v| v.as_str())
    }

    /// Get an option as a bool
    pub fn get_option_bool(&self, key: &str) -> Option<bool> {
        self.options.get(key).and_then(|v| v.as_bool())
    }
}

/// Trait for components that support live preview
pub trait Previewable: Component {
    /// Get a preview representation of this component
    fn preview(&self, node: &Node) -> PreviewData;
}

/// Preview data for a component
#[derive(Debug, Clone, Default)]
pub struct PreviewData {
    /// Preview type
    pub preview_type: PreviewType,
    /// HTML content for web preview
    pub html: Option<String>,
    /// SVG content for diagram preview
    pub svg: Option<String>,
    /// Plain text representation
    pub text: Option<String>,
    /// Custom data
    pub data: std::collections::HashMap<String, String>,
}

impl PreviewData {
    /// Create an empty preview
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a text preview
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            preview_type: PreviewType::Text,
            text: Some(content.into()),
            ..Default::default()
        }
    }

    /// Create an HTML preview
    pub fn html(content: impl Into<String>) -> Self {
        Self {
            preview_type: PreviewType::Html,
            html: Some(content.into()),
            ..Default::default()
        }
    }

    /// Create an SVG preview
    pub fn svg(content: impl Into<String>) -> Self {
        Self {
            preview_type: PreviewType::Svg,
            svg: Some(content.into()),
            ..Default::default()
        }
    }
}

/// Types of preview
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PreviewType {
    /// No preview available
    #[default]
    None,
    /// Plain text preview
    Text,
    /// HTML preview
    Html,
    /// SVG diagram preview
    Svg,
    /// Interactive preview
    Interactive,
}

/// Trait for components that can be serialized to a schema format
pub trait SchemaSerializable: Component {
    /// Serialize this component to a schema representation
    fn to_schema(&self, node: &Node) -> EngineResult<serde_json::Value>;

    /// Create a node from a schema representation
    fn from_schema(&self, schema: &serde_json::Value) -> EngineResult<Node>;
}

/// Trait for components that support undo/redo
pub trait Undoable: Component {
    /// The snapshot type for this component
    type Snapshot: Clone + Send + Sync;

    /// Take a snapshot of the current state
    fn take_snapshot(&self, node: &Node) -> Self::Snapshot;

    /// Restore from a snapshot
    fn restore_snapshot(&self, node: &mut Node, snapshot: &Self::Snapshot);

    /// Get a description of what changed
    fn describe_change(&self, before: &Self::Snapshot, after: &Self::Snapshot) -> String;
}

/// Marker trait for components that are thread-safe
pub trait ThreadSafeComponent: Component + Send + Sync {}

/// Blanket implementation for all Send + Sync components
impl<T: Component + Send + Sync> ThreadSafeComponent for T {}

/// Helper macro for implementing basic Component trait
#[macro_export]
macro_rules! impl_component {
    ($type:ty, $id:expr, $name:expr) => {
        impl $crate::traits::Component for $type {
            fn type_id(&self) -> &str {
                $id
            }

            fn name(&self) -> &str {
                $name
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent;

    impl Component for TestComponent {
        fn type_id(&self) -> &str {
            "test.component"
        }

        fn name(&self) -> &str {
            "Test Component"
        }
    }

    #[test]
    fn test_component_trait() {
        let comp = TestComponent;
        assert_eq!(comp.type_id(), "test.component");
        assert_eq!(comp.name(), "Test Component");
    }

    #[test]
    fn test_preview_data() {
        let text_preview = PreviewData::text("Hello, World!");
        assert_eq!(text_preview.preview_type, PreviewType::Text);
        assert_eq!(text_preview.text, Some("Hello, World!".to_string()));

        let html_preview = PreviewData::html("<p>Hello</p>");
        assert_eq!(html_preview.preview_type, PreviewType::Html);
    }

    #[test]
    fn test_code_gen_context() {
        let graph = ProjectGraph::default();
        let ctx = CodeGenContext::new(graph)
            .with_language("rust")
            .with_framework("axum")
            .with_option("generate_tests", true);

        assert_eq!(ctx.target_language, "rust");
        assert_eq!(ctx.target_framework, Some("axum".to_string()));
        assert_eq!(ctx.get_option_bool("generate_tests"), Some(true));
    }
}
