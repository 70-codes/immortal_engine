//! Component Definition types
//!
//! ComponentDefinition is a template/blueprint that describes what a component
//! looks like and what it can do. When a user drags a component onto the canvas,
//! a Node is created based on the ComponentDefinition.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use imortal_core::{
    ComponentCategory, ConfigValue, DataType, PortDirection, PortKind, UiHints, Validation,
};
use imortal_ir::{Field, Node, Port};
use imortal_ir::port::PortCollection;

/// Definition of a component type (template)
///
/// This describes what a component looks like and how it behaves.
/// When a user drags this component onto the canvas, a Node is created
/// from this definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinition {
    /// Unique identifier for this component type (e.g., "auth.login", "data.entity")
    pub id: String,

    /// Human-readable name for display
    pub name: String,

    /// Category for organizing in the palette
    pub category: ComponentCategory,

    /// Description of what this component does
    pub description: String,

    /// Icon for display (emoji or icon name)
    pub icon: String,

    /// Default fields when component is instantiated
    pub fields: Vec<FieldDefinition>,

    /// Port definitions for connections
    pub ports: PortDefinitions,

    /// Configuration options for this component
    pub config: Vec<ConfigOption>,

    /// Tags for searching and filtering
    pub tags: Vec<String>,

    /// Whether this component is deprecated
    pub deprecated: bool,

    /// Deprecation message if deprecated
    pub deprecation_message: Option<String>,

    /// Minimum number of instances allowed (0 = no minimum)
    pub min_instances: u32,

    /// Maximum number of instances allowed (0 = unlimited)
    pub max_instances: u32,

    /// Whether this component can have custom fields added
    pub allow_custom_fields: bool,

    /// Default size when placed on canvas
    pub default_width: f32,
    pub default_height: f32,

    /// Code generator identifier
    pub generator: Option<String>,

    /// Documentation URL
    pub docs_url: Option<String>,

    /// Version of this component definition
    pub version: String,
}

