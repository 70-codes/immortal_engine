//! Edge type representing connections between nodes
//!
//! Edges are the arrows/connections drawn between components on the canvas.
//! They represent data flow, relationships, triggers, navigation, and dependencies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use imortal_core::{ConfigValue, ConnectionType, EdgeId, NodeId, PortId, RelationType};

/// An edge (connection) between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Unique identifier for this edge
    pub id: EdgeId,

    /// Source node ID
    pub from_node: NodeId,

    /// Source port ID on the source node
    pub from_port: PortId,

    /// Target node ID
    pub to_node: NodeId,

    /// Target port ID on the target node
    pub to_port: PortId,

    /// The type of connection this edge represents
    pub connection_type: ConnectionType,

    /// Optional data mapping/transformation between source and target
    pub data_mapping: Option<DataMapping>,

    /// User-defined label for this edge
    pub label: Option<String>,

    /// Whether this edge is enabled (disabled edges are shown dimmed)
    pub enabled: bool,

    /// Whether this edge is selected in the UI
    #[serde(skip)]
    pub selected: bool,

    /// Visual style for this edge
    pub style: EdgeStyle,

    /// Optional description/documentation
    pub description: Option<String>,

    /// Custom metadata
    pub metadata: HashMap<String, ConfigValue>,

    /// Waypoints for custom edge routing (intermediate points)
    pub waypoints: Vec<EdgeWaypoint>,

    /// Z-index for layering (higher = on top)
    pub z_index: i32,
}

