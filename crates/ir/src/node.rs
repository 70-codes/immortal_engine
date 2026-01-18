//! Node type representing components on the canvas
//!
//! A Node is an instance of a component placed on the visual canvas.
//! It contains the component's configuration, fields, ports, and visual properties.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use imortal_core::{
    ConfigValue, Position, Size, NodeId, ComponentCategory, DataType,
};

use crate::field::Field;
use crate::port::{Port, PortCollection};

/// A node (component instance) on the canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node
    pub id: NodeId,

    /// The type of component this node represents (e.g., "auth.login", "data.entity")
    pub component_type: String,

    /// User-defined name for this instance
    pub name: String,

    /// Position on the canvas
    pub position: Position,

    /// Size of the node on the canvas
    pub size: Size,

    /// Fields defined in this component instance
    pub fields: Vec<Field>,

    /// Input and output ports for connections
    pub ports: PortCollection,

    /// Configuration values for this component
    pub config: HashMap<String, ConfigValue>,

    /// Category for visual organization
    pub category: ComponentCategory,

    /// Visual state: whether the node is collapsed
    pub collapsed: bool,

    /// Visual state: whether the node is selected
    #[serde(skip)]
    pub selected: bool,

    /// Whether this node is locked (cannot be moved or edited)
    pub locked: bool,

    /// Whether this node is visible
    pub visible: bool,

    /// Optional description/documentation
    pub description: Option<String>,

    /// Optional icon for display
    pub icon: Option<String>,

    /// Color hint for the node header
    pub color: Option<NodeColor>,

    /// Tags for filtering and organization
    pub tags: Vec<String>,

    /// Custom metadata
    pub metadata: HashMap<String, ConfigValue>,

    /// Z-index for layering (higher = on top)
    pub z_index: i32,

    /// Group ID if this node belongs to a group
    pub group_id: Option<Uuid>,

    /// Timestamp when this node was created
    pub created_at: Option<String>,

    /// Timestamp when this node was last modified
    pub modified_at: Option<String>,
}

