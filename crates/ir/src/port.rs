//! Port definitions for component connections
//!
//! Ports are the connection points on components where edges (arrows) can attach.
//! Each component has input ports (receiving data/signals) and output ports (sending data/signals).

use serde::{Deserialize, Serialize};
use imortal_core::{DataType, PortDirection, PortKind, PortId};

/// A port on a component where connections can be made
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    /// Unique identifier for this port within the component
    pub id: PortId,

    /// Human-readable name for display
    pub name: String,

    /// Description of what this port does
    pub description: Option<String>,

    /// Whether this is an input or output port
    pub direction: PortDirection,

    /// The kind of port (data, trigger, flow)
    pub kind: PortKind,

    /// The data type this port accepts or produces
    pub data_type: DataType,

    /// Whether multiple connections can be made to/from this port
    pub multiple: bool,

    /// Whether this port is required for the component to function
    pub required: bool,

    /// Default value if the port is not connected (for inputs only)
    pub default_value: Option<String>,

    /// Visual index for ordering ports on the component (lower = higher on the component)
    pub order: i32,
}

impl Port {
    /// Create a new port with the given id, name, direction, and data type
    pub fn new(
        id: impl Into<PortId>,
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

    // ========== Input Port Constructors ==========

    /// Create a data input port
    pub fn data_in(id: impl Into<PortId>, name: impl Into<String>, data_type: DataType) -> Self {
        Self::new(id, name, PortDirection::Input, data_type)
    }

    /// Create a trigger input port (receives events/signals)
    pub fn trigger_in(id: impl Into<PortId>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Trigger,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Input, DataType::Trigger)
        }
    }

    /// Create a flow input port (control flow)
    pub fn flow_in(id: impl Into<PortId>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Flow,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Input, DataType::Trigger)
        }
    }

    // ========== Output Port Constructors ==========

    /// Create a data output port
    pub fn data_out(id: impl Into<PortId>, name: impl Into<String>, data_type: DataType) -> Self {
        Self::new(id, name, PortDirection::Output, data_type)
    }

    /// Create a trigger output port (sends events/signals)
    pub fn trigger_out(id: impl Into<PortId>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Trigger,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Output, DataType::Trigger)
        }
    }

    /// Create a flow output port (control flow)
    pub fn flow_out(id: impl Into<PortId>, name: impl Into<String>) -> Self {
        Self {
            kind: PortKind::Flow,
            data_type: DataType::Trigger,
            ..Self::new(id, name, PortDirection::Output, DataType::Trigger)
        }
    }

    // ========== Builder Methods ==========

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Allow multiple connections to this port
    pub fn with_multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Mark this port as required
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Set a default value for this port
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self
    }

    /// Set the visual order
    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    // ========== Query Methods ==========

    /// Check if this is an input port
    pub fn is_input(&self) -> bool {
        matches!(self.direction, PortDirection::Input)
    }

    /// Check if this is an output port
    pub fn is_output(&self) -> bool {
        matches!(self.direction, PortDirection::Output)
    }

    /// Check if this is a data port
    pub fn is_data(&self) -> bool {
        matches!(self.kind, PortKind::Data)
    }

    /// Check if this is a trigger port
    pub fn is_trigger(&self) -> bool {
        matches!(self.kind, PortKind::Trigger)
    }

    /// Check if this is a flow port
    pub fn is_flow(&self) -> bool {
        matches!(self.kind, PortKind::Flow)
    }

    /// Check if this port can connect to another port
    pub fn can_connect_to(&self, other: &Port) -> bool {
        // Can only connect output -> input
        if self.direction == other.direction {
            return false;
        }

        // Data types must be compatible
        if !self.data_type.is_compatible_with(&other.data_type) {
            return false;
        }

        // Port kinds should match (data to data, trigger to trigger)
        if self.kind != other.kind {
            return false;
        }

        true
    }

    /// Get the color hint for this port based on its kind and data type
    pub fn color_hint(&self) -> PortColor {
        match self.kind {
            PortKind::Trigger => PortColor::Trigger,
            PortKind::Flow => PortColor::Flow,
            PortKind::Data => match &self.data_type {
                DataType::String | DataType::Text => PortColor::String,
                DataType::Int32 | DataType::Int64 => PortColor::Integer,
                DataType::Float32 | DataType::Float64 => PortColor::Float,
                DataType::Bool => PortColor::Boolean,
                DataType::Entity(_) | DataType::Reference(_) => PortColor::Entity,
                DataType::Array(_) => PortColor::Array,
                DataType::Any => PortColor::Any,
                _ => PortColor::Default,
            },
        }
    }
}

/// Color hints for rendering ports in the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortColor {
    Default,
    String,
    Integer,
    Float,
    Boolean,
    Entity,
    Array,
    Any,
    Trigger,
    Flow,
}