impl Edge {
    /// Create a new edge between two ports
    pub fn new(
        from_node: NodeId,
        from_port: impl Into<PortId>,
        to_node: NodeId,
        to_port: impl Into<PortId>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_node,
            from_port: from_port.into(),
            to_node,
            to_port: to_port.into(),
            connection_type: ConnectionType::DataFlow,
            data_mapping: None,
            label: None,
            enabled: true,
            selected: false,
            style: EdgeStyle::default(),
            description: None,
            metadata: HashMap::new(),
            waypoints: Vec::new(),
            z_index: 0,
        }
    }

    /// Create a data flow edge
    pub fn data_flow(
        from_node: NodeId,
        from_port: impl Into<PortId>,
        to_node: NodeId,
        to_port: impl Into<PortId>,
    ) -> Self {
        let mut edge = Self::new(from_node, from_port, to_node, to_port);
        edge.connection_type = ConnectionType::DataFlow;
        edge
    }

    /// Create a trigger edge
    pub fn trigger(
        from_node: NodeId,
        from_port: impl Into<PortId>,
        to_node: NodeId,
        to_port: impl Into<PortId>,
    ) -> Self {
        let mut edge = Self::new(from_node, from_port, to_node, to_port);
        edge.connection_type = ConnectionType::Trigger;
        edge.style.line_style = LineStyle::Dashed;
        edge
    }

    /// Create a navigation edge
    pub fn navigation(
        from_node: NodeId,
        from_port: impl Into<PortId>,
        to_node: NodeId,
        to_port: impl Into<PortId>,
    ) -> Self {
        let mut edge = Self::new(from_node, from_port, to_node, to_port);
        edge.connection_type = ConnectionType::Navigation;
        edge.style.color = EdgeColor::Blue;
        edge
    }

    /// Create a relationship edge between entities
    pub fn relationship(
        from_node: NodeId,
        to_node: NodeId,
        relation_type: RelationType,
    ) -> Self {
        let mut edge = Self::new(from_node, "entity", to_node, "entity");
        edge.connection_type = ConnectionType::Relationship(relation_type);
        edge.style.color = EdgeColor::Purple;
        edge.style.arrow_end = match relation_type {
            RelationType::OneToOne => ArrowStyle::Arrow,
            RelationType::OneToMany => ArrowStyle::ManyArrow,
            RelationType::ManyToOne => ArrowStyle::Arrow,
            RelationType::ManyToMany => ArrowStyle::ManyArrow,
        };
        edge.style.arrow_start = match relation_type {
            RelationType::OneToOne => ArrowStyle::None,
            RelationType::OneToMany => ArrowStyle::None,
            RelationType::ManyToOne => ArrowStyle::ManyArrow,
            RelationType::ManyToMany => ArrowStyle::ManyArrow,
        };
        edge
    }

    /// Create a dependency edge
    pub fn dependency(from_node: NodeId, to_node: NodeId) -> Self {
        let mut edge = Self::new(from_node, "out", to_node, "in");
        edge.connection_type = ConnectionType::Dependency;
        edge.style.line_style = LineStyle::Dotted;
        edge.style.color = EdgeColor::Gray;
        edge
    }

    // ========== Builder Methods ==========

    /// Set the connection type
    pub fn with_connection_type(mut self, connection_type: ConnectionType) -> Self {
        self.connection_type = connection_type;
        self
    }

    /// Set the data mapping
    pub fn with_mapping(mut self, mapping: DataMapping) -> Self {
        self.data_mapping = Some(mapping);
        self
    }

    /// Set the label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the style
    pub fn with_style(mut self, style: EdgeStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the color
    pub fn with_color(mut self, color: EdgeColor) -> Self {
        self.style.color = color;
        self
    }

    /// Set the line style
    pub fn with_line_style(mut self, line_style: LineStyle) -> Self {
        self.style.line_style = line_style;
        self
    }

    /// Add a waypoint
    pub fn with_waypoint(mut self, x: f32, y: f32) -> Self {
        self.waypoints.push(EdgeWaypoint::new(x, y));
        self
    }

    /// Disable this edge
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Set the z-index
    pub fn with_z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }

    // ========== Query Methods ==========

    /// Check if this edge connects to a specific node
    pub fn connects_to(&self, node_id: NodeId) -> bool {
        self.from_node == node_id || self.to_node == node_id
    }

    /// Check if this edge is between two specific nodes (in either direction)
    pub fn is_between(&self, node_a: NodeId, node_b: NodeId) -> bool {
        (self.from_node == node_a && self.to_node == node_b)
            || (self.from_node == node_b && self.to_node == node_a)
    }

    /// Check if this edge goes from node_a to node_b (direction matters)
    pub fn is_from_to(&self, node_a: NodeId, node_b: NodeId) -> bool {
        self.from_node == node_a && self.to_node == node_b
    }

    /// Check if this is a self-referencing edge (same source and target node)
    pub fn is_self_reference(&self) -> bool {
        self.from_node == self.to_node
    }

    /// Check if this is a data flow edge
    pub fn is_data_flow(&self) -> bool {
        matches!(self.connection_type, ConnectionType::DataFlow)
    }

    /// Check if this is a trigger edge
    pub fn is_trigger(&self) -> bool {
        matches!(self.connection_type, ConnectionType::Trigger)
    }

    /// Check if this is a relationship edge
    pub fn is_relationship(&self) -> bool {
        matches!(self.connection_type, ConnectionType::Relationship(_))
    }

    /// Get the relationship type if this is a relationship edge
    pub fn relationship_type(&self) -> Option<RelationType> {
        match &self.connection_type {
            ConnectionType::Relationship(rt) => Some(*rt),
            _ => None,
        }
    }

    /// Check if this is a navigation edge
    pub fn is_navigation(&self) -> bool {
        matches!(self.connection_type, ConnectionType::Navigation)
    }

    /// Check if this is a dependency edge
    pub fn is_dependency(&self) -> bool {
        matches!(self.connection_type, ConnectionType::Dependency)
    }

    // ========== Mutation Methods ==========

    /// Toggle selection state
    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }

    /// Toggle enabled state
    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Add a waypoint at the specified position
    pub fn add_waypoint(&mut self, x: f32, y: f32) {
        self.waypoints.push(EdgeWaypoint::new(x, y));
    }

    /// Insert a waypoint at a specific index
    pub fn insert_waypoint(&mut self, index: usize, x: f32, y: f32) {
        if index <= self.waypoints.len() {
            self.waypoints.insert(index, EdgeWaypoint::new(x, y));
        }
    }

    /// Remove a waypoint by index
    pub fn remove_waypoint(&mut self, index: usize) -> Option<EdgeWaypoint> {
        if index < self.waypoints.len() {
            Some(self.waypoints.remove(index))
        } else {
            None
        }
    }

    /// Clear all waypoints
    pub fn clear_waypoints(&mut self) {
        self.waypoints.clear();
    }

    /// Update source connection
    pub fn set_source(&mut self, node_id: NodeId, port_id: impl Into<PortId>) {
        self.from_node = node_id;
        self.from_port = port_id.into();
    }

    /// Update target connection
    pub fn set_target(&mut self, node_id: NodeId, port_id: impl Into<PortId>) {
        self.to_node = node_id;
        self.to_port = port_id.into();
    }

    /// Reverse the edge direction
    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.from_node, &mut self.to_node);
        std::mem::swap(&mut self.from_port, &mut self.to_port);
        self.waypoints.reverse();
    }

    /// Clone this edge with a new ID
    pub fn duplicate(&self) -> Self {
        let mut clone = self.clone();
        clone.id = Uuid::new_v4();
        clone.selected = false;
        clone
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            from_node: Uuid::nil(),
            from_port: String::new(),
            to_node: Uuid::nil(),
            to_port: String::new(),
            connection_type: ConnectionType::DataFlow,
            data_mapping: None,
            label: None,
            enabled: true,
            selected: false,
            style: EdgeStyle::default(),
            description: None,
            metadata: HashMap::new(),
            waypoints: Vec::new(),
            z_index: 0,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Edge {}

impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Data mapping configuration for transforming data between ports
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DataMapping {
    /// Field-level mappings (source field -> target field)
    pub field_mappings: HashMap<String, FieldMapping>,

    /// Optional transformation expression or script
    pub transform: Option<String>,

    /// Whether to pass through unmapped fields
    pub pass_through: bool,

    /// Default values for target fields not in source
    pub defaults: HashMap<String, ConfigValue>,

    /// Fields to exclude from mapping
    pub exclude: Vec<String>,
}

impl DataMapping {
    /// Create a new empty data mapping
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a pass-through mapping (all fields mapped as-is)
    pub fn pass_through() -> Self {
        Self {
            pass_through: true,
            ..Default::default()
        }
    }

    /// Add a direct field mapping (same name source to target)
    pub fn map_field(mut self, field: impl Into<String>) -> Self {
        let field_name = field.into();
        self.field_mappings.insert(
            field_name.clone(),
            FieldMapping::direct(field_name),
        );
        self
    }

    /// Add a renamed field mapping (source field -> different target field)
    pub fn map_field_to(
        mut self,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        self.field_mappings.insert(
            source.into(),
            FieldMapping::renamed(target),
        );
        self
    }

    /// Add a transformed field mapping
    pub fn map_field_with_transform(
        mut self,
        source: impl Into<String>,
        target: impl Into<String>,
        transform: impl Into<String>,
    ) -> Self {
        self.field_mappings.insert(
            source.into(),
            FieldMapping::transformed(target, transform),
        );
        self
    }

    /// Set a default value for a target field
    pub fn with_default(mut self, field: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.defaults.insert(field.into(), value.into());
        self
    }

    /// Exclude a field from mapping
    pub fn exclude_field(mut self, field: impl Into<String>) -> Self {
        self.exclude.push(field.into());
        self
    }

    /// Set the transformation expression
    pub fn with_transform(mut self, transform: impl Into<String>) -> Self {
        self.transform = Some(transform.into());
        self
    }
}

/// Individual field mapping configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldMapping {
    /// Target field name
    pub target: String,

    /// Optional transformation expression
    pub transform: Option<String>,

    /// Whether this mapping is enabled
    pub enabled: bool,
}