impl ComponentDefinition {
    /// Create a new component definition
    pub fn new(id: impl Into<String>, name: impl Into<String>, category: ComponentCategory) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category,
            description: String::new(),
            icon: category.icon().to_string(),
            fields: Vec::new(),
            ports: PortDefinitions::default(),
            config: Vec::new(),
            tags: Vec::new(),
            deprecated: false,
            deprecation_message: None,
            min_instances: 0,
            max_instances: 0,
            allow_custom_fields: false,
            default_width: 200.0,
            default_height: 150.0,
            generator: None,
            docs_url: None,
            version: "1.0.0".to_string(),
        }
    }

    // ========== Builder Methods ==========

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Add a field definition
    pub fn with_field(mut self, field: FieldDefinition) -> Self {
        self.fields.push(field);
        self
    }

    /// Add multiple field definitions
    pub fn with_fields(mut self, fields: Vec<FieldDefinition>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// Add an input port
    pub fn with_input(mut self, port: PortDefinition) -> Self {
        self.ports.inputs.push(port);
        self
    }

    /// Add an output port
    pub fn with_output(mut self, port: PortDefinition) -> Self {
        self.ports.outputs.push(port);
        self
    }

    /// Add a configuration option
    pub fn with_config(mut self, config: ConfigOption) -> Self {
        self.config.push(config);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set the generator identifier
    pub fn with_generator(mut self, generator: impl Into<String>) -> Self {
        self.generator = Some(generator.into());
        self
    }

    /// Set the docs URL
    pub fn with_docs(mut self, url: impl Into<String>) -> Self {
        self.docs_url = Some(url.into());
        self
    }

    /// Mark as deprecated
    pub fn deprecated(mut self, message: impl Into<String>) -> Self {
        self.deprecated = true;
        self.deprecation_message = Some(message.into());
        self
    }

    /// Allow custom fields
    pub fn allow_custom_fields(mut self) -> Self {
        self.allow_custom_fields = true;
        self
    }

    /// Set instance limits
    pub fn with_instance_limits(mut self, min: u32, max: u32) -> Self {
        self.min_instances = min;
        self.max_instances = max;
        self
    }

    /// Set default size
    pub fn with_default_size(mut self, width: f32, height: f32) -> Self {
        self.default_width = width;
        self.default_height = height;
        self
    }

    // ========== Factory Methods ==========

    /// Create a Node instance from this definition
    pub fn instantiate(&self, name: impl Into<String>) -> Node {
        let name = name.into();
        let mut node = Node::new(&self.id, &name);

        node.category = self.category;
        node.icon = Some(self.icon.clone());
        node.description = if self.description.is_empty() {
            None
        } else {
            Some(self.description.clone())
        };
        node.size = imortal_core::Size::new(self.default_width, self.default_height);

        // Add fields
        for field_def in &self.fields {
            node.fields.push(field_def.to_field());
        }

        // Add ports
        for port_def in &self.ports.inputs {
            node.ports.add_input(port_def.to_port());
        }
        for port_def in &self.ports.outputs {
            node.ports.add_output(port_def.to_port());
        }

        // Add default config values
        for config_opt in &self.config {
            if let Some(default) = &config_opt.default_value {
                node.config.insert(config_opt.id.clone(), default.clone());
            }
        }

        node
    }

    /// Create a Node instance with the component name as the node name
    pub fn instantiate_default(&self) -> Node {
        self.instantiate(&self.name)
    }

    // ========== Query Methods ==========

    /// Get a field definition by name
    pub fn get_field(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Get a config option by ID
    pub fn get_config(&self, id: &str) -> Option<&ConfigOption> {
        self.config.iter().find(|c| c.id == id)
    }

    /// Get an input port definition by ID
    pub fn get_input(&self, id: &str) -> Option<&PortDefinition> {
        self.ports.inputs.iter().find(|p| p.id == id)
    }

    /// Get an output port definition by ID
    pub fn get_output(&self, id: &str) -> Option<&PortDefinition> {
        self.ports.outputs.iter().find(|p| p.id == id)
    }

    /// Check if this component has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

impl Default for ComponentDefinition {
    fn default() -> Self {
        Self::new("custom.component", "Custom Component", ComponentCategory::Custom)
    }
}

/// Port definitions for a component
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortDefinitions {
    /// Input port definitions
    pub inputs: Vec<PortDefinition>,
    /// Output port definitions
    pub outputs: Vec<PortDefinition>,
}

impl PortDefinitions {
    /// Create empty port definitions
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with specified inputs and outputs
    pub fn with_ports(inputs: Vec<PortDefinition>, outputs: Vec<PortDefinition>) -> Self {
        Self { inputs, outputs }
    }
}

/// Definition of a port on a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDefinition {
    /// Port identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Description
    pub description: Option<String>,

    /// Port direction (input or output)
    pub direction: PortDirection,

    /// Port kind (data, trigger, flow)
    pub kind: PortKind,

    /// Data type accepted/produced by this port
    pub data_type: DataType,

    /// Whether multiple connections are allowed
    pub multiple: bool,

    /// Whether this port is required to be connected
    pub required: bool,

    /// Default value if not connected (for inputs)
    pub default_value: Option<String>,

    /// Display order
    pub order: i32,
}

impl PortDefinition {
    /// Create a new port definition
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        direction: PortDirection,
        data_type: DataType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            direction,
            kind: PortKind::Data,
            data_type,
            multiple: false,
            required: false,
            default_value: None,
            order: 0,
        }
    }

    /// Create a data input port
    pub fn data_in(id: impl Into<String>, name: impl Into<String>, data_type: DataType) -> Self {
        Self::new(id, name, PortDirection::Input, data_type)
    }

    /// Create a data output port
    pub fn data_out(id: impl Into<String>, name: impl Into<String>, data_type: DataType) -> Self {
        Self::new(id, name, PortDirection::Output, data_type)
    }

    /// Create a trigger input port
    pub fn trigger_in(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Trigger,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Input, DataType::Trigger)
        }
    }

    /// Create a trigger output port
    pub fn trigger_out(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Trigger,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Output, DataType::Trigger)
        }
    }

    /// Create a flow input port
    pub fn flow_in(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Flow,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Input, DataType::Trigger)
        }
    }

    /// Create a flow output port
    pub fn flow_out(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Flow,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Output, DataType::Trigger)
        }
    }

    // ========== Builder Methods ==========

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Allow multiple connections
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Set default value
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self
    }

    /// Set display order
    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Convert to Port instance
    pub fn to_port(&self) -> Port {
        let mut port = Port::new(&self.id, &self.name, self.direction, self.data_type.clone());
        port.kind = self.kind;
        port.description = self.description.clone();
        port.multiple = self.multiple;
        port.required = self.required;
        port.default_value = self.default_value.clone();
        port.order = self.order;
        port
    }
}