impl Node {
    /// Create a new node with the given component type and name
    pub fn new(component_type: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            component_type: component_type.into(),
            name: name.into(),
            position: Position::default(),
            size: Size::default_component(),
            fields: Vec::new(),
            ports: PortCollection::new(),
            config: HashMap::new(),
            category: ComponentCategory::Custom,
            collapsed: false,
            selected: false,
            locked: false,
            visible: true,
            description: None,
            icon: None,
            color: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
            z_index: 0,
            group_id: None,
            created_at: None,
            modified_at: None,
        }
    }

    /// Create a new entity node (for data modeling)
    pub fn new_entity(name: impl Into<String>) -> Self {
        let name = name.into();
        let mut node = Self::new("data.entity", &name);
        node.category = ComponentCategory::Data;
        node.icon = Some("📊".to_string());

        // Add default ID field
        node.fields.push(
            Field::uuid("id")
                .primary_key()
                .with_label("ID")
                .with_description("Unique identifier")
        );

        // Add standard entity ports
        node.ports.add_input(Port::data_in(
            "entity",
            &name,
            DataType::Entity(name.clone()),
        ));
        node.ports.add_output(Port::data_out(
            "entity",
            &name,
            DataType::Entity(name.clone()),
        ));

        node
    }

    /// Create a new login component
    pub fn new_login() -> Self {
        let mut node = Self::new("auth.login", "Login");
        node.category = ComponentCategory::Auth;
        node.icon = Some("🔐".to_string());

        // Add default fields
        node.fields.push(
            Field::string("email")
                .required()
                .with_label("Email")
                .with_placeholder("Enter email")
                .with_validation(imortal_core::Validation::Email)
        );
        node.fields.push(
            Field::string("password")
                .required()
                .secret()
                .with_label("Password")
                .with_placeholder("Enter password")
                .with_validation(imortal_core::Validation::MinLength(8))
        );

        // Add ports
        node.ports.add_input(Port::trigger_in("submit", "Submit"));
        node.ports.add_output(Port::data_out("user", "User", DataType::Entity("User".to_string())));
        node.ports.add_output(Port::trigger_out("success", "On Success"));
        node.ports.add_output(Port::trigger_out("failure", "On Failure"));

        node
    }

    /// Create a new register component
    pub fn new_register() -> Self {
        let mut node = Self::new("auth.register", "Register");
        node.category = ComponentCategory::Auth;
        node.icon = Some("📝".to_string());

        // Add default fields
        node.fields.push(
            Field::string("username")
                .required()
                .with_label("Username")
                .with_placeholder("Choose a username")
                .with_validation(imortal_core::Validation::MinLength(3))
        );
        node.fields.push(
            Field::string("email")
                .required()
                .with_label("Email")
                .with_placeholder("Enter email")
                .with_validation(imortal_core::Validation::Email)
        );
        node.fields.push(
            Field::string("password")
                .required()
                .secret()
                .with_label("Password")
                .with_placeholder("Create password")
                .with_validation(imortal_core::Validation::MinLength(8))
        );
        node.fields.push(
            Field::string("confirm_password")
                .required()
                .secret()
                .with_label("Confirm Password")
                .with_placeholder("Confirm password")
        );

        // Add ports
        node.ports.add_input(Port::trigger_in("submit", "Submit"));
        node.ports.add_output(Port::data_out("user", "User", DataType::Entity("User".to_string())));
        node.ports.add_output(Port::trigger_out("success", "On Success"));
        node.ports.add_output(Port::trigger_out("failure", "On Failure"));

        node
    }

    /// Create a new REST API endpoint component
    pub fn new_rest_endpoint(name: impl Into<String>) -> Self {
        let name = name.into();
        let mut node = Self::new("api.rest", &name);
        node.category = ComponentCategory::Api;
        node.icon = Some("🔌".to_string());

        // Add configuration
        node.config.insert("method".to_string(), ConfigValue::String("GET".to_string()));
        node.config.insert("path".to_string(), ConfigValue::String(format!("/{}", name.to_lowercase())));
        node.config.insert("auth_required".to_string(), ConfigValue::Bool(false));

        // Add ports
        node.ports.add_input(Port::data_in("request", "Request", DataType::Any));
        node.ports.add_output(Port::data_out("response", "Response", DataType::Any));
        node.ports.add_output(Port::trigger_out("on_request", "On Request"));

        node
    }

    /// Create a new database component
    pub fn new_database(name: impl Into<String>) -> Self {
        let name = name.into();
        let mut node = Self::new("storage.database", &name);
        node.category = ComponentCategory::Storage;
        node.icon = Some("💾".to_string());

        // Add configuration
        node.config.insert("backend".to_string(), ConfigValue::String("postgres".to_string()));
        node.config.insert("connection_string".to_string(), ConfigValue::String("".to_string()));

        // Add ports
        node.ports.add_input(Port::data_in("query", "Query", DataType::Any));
        node.ports.add_output(Port::data_out("result", "Result", DataType::Any));

        node
    }

    // ========== Builder Methods ==========

    /// Set the position
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = Position::new(x, y);
        self
    }

    /// Set the size
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }

    /// Add a field
    pub fn with_field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Add multiple fields
    pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// Add an input port
    pub fn with_input(mut self, port: Port) -> Self {
        self.ports.add_input(port);
        self
    }

    /// Add an output port
    pub fn with_output(mut self, port: Port) -> Self {
        self.ports.add_output(port);
        self
    }

    /// Set a configuration value
    pub fn with_config(mut self, key: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.config.insert(key.into(), value.into());
        self
    }

    /// Set the category
    pub fn with_category(mut self, category: ComponentCategory) -> Self {
        self.category = category;
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the color
    pub fn with_color(mut self, color: NodeColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set locked state
    pub fn locked(mut self) -> Self {
        self.locked = true;
        self
    }

    /// Set the z-index
    pub fn with_z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }

    // ========== Query Methods ==========

    /// Get a field by name
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Get a mutable field by name
    pub fn get_field_mut(&mut self, name: &str) -> Option<&mut Field> {
        self.fields.iter_mut().find(|f| f.name == name)
    }

    /// Get a port by ID
    pub fn get_port(&self, id: &str) -> Option<&Port> {
        self.ports.get(id)
    }

    /// Get an input port by ID
    pub fn get_input_port(&self, id: &str) -> Option<&Port> {
        self.ports.get_input(id)
    }

    /// Get an output port by ID
    pub fn get_output_port(&self, id: &str) -> Option<&Port> {
        self.ports.get_output(id)
    }

    /// Get a configuration value
    pub fn get_config(&self, key: &str) -> Option<&ConfigValue> {
        self.config.get(key)
    }

    /// Get a configuration value as a string
    pub fn get_config_str(&self, key: &str) -> Option<&str> {
        self.config.get(key).and_then(|v| v.as_str())
    }

    /// Get a configuration value as a bool
    pub fn get_config_bool(&self, key: &str) -> Option<bool> {
        self.config.get(key).and_then(|v| v.as_bool())
    }

    /// Check if this node has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Get the center position of the node
    pub fn center(&self) -> Position {
        Position::new(
            self.position.x + self.size.width / 2.0,
            self.position.y + self.size.height / 2.0,
        )
    }

    /// Get the bounding rectangle
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.position.x, self.position.y, self.size.width, self.size.height)
    }

    /// Check if a point is inside this node
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.x
            && x <= self.position.x + self.size.width
            && y >= self.position.y
            && y <= self.position.y + self.size.height
    }

    /// Check if this node intersects with another
    pub fn intersects(&self, other: &Node) -> bool {
        let (x1, y1, w1, h1) = self.bounds();
        let (x2, y2, w2, h2) = other.bounds();

        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    // ========== Mutation Methods ==========

    /// Move the node by a delta
    pub fn translate(&mut self, dx: f32, dy: f32) {
        if !self.locked {
            self.position.x += dx;
            self.position.y += dy;
        }
    }

    /// Set the position
    pub fn set_position(&mut self, x: f32, y: f32) {
        if !self.locked {
            self.position = Position::new(x, y);
        }
    }

    /// Resize the node
    pub fn resize(&mut self, width: f32, height: f32) {
        if !self.locked {
            self.size = Size::new(width.max(100.0), height.max(50.0));
        }
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Toggle selection state
    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }

    /// Add a field
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    /// Remove a field by name
    pub fn remove_field(&mut self, name: &str) -> Option<Field> {
        if let Some(idx) = self.fields.iter().position(|f| f.name == name) {
            Some(self.fields.remove(idx))
        } else {
            None
        }
    }

    /// Set a configuration value
    pub fn set_config(&mut self, key: impl Into<String>, value: impl Into<ConfigValue>) {
        self.config.insert(key.into(), value.into());
    }

    /// Bring this node to front (increase z-index)
    pub fn bring_to_front(&mut self, max_z: i32) {
        self.z_index = max_z + 1;
    }

    /// Send this node to back (set z-index to 0)
    pub fn send_to_back(&mut self) {
        self.z_index = 0;
    }

    /// Clone this node with a new ID
    pub fn duplicate(&self) -> Self {
        let mut clone = self.clone();
        clone.id = Uuid::new_v4();
        clone.name = format!("{} (copy)", self.name);
        clone.position.x += 20.0;
        clone.position.y += 20.0;
        clone.selected = false;
        clone
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new("custom.node", "New Node")
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Color options for node headers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeColor {
    Default,
    Red,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Indigo,
    Purple,
    Pink,
    Gray,
}

impl NodeColor {
    /// Get the RGB color values (0-255)
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            NodeColor::Default => (100, 100, 100),
            NodeColor::Red => (239, 68, 68),
            NodeColor::Orange => (249, 115, 22),
            NodeColor::Yellow => (234, 179, 8),
            NodeColor::Green => (34, 197, 94),
            NodeColor::Teal => (20, 184, 166),
            NodeColor::Blue => (59, 130, 246),
            NodeColor::Indigo => (99, 102, 241),
            NodeColor::Purple => (168, 85, 247),
            NodeColor::Pink => (236, 72, 153),
            NodeColor::Gray => (107, 114, 128),
        }
    }

    /// Get the color as a hex string
    pub fn hex(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Get all available colors
    pub fn all() -> &'static [NodeColor] {
        &[
            NodeColor::Default,
            NodeColor::Red,
            NodeColor::Orange,
            NodeColor::Yellow,
            NodeColor::Green,
            NodeColor::Teal,
            NodeColor::Blue,
            NodeColor::Indigo,
            NodeColor::Purple,
            NodeColor::Pink,
            NodeColor::Gray,
        ]
    }
}