impl FieldMapping {
    /// Create a direct mapping (same field name, no transform)
    pub fn direct(field: impl Into<String>) -> Self {
        Self {
            target: field.into(),
            transform: None,
            enabled: true,
        }
    }

    /// Create a renamed mapping (different target field name)
    pub fn renamed(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            transform: None,
            enabled: true,
        }
    }

    /// Create a transformed mapping
    pub fn transformed(target: impl Into<String>, transform: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            transform: Some(transform.into()),
            enabled: true,
        }
    }
}

/// Visual style configuration for edges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeStyle {
    /// Color of the edge
    pub color: EdgeColor,

    /// Line style (solid, dashed, dotted)
    pub line_style: LineStyle,

    /// Line thickness
    pub thickness: f32,

    /// Arrow style at the start of the edge
    pub arrow_start: ArrowStyle,

    /// Arrow style at the end of the edge
    pub arrow_end: ArrowStyle,

    /// Whether to animate the edge (for data flow visualization)
    pub animated: bool,

    /// Curve style
    pub curve: CurveStyle,

    /// Opacity (0.0 - 1.0)
    pub opacity: f32,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            color: EdgeColor::Default,
            line_style: LineStyle::Solid,
            thickness: 2.0,
            arrow_start: ArrowStyle::None,
            arrow_end: ArrowStyle::Arrow,
            animated: false,
            curve: CurveStyle::Bezier,
            opacity: 1.0,
        }
    }
}