impl PortColor {
    /// Get the RGB color values (0-255)
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            PortColor::Default => (150, 150, 150),  // Gray
            PortColor::String => (255, 200, 100),   // Orange/Yellow
            PortColor::Integer => (100, 200, 255),  // Light Blue
            PortColor::Float => (100, 255, 200),    // Cyan/Teal
            PortColor::Boolean => (255, 100, 100),  // Red
            PortColor::Entity => (200, 100, 255),   // Purple
            PortColor::Array => (255, 150, 200),    // Pink
            PortColor::Any => (255, 255, 255),      // White
            PortColor::Trigger => (255, 255, 100),  // Yellow
            PortColor::Flow => (100, 255, 100),     // Green
        }
    }

    /// Get the color as a hex string
    pub fn hex(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

/// Collection of ports for a component, separated by direction
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortCollection {
    /// Input ports (receive data/signals)
    pub inputs: Vec<Port>,

    /// Output ports (send data/signals)
    pub outputs: Vec<Port>,
}

impl PortCollection {
    /// Create an empty port collection
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Create a port collection with the given inputs and outputs
    pub fn with_ports(inputs: Vec<Port>, outputs: Vec<Port>) -> Self {
        Self { inputs, outputs }
    }

    /// Add an input port
    pub fn add_input(&mut self, port: Port) -> &mut Self {
        debug_assert!(port.is_input(), "Port must be an input port");
        self.inputs.push(port);
        self
    }

    /// Add an output port
    pub fn add_output(&mut self, port: Port) -> &mut Self {
        debug_assert!(port.is_output(), "Port must be an output port");
        self.outputs.push(port);
        self
    }

    /// Get a port by its ID (searches both inputs and outputs)
    pub fn get(&self, id: &str) -> Option<&Port> {
        self.inputs
            .iter()
            .chain(self.outputs.iter())
            .find(|p| p.id == id)
    }

    /// Get a mutable reference to a port by its ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Port> {
        self.inputs
            .iter_mut()
            .chain(self.outputs.iter_mut())
            .find(|p| p.id == id)
    }

    /// Get an input port by ID
    pub fn get_input(&self, id: &str) -> Option<&Port> {
        self.inputs.iter().find(|p| p.id == id)
    }

    /// Get an output port by ID
    pub fn get_output(&self, id: &str) -> Option<&Port> {
        self.outputs.iter().find(|p| p.id == id)
    }

    /// Get all ports as an iterator
    pub fn all(&self) -> impl Iterator<Item = &Port> {
        self.inputs.iter().chain(self.outputs.iter())
    }

    /// Get the total number of ports
    pub fn len(&self) -> usize {
        self.inputs.len() + self.outputs.len()
    }

    /// Check if there are no ports
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty() && self.outputs.is_empty()
    }

    /// Sort ports by their order field
    pub fn sort_by_order(&mut self) {
        self.inputs.sort_by_key(|p| p.order);
        self.outputs.sort_by_key(|p| p.order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_creation() {
        let port = Port::data_in("user_id", "User ID", DataType::Uuid);
        assert!(port.is_input());
        assert!(port.is_data());
        assert_eq!(port.id, "user_id");
        assert_eq!(port.name, "User ID");
    }

    #[test]
    fn test_port_builder() {
        let port = Port::data_out("result", "Result", DataType::Entity("User".to_string()))
            .with_description("The authenticated user")
            .with_multiple(true)
            .with_order(1);

        assert!(port.is_output());
        assert_eq!(port.description, Some("The authenticated user".to_string()));
        assert!(port.multiple);
        assert_eq!(port.order, 1);
    }

    #[test]
    fn test_port_can_connect() {
        let output = Port::data_out("out", "Output", DataType::String);
        let input = Port::data_in("in", "Input", DataType::String);
        let other_output = Port::data_out("out2", "Output 2", DataType::String);

        assert!(output.can_connect_to(&input));
        assert!(!output.can_connect_to(&other_output)); // Can't connect output to output
    }

    #[test]
    fn test_port_collection() {
        let mut ports = PortCollection::new();
        ports.add_input(Port::data_in("email", "Email", DataType::String));
        ports.add_input(Port::data_in("password", "Password", DataType::String));
        ports.add_output(Port::trigger_out("success", "On Success"));

        assert_eq!(ports.len(), 3);
        assert!(ports.get("email").is_some());
        assert!(ports.get_input("email").is_some());
        assert!(ports.get_output("success").is_some());
    }

    #[test]
    fn test_port_color() {
        let string_port = Port::data_in("name", "Name", DataType::String);
        let trigger_port = Port::trigger_out("done", "Done");

        assert_eq!(string_port.color_hint(), PortColor::String);
        assert_eq!(trigger_port.color_hint(), PortColor::Trigger);
    }
}
