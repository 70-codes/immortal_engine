//! Group type for visual organization of nodes
//!
//! Groups allow users to visually organize related nodes on the canvas.
//! Groups can contain nodes and can be nested within other groups.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use imortal_core::{ConfigValue, NodeId, Position, Size};

/// A group for organizing nodes on the canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Unique identifier for this group
    pub id: Uuid,

    /// User-defined name for this group
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Position on the canvas (top-left corner)
    pub position: Position,

    /// Size of the group
    pub size: Size,

    /// IDs of nodes contained in this group
    pub node_ids: HashSet<NodeId>,

    /// Parent group ID (for nested groups)
    pub parent_id: Option<Uuid>,

    /// Whether the group is collapsed (hides contents)
    pub collapsed: bool,

    /// Whether the group is locked (cannot be modified)
    pub locked: bool,

    /// Whether the group is visible
    pub visible: bool,

    /// Whether the group is selected
    #[serde(skip)]
    pub selected: bool,

    /// Background color
    pub color: GroupColor,

    /// Opacity of the group background (0.0 - 1.0)
    pub opacity: f32,

    /// Border style
    pub border_style: BorderStyle,

    /// Whether to show the group header/title
    pub show_header: bool,

    /// Custom icon for the group
    pub icon: Option<String>,

    /// Tags for filtering and organization
    pub tags: Vec<String>,

    /// Custom metadata
    pub metadata: std::collections::HashMap<String, ConfigValue>,

    /// Z-index for layering (lower = behind nodes)
    pub z_index: i32,

    /// Padding inside the group
    pub padding: f32,

    /// Whether to auto-resize to fit contents
    pub auto_resize: bool,
}