/// Edge color options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeColor {
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
    White,
}

impl EdgeColor {
    /// Get the RGB color values (0-255)
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            EdgeColor::Default => (150, 150, 150),
            EdgeColor::Red => (239, 68, 68),
            EdgeColor::Orange => (249, 115, 22),
            EdgeColor::Yellow => (234, 179, 8),
            EdgeColor::Green => (34, 197, 94),
            EdgeColor::Teal => (20, 184, 166),
            EdgeColor::Blue => (59, 130, 246),
            EdgeColor::Indigo => (99, 102, 241),
            EdgeColor::Purple => (168, 85, 247),
            EdgeColor::Pink => (236, 72, 153),
            EdgeColor::Gray => (107, 114, 128),
            EdgeColor::White => (255, 255, 255),
        }
    }

    /// Get the color as a hex string
    pub fn hex(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl Default for EdgeColor {
    fn default() -> Self {
        EdgeColor::Default
    }
}

/// Line style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

/// Arrow style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArrowStyle {
    #[default]
    None,
    Arrow,
    FilledArrow,
    Diamond,
    FilledDiamond,
    Circle,
    FilledCircle,
    ManyArrow, // For one-to-many relationships (crow's foot)
}

/// Curve style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CurveStyle {
    /// Straight line
    Straight,
    /// Smooth bezier curve
    #[default]
    Bezier,
    /// Stepped/orthogonal routing
    Orthogonal,
    /// Smooth curve through waypoints
    Smooth,
}