impl Default for NodeColor {
    fn default() -> Self {
        NodeColor::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new("test.component", "Test Node");
        assert_eq!(node.component_type, "test.component");
        assert_eq!(node.name, "Test Node");
        assert!(!node.selected);
        assert!(!node.locked);
    }

    #[test]
    fn test_entity_node() {
        let node = Node::new_entity("User");
        assert_eq!(node.component_type, "data.entity");
        assert_eq!(node.name, "User");
        assert_eq!(node.category, ComponentCategory::Data);
        assert!(node.get_field("id").is_some());
        assert!(node.ports.get_output("entity").is_some());
    }

    #[test]
    fn test_login_node() {
        let node = Node::new_login();
        assert_eq!(node.component_type, "auth.login");
        assert!(node.get_field("email").is_some());
        assert!(node.get_field("password").is_some());
        assert!(node.ports.get_output("success").is_some());
        assert!(node.ports.get_output("failure").is_some());
    }

    #[test]
    fn test_node_builder() {
        let node = Node::new("test", "Test")
            .with_position(100.0, 200.0)
            .with_size(300.0, 150.0)
            .with_description("A test node")
            .with_tag("test")
            .with_config("key", "value");

        assert_eq!(node.position.x, 100.0);
        assert_eq!(node.position.y, 200.0);
        assert_eq!(node.size.width, 300.0);
        assert!(node.has_tag("test"));
        assert_eq!(node.get_config_str("key"), Some("value"));
    }