/// Definition of a field in a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Field name (used in code generation)
    pub name: String,

    /// Display label
    pub label: Option<String>,

    /// Data type
    pub data_type: DataType,

    /// Whether this field is required
    pub required: bool,

    /// Default value
    pub default_value: Option<ConfigValue>,

    /// Validation rules
    pub validations: Vec<Validation>,

    /// UI hints
    pub ui_hints: UiHints,

    /// Description
    pub description: Option<String>,

    /// Whether this field is read-only
    pub read_only: bool,

    /// Whether this is a secret field
    pub secret: bool,
}

impl FieldDefinition {
    /// Create a new field definition
    pub fn new(name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            name: name.into(),
            label: None,
            data_type,
            required: false,
            default_value: None,
            validations: Vec::new(),
            ui_hints: UiHints::default(),
            description: None,
            read_only: false,
            secret: false,
        }
    }

    // ========== Type Constructors ==========

    /// Create a string field
    pub fn string(name: impl Into<String>) -> Self {
        Self::new(name, DataType::String)
    }

    /// Create a text field (long content)
    pub fn text(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Text)
    }

    /// Create an integer field
    pub fn int(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Int32)
    }

    /// Create a long integer field
    pub fn long(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Int64)
    }

    /// Create a float field
    pub fn float(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Float64)
    }

    /// Create a boolean field
    pub fn bool(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Bool)
    }

    /// Create a UUID field
    pub fn uuid(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Uuid)
    }

    /// Create a datetime field
    pub fn datetime(name: impl Into<String>) -> Self {
        Self::new(name, DataType::DateTime)
    }

    /// Create a reference field
    pub fn reference(name: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(name, DataType::Reference(target.into()))
    }

    // ========== Builder Methods ==========

    /// Set the label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        if !self.validations.iter().any(|v| matches!(v, Validation::Required)) {
            self.validations.push(Validation::Required);
        }
        self
    }

    /// Set default value
    pub fn with_default(mut self, value: impl Into<ConfigValue>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Add a validation
    pub fn with_validation(mut self, validation: Validation) -> Self {
        self.validations.push(validation);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.ui_hints.placeholder = Some(placeholder.into());
        self
    }

    /// Set help text
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.ui_hints.help = Some(help.into());
        self
    }

    /// Mark as secret (passwords, API keys)
    pub fn secret(mut self) -> Self {
        self.secret = true;
        self.ui_hints.secret = true;
        self
    }

    /// Mark as read-only
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Set display order
    pub fn with_order(mut self, order: i32) -> Self {
        self.ui_hints.order = order;
        self
    }

    /// Set custom widget type
    pub fn with_widget(mut self, widget: impl Into<String>) -> Self {
        self.ui_hints.widget = Some(widget.into());
        self
    }

    /// Show in list views
    pub fn show_in_list(mut self) -> Self {
        self.ui_hints.show_in_list = true;
        self
    }

    /// Convert to Field instance
    pub fn to_field(&self) -> Field {
        let mut field = Field::new(&self.name, self.data_type.clone());
        field.label = self.label.clone();
        field.required = self.required;
        field.default_value = self.default_value.clone();
        field.validations = self.validations.clone();
        field.ui_hints = self.ui_hints.clone();
        field.description = self.description.clone();
        field.read_only = self.read_only;
        if self.secret {
            field.ui_hints.secret = true;
        }
        field
    }
}

/// Configuration option for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    /// Configuration key/identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Description
    pub description: Option<String>,

    /// Type of configuration value
    pub config_type: ConfigType,

    /// Default value
    pub default_value: Option<ConfigValue>,

    /// Whether this option is required
    pub required: bool,

    /// Possible values (for enum/select types)
    pub options: Vec<ConfigOptionValue>,

    /// Validation constraints
    pub constraints: ConfigConstraints,

    /// Display order
    pub order: i32,

    /// Group/section name for organizing config
    pub group: Option<String>,

    /// Whether this is an advanced option
    pub advanced: bool,
}