/// Waypoint for custom edge routing
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EdgeWaypoint {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Whether this waypoint is locked (can't be auto-adjusted)
    pub locked: bool,
}

impl EdgeWaypoint {
    /// Create a new waypoint
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, locked: false }
    }

    /// Create a locked waypoint
    pub fn locked(x: f32, y: f32) -> Self {
        Self { x, y, locked: true }
    }

    /// Get position as tuple
    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Distance to another waypoint
    pub fn distance_to(&self, other: &EdgeWaypoint) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let from_node = Uuid::new_v4();
        let to_node = Uuid::new_v4();

        let edge = Edge::new(from_node, "output", to_node, "input");

        assert_eq!(edge.from_node, from_node);
        assert_eq!(edge.from_port, "output");
        assert_eq!(edge.to_node, to_node);
        assert_eq!(edge.to_port, "input");
        assert!(edge.enabled);
    }

    #[test]
    fn test_edge_types() {
        let from_node = Uuid::new_v4();
        let to_node = Uuid::new_v4();

        let data_edge = Edge::data_flow(from_node, "out", to_node, "in");
        assert!(data_edge.is_data_flow());

        let trigger_edge = Edge::trigger(from_node, "trigger_out", to_node, "trigger_in");
        assert!(trigger_edge.is_trigger());

        let rel_edge = Edge::relationship(from_node, to_node, RelationType::OneToMany);
        assert!(rel_edge.is_relationship());
        assert_eq!(rel_edge.relationship_type(), Some(RelationType::OneToMany));
    }

    #[test]
    fn test_edge_connects_to() {
        let node_a = Uuid::new_v4();
        let node_b = Uuid::new_v4();
        let node_c = Uuid::new_v4();

        let edge = Edge::new(node_a, "out", node_b, "in");

        assert!(edge.connects_to(node_a));
        assert!(edge.connects_to(node_b));
        assert!(!edge.connects_to(node_c));
    }

    #[test]
    fn test_edge_builder() {
        let edge = Edge::new(Uuid::new_v4(), "out", Uuid::new_v4(), "in")
            .with_label("Test Edge")
            .with_color(EdgeColor::Blue)
            .with_line_style(LineStyle::Dashed)
            .with_waypoint(100.0, 50.0);

        assert_eq!(edge.label, Some("Test Edge".to_string()));
        assert_eq!(edge.style.color, EdgeColor::Blue);
        assert_eq!(edge.style.line_style, LineStyle::Dashed);
        assert_eq!(edge.waypoints.len(), 1);
    }

    #[test]
    fn test_data_mapping() {
        let mapping = DataMapping::new()
            .map_field("id")
            .map_field_to("email", "user_email")
            .map_field_with_transform("name", "full_name", "uppercase(name)")
            .with_default("status", "active");

        assert!(mapping.field_mappings.contains_key("id"));
        assert_eq!(
            mapping.field_mappings.get("email").unwrap().target,
            "user_email"
        );
        assert!(mapping.defaults.contains_key("status"));
    }

    #[test]
    fn test_edge_reverse() {
        let node_a = Uuid::new_v4();
        let node_b = Uuid::new_v4();

        let mut edge = Edge::new(node_a, "out", node_b, "in");
        edge.reverse();

        assert_eq!(edge.from_node, node_b);
        assert_eq!(edge.to_node, node_a);
        assert_eq!(edge.from_port, "in");
        assert_eq!(edge.to_port, "out");
    }

    #[test]
    fn test_edge_waypoints() {
        let mut edge = Edge::new(Uuid::new_v4(), "out", Uuid::new_v4(), "in");

        edge.add_waypoint(100.0, 50.0);
        edge.add_waypoint(200.0, 75.0);
        assert_eq!(edge.waypoints.len(), 2);

        edge.insert_waypoint(1, 150.0, 60.0);
        assert_eq!(edge.waypoints.len(), 3);
        assert_eq!(edge.waypoints[1].x, 150.0);

        edge.remove_waypoint(1);
        assert_eq!(edge.waypoints.len(), 2);

        edge.clear_waypoints();
        assert!(edge.waypoints.is_empty());
    }

    #[test]
    fn test_edge_color() {
        let color = EdgeColor::Blue;
        assert_eq!(color.rgb(), (59, 130, 246));
        assert_eq!(color.hex(), "#3B82F6");
    }

    #[test]
    fn test_edge_self_reference() {
        let node = Uuid::new_v4();
        let self_edge = Edge::new(node, "out", node, "in");
        assert!(self_edge.is_self_reference());

        let normal_edge = Edge::new(Uuid::new_v4(), "out", Uuid::new_v4(), "in");
        assert!(!normal_edge.is_self_reference());
    }

    #[test]
    fn test_edge_duplicate() {
        let original = Edge::new(Uuid::new_v4(), "out", Uuid::new_v4(), "in")
            .with_label("Original");

        let duplicate = original.duplicate();

        assert_ne!(original.id, duplicate.id);
        assert_eq!(original.from_node, duplicate.from_node);
        assert_eq!(original.label, duplicate.label);
    }
}