    #[test]
    fn test_node_contains_point() {
        let node = Node::new("test", "Test")
            .with_position(100.0, 100.0)
            .with_size(200.0, 100.0);

        assert!(node.contains_point(150.0, 150.0));
        assert!(!node.contains_point(50.0, 50.0));
        assert!(!node.contains_point(350.0, 150.0));
    }

    #[test]
    fn test_node_translate() {
        let mut node = Node::new("test", "Test")
            .with_position(100.0, 100.0);

        node.translate(50.0, -25.0);
        assert_eq!(node.position.x, 150.0);
        assert_eq!(node.position.y, 75.0);
    }

    #[test]
    fn test_locked_node() {
        let mut node = Node::new("test", "Test")
            .with_position(100.0, 100.0)
            .locked();

        node.translate(50.0, 50.0);
        // Position should not change when locked
        assert_eq!(node.position.x, 100.0);
        assert_eq!(node.position.y, 100.0);
    }

    #[test]
    fn test_node_duplicate() {
        let original = Node::new("test", "Test")
            .with_position(100.0, 100.0);

        let duplicate = original.duplicate();

        assert_ne!(original.id, duplicate.id);
        assert!(duplicate.name.contains("copy"));
        assert_eq!(duplicate.position.x, 120.0);
        assert_eq!(duplicate.position.y, 120.0);
    }

    #[test]
    fn test_node_fields() {
        let mut node = Node::new_entity("User");
        node.add_field(Field::string("username").required());
        node.add_field(Field::string("email").required());

        assert_eq!(node.fields.len(), 3); // id + username + email
        assert!(node.get_field("username").is_some());

        node.remove_field("email");
        assert!(node.get_field("email").is_none());
    }

    #[test]
    fn test_node_color() {
        let color = NodeColor::Blue;
        let (r, g, b) = color.rgb();
        assert_eq!((r, g, b), (59, 130, 246));
        assert_eq!(color.hex(), "#3B82F6");
    }
}