impl ConfigOption {
    /// Create a new configuration option
    pub fn new(id: impl Into<String>, name: impl Into<String>, config_type: ConfigType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            config_type,
            default_value: None,
            required: false,
            options: Vec::new(),
            constraints: ConfigConstraints::default(),
            order: 0,
            group: None,
            advanced: false,
        }
    }

    // ========== Type Constructors ==========

    /// Create a string config option
    pub fn string(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::String)
    }

    /// Create an integer config option
    pub fn integer(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::Integer)
    }

    /// Create a float config option
    pub fn float(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::Float)
    }

    /// Create a boolean config option
    pub fn boolean(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::Boolean)
    }

    /// Create a select/enum config option
    pub fn select(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::Select)
    }

    /// Create a duration config option
    pub fn duration(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::Duration)
    }

    /// Create a path config option
    pub fn path(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(id, name, ConfigType::Path)
    }

    // ========== Builder Methods ==========

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set default value
    pub fn with_default(mut self, value: impl Into<ConfigValue>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Add a selectable option
    pub fn with_option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.options.push(ConfigOptionValue {
            value: value.into(),
            label: label.into(),
            description: None,
        });
        self
    }

    /// Set minimum value
    pub fn with_min(mut self, min: f64) -> Self {
        self.constraints.min = Some(min);
        self
    }

    /// Set maximum value
    pub fn with_max(mut self, max: f64) -> Self {
        self.constraints.max = Some(max);
        self
    }

    /// Set min and max string length
    pub fn with_length(mut self, min: usize, max: usize) -> Self {
        self.constraints.min_length = Some(min);
        self.constraints.max_length = Some(max);
        self
    }

    /// Set pattern (regex)
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.constraints.pattern = Some(pattern.into());
        self
    }

    /// Set display order
    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Set group name
    pub fn in_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Mark as advanced option
    pub fn advanced(mut self) -> Self {
        self.advanced = true;
        self
    }
}

/// Type of configuration value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConfigType {
    #[default]
    String,
    Text,
    Integer,
    Float,
    Boolean,
    Select,
    MultiSelect,
    Color,
    Duration,
    Path,
    Url,
    Json,
    Code,
}

/// A selectable option value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOptionValue {
    /// The actual value
    pub value: String,
    /// Display label
    pub label: String,
    /// Optional description
    pub description: Option<String>,
}

/// Constraints for configuration values
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigConstraints {
    /// Minimum numeric value
    pub min: Option<f64>,
    /// Maximum numeric value
    pub max: Option<f64>,
    /// Minimum string length
    pub min_length: Option<usize>,
    /// Maximum string length
    pub max_length: Option<usize>,
    /// Regex pattern for validation
    pub pattern: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_definition_creation() {
        let def = ComponentDefinition::new("auth.login", "Login", ComponentCategory::Auth)
            .with_description("User login component")
            .with_icon("🔐")
            .with_field(FieldDefinition::string("email").required())
            .with_field(FieldDefinition::string("password").required().secret());

        assert_eq!(def.id, "auth.login");
        assert_eq!(def.name, "Login");
        assert_eq!(def.fields.len(), 2);
    }

    #[test]
    fn test_component_instantiation() {
        let def = ComponentDefinition::new("data.entity", "Entity", ComponentCategory::Data)
            .with_field(FieldDefinition::uuid("id").required())
            .with_field(FieldDefinition::string("name"));

        let node = def.instantiate("User");

        assert_eq!(node.name, "User");
        assert_eq!(node.component_type, "data.entity");
        assert_eq!(node.fields.len(), 2);
    }

    #[test]
    fn test_field_definition() {
        let field = FieldDefinition::string("email")
            .required()
            .with_label("Email Address")
            .with_placeholder("Enter your email")
            .with_validation(Validation::Email);

        assert_eq!(field.name, "email");
        assert!(field.required);
        assert_eq!(field.label, Some("Email Address".to_string()));
    }

    #[test]
    fn test_port_definition() {
        let port = PortDefinition::trigger_out("success", "On Success")
            .with_description("Triggered on successful operation");

        assert_eq!(port.id, "success");
        assert_eq!(port.kind, PortKind::Trigger);
        assert_eq!(port.direction, PortDirection::Output);
    }

    #[test]
    fn test_config_option() {
        let config = ConfigOption::select("backend", "Database Backend")
            .with_option("postgres", "PostgreSQL")
            .with_option("mysql", "MySQL")
            .with_option("sqlite", "SQLite")
            .with_default("postgres")
            .required();

        assert_eq!(config.id, "backend");
        assert_eq!(config.options.len(), 3);
        assert!(config.required);
    }
}