impl Group {
    /// Create a new group with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            position: Position::default(),
            size: Size::new(300.0, 200.0),
            node_ids: HashSet::new(),
            parent_id: None,
            collapsed: false,
            locked: false,
            visible: true,
            selected: false,
            color: GroupColor::Default,
            opacity: 0.1,
            border_style: BorderStyle::Solid,
            show_header: true,
            icon: None,
            tags: Vec::new(),
            metadata: std::collections::HashMap::new(),
            z_index: -1, // Behind nodes by default
            padding: 20.0,
            auto_resize: true,
        }
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

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the color
    pub fn with_color(mut self, color: GroupColor) -> Self {
        self.color = color;
        self
    }

    /// Set the opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the border style
    pub fn with_border(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set the parent group
    pub fn with_parent(mut self, parent_id: Uuid) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Add a node to this group
    pub fn with_node(mut self, node_id: NodeId) -> Self {
        self.node_ids.insert(node_id);
        self
    }

    /// Add multiple nodes to this group
    pub fn with_nodes(mut self, node_ids: impl IntoIterator<Item = NodeId>) -> Self {
        self.node_ids.extend(node_ids);
        self
    }

    /// Set the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
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

    /// Set collapsed state
    pub fn collapsed(mut self) -> Self {
        self.collapsed = true;
        self
    }

    /// Disable auto-resize
    pub fn fixed_size(mut self) -> Self {
        self.auto_resize = false;
        self
    }

    /// Hide the header
    pub fn without_header(mut self) -> Self {
        self.show_header = false;
        self
    }

    /// Set the padding
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the z-index
    pub fn with_z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }

    // ========== Node Management ==========

    /// Add a node to this group
    pub fn add_node(&mut self, node_id: NodeId) {
        self.node_ids.insert(node_id);
    }

    /// Remove a node from this group
    pub fn remove_node(&mut self, node_id: &NodeId) -> bool {
        self.node_ids.remove(node_id)
    }

    /// Check if this group contains a specific node
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.node_ids.contains(node_id)
    }

    /// Get the number of nodes in this group
    pub fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    /// Check if this group is empty (no nodes)
    pub fn is_empty(&self) -> bool {
        self.node_ids.is_empty()
    }

    /// Clear all nodes from this group
    pub fn clear_nodes(&mut self) {
        self.node_ids.clear();
    }

    /// Get an iterator over node IDs
    pub fn nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.node_ids.iter()
    }

    // ========== Query Methods ==========

    /// Check if this group has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Check if this group is a child of another group
    pub fn is_child_of(&self, parent_id: Uuid) -> bool {
        self.parent_id == Some(parent_id)
    }

    /// Check if this is a root-level group (no parent)
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Get the center position of the group
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

    /// Check if a point is inside this group
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.x
            && x <= self.position.x + self.size.width
            && y >= self.position.y
            && y <= self.position.y + self.size.height
    }

    /// Check if a rectangle intersects with this group
    pub fn intersects_rect(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        self.position.x < x + width
            && self.position.x + self.size.width > x
            && self.position.y < y + height
            && self.position.y + self.size.height > y
    }

    /// Get the content area (inside padding)
    pub fn content_bounds(&self) -> (f32, f32, f32, f32) {
        (
            self.position.x + self.padding,
            self.position.y + self.padding + if self.show_header { 30.0 } else { 0.0 },
            self.size.width - self.padding * 2.0,
            self.size.height - self.padding * 2.0 - if self.show_header { 30.0 } else { 0.0 },
        )
    }

    // ========== Mutation Methods ==========

    /// Move the group by a delta
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

    /// Resize the group
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

    /// Calculate bounds from contained nodes
    /// Returns the bounding box that would contain all the given node positions
    pub fn calculate_bounds_from_nodes(
        &self,
        node_positions: impl IntoIterator<Item = (f32, f32, f32, f32)>, // (x, y, width, height)
    ) -> Option<(f32, f32, f32, f32)> {
        let positions: Vec<_> = node_positions.into_iter().collect();

        if positions.is_empty() {
            return None;
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for (x, y, w, h) in positions {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + w);
            max_y = max_y.max(y + h);
        }

        // Add padding and header space
        let header_height = if self.show_header { 30.0 } else { 0.0 };

        Some((
            min_x - self.padding,
            min_y - self.padding - header_height,
            (max_x - min_x) + self.padding * 2.0,
            (max_y - min_y) + self.padding * 2.0 + header_height,
        ))
    }

    /// Update position and size from calculated bounds
    pub fn fit_to_bounds(&mut self, bounds: (f32, f32, f32, f32)) {
        if !self.locked && self.auto_resize {
            let (x, y, w, h) = bounds;
            self.position = Position::new(x, y);
            self.size = Size::new(w.max(100.0), h.max(50.0));
        }
    }

    /// Clone this group with a new ID
    pub fn duplicate(&self) -> Self {
        let mut clone = self.clone();
        clone.id = Uuid::new_v4();
        clone.name = format!("{} (copy)", self.name);
        clone.position.x += 20.0;
        clone.position.y += 20.0;
        clone.selected = false;
        clone.node_ids.clear(); // Don't copy node references
        clone
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new("New Group")
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Group {}

impl std::hash::Hash for Group {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Color options for groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GroupColor {
    #[default]
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

impl GroupColor {
    /// Get the RGB color values (0-255)
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            GroupColor::Default => (100, 100, 100),
            GroupColor::Red => (239, 68, 68),
            GroupColor::Orange => (249, 115, 22),
            GroupColor::Yellow => (234, 179, 8),
            GroupColor::Green => (34, 197, 94),
            GroupColor::Teal => (20, 184, 166),
            GroupColor::Blue => (59, 130, 246),
            GroupColor::Indigo => (99, 102, 241),
            GroupColor::Purple => (168, 85, 247),
            GroupColor::Pink => (236, 72, 153),
            GroupColor::Gray => (107, 114, 128),
        }
    }

    /// Get the color as a hex string
    pub fn hex(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Get all available colors
    pub fn all() -> &'static [GroupColor] {
        &[
            GroupColor::Default,
            GroupColor::Red,
            GroupColor::Orange,
            GroupColor::Yellow,
            GroupColor::Green,
            GroupColor::Teal,
            GroupColor::Blue,
            GroupColor::Indigo,
            GroupColor::Purple,
            GroupColor::Pink,
            GroupColor::Gray,
        ]
    }
}

/// Border style options for groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BorderStyle {
    /// No border
    None,
    /// Solid line border
    #[default]
    Solid,
    /// Dashed line border
    Dashed,
    /// Dotted line border
    Dotted,
    /// Double line border
    Double,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_creation() {
        let group = Group::new("Test Group");
        assert_eq!(group.name, "Test Group");
        assert!(group.is_empty());
        assert!(group.is_root());
        assert!(!group.locked);
    }

    #[test]
    fn test_group_builder() {
        let group = Group::new("My Group")
            .with_position(100.0, 200.0)
            .with_size(400.0, 300.0)
            .with_color(GroupColor::Blue)
            .with_opacity(0.2)
            .with_tag("test");

        assert_eq!(group.position.x, 100.0);
        assert_eq!(group.position.y, 200.0);
        assert_eq!(group.size.width, 400.0);
        assert_eq!(group.color, GroupColor::Blue);
        assert!((group.opacity - 0.2).abs() < f32::EPSILON);
        assert!(group.has_tag("test"));
    }

    #[test]
    fn test_group_node_management() {
        let mut group = Group::new("Test");
        let node1 = Uuid::new_v4();
        let node2 = Uuid::new_v4();

        group.add_node(node1);
        group.add_node(node2);

        assert_eq!(group.node_count(), 2);
        assert!(group.contains_node(&node1));
        assert!(group.contains_node(&node2));

        group.remove_node(&node1);
        assert_eq!(group.node_count(), 1);
        assert!(!group.contains_node(&node1));
    }

    #[test]
    fn test_group_contains_point() {
        let group = Group::new("Test")
            .with_position(100.0, 100.0)
            .with_size(200.0, 150.0);

        assert!(group.contains_point(150.0, 150.0));
        assert!(group.contains_point(100.0, 100.0)); // Edge
        assert!(!group.contains_point(50.0, 50.0));
        assert!(!group.contains_point(350.0, 150.0));
    }

    #[test]
    fn test_group_translate() {
        let mut group = Group::new("Test")
            .with_position(100.0, 100.0);

        group.translate(50.0, -25.0);
        assert_eq!(group.position.x, 150.0);
        assert_eq!(group.position.y, 75.0);
    }

    #[test]
    fn test_locked_group() {
        let mut group = Group::new("Test")
            .with_position(100.0, 100.0)
            .locked();

        group.translate(50.0, 50.0);
        // Position should not change when locked
        assert_eq!(group.position.x, 100.0);
        assert_eq!(group.position.y, 100.0);
    }

    #[test]
    fn test_group_duplicate() {
        let mut original = Group::new("Original")
            .with_position(100.0, 100.0)
            .with_color(GroupColor::Red);

        original.add_node(Uuid::new_v4());

        let duplicate = original.duplicate();

        assert_ne!(original.id, duplicate.id);
        assert!(duplicate.name.contains("copy"));
        assert_eq!(duplicate.position.x, 120.0);
        assert_eq!(duplicate.color, GroupColor::Red);
        assert!(duplicate.is_empty()); // Nodes not copied
    }

    #[test]
    fn test_group_color() {
        let color = GroupColor::Blue;
        assert_eq!(color.rgb(), (59, 130, 246));
        assert_eq!(color.hex(), "#3B82F6");
    }

    #[test]
    fn test_group_parent() {
        let parent_id = Uuid::new_v4();
        let group = Group::new("Child").with_parent(parent_id);

        assert!(!group.is_root());
        assert!(group.is_child_of(parent_id));
    }

    #[test]
    fn test_calculate_bounds() {
        let group = Group::new("Test")
            .with_padding(10.0)
            .without_header();

        let nodes = vec![
            (100.0, 100.0, 50.0, 50.0),
            (200.0, 150.0, 50.0, 50.0),
        ];

        let bounds = group.calculate_bounds_from_nodes(nodes);
        assert!(bounds.is_some());

        let (x, y, w, h) = bounds.unwrap();
        assert_eq!(x, 90.0); // 100 - 10 padding
        assert_eq!(y, 90.0); // 100 - 10 padding
        assert_eq!(w, 170.0); // (250 - 100) + 20 padding
        assert_eq!(h, 120.0); // (200 - 100) + 20 padding
    }
}
