//! ProjectGraph - the main graph structure for Immortal Engine projects
//!
//! The ProjectGraph is the central data structure that holds all nodes (components),
//! edges (connections), and groups for a visual project. It provides methods for
//! managing the graph, querying relationships, and validating the structure.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use imortal_core::{EdgeId, NodeId, EngineError, EngineResult, RelationType, ConnectionType};

use crate::edge::Edge;
use crate::group::Group;
use crate::node::Node;
use crate::project::ProjectMeta;

/// The main graph structure for an Immortal Engine project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGraph {
    /// Project metadata
    pub meta: ProjectMeta,

    /// All nodes in the graph, keyed by their ID
    pub nodes: HashMap<NodeId, Node>,

    /// All edges in the graph, keyed by their ID
    pub edges: HashMap<EdgeId, Edge>,

    /// All groups in the graph, keyed by their ID
    pub groups: HashMap<Uuid, Group>,

    /// Currently selected node IDs
    #[serde(skip)]
    pub selected_nodes: HashSet<NodeId>,

    /// Currently selected edge IDs
    #[serde(skip)]
    pub selected_edges: HashSet<EdgeId>,

    /// Currently selected group IDs
    #[serde(skip)]
    pub selected_groups: HashSet<Uuid>,

    /// Canvas viewport state
    pub viewport: Viewport,

    /// Whether the graph has been modified since last save
    #[serde(skip)]
    pub dirty: bool,
}

impl ProjectGraph {
    /// Create a new empty project graph with the given metadata
    pub fn new(meta: ProjectMeta) -> Self {
        Self {
            meta,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            groups: HashMap::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            selected_groups: HashSet::new(),
            viewport: Viewport::default(),
            dirty: false,
        }
    }

    /// Create a new project with just a name
    pub fn with_name(name: impl Into<String>) -> Self {
        Self::new(ProjectMeta::new(name))
    }

    // ========== Node Operations ==========

    /// Add a node to the graph and return its ID
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = node.id;
        self.nodes.insert(id, node);
        self.dirty = true;
        id
    }

    /// Create and add a node, returning its ID
    pub fn create_node(&mut self, component_type: impl Into<String>, name: impl Into<String>) -> NodeId {
        let node = Node::new(component_type, name);
        self.add_node(node)
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.dirty = true;
        self.nodes.get_mut(&id)
    }

    /// Remove a node and all its connected edges
    pub fn remove_node(&mut self, id: NodeId) -> Option<Node> {
        // Remove all edges connected to this node
        self.remove_edges_for_node(id);

        // Remove from any groups
        for group in self.groups.values_mut() {
            group.remove_node(&id);
        }

        // Remove from selection
        self.selected_nodes.remove(&id);

        self.dirty = true;
        self.nodes.remove(&id)
    }

    /// Check if a node exists
    pub fn has_node(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// Get the number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get all nodes as an iterator
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }

    /// Find nodes by component type
    pub fn find_nodes_by_type(&self, component_type: &str) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| n.component_type == component_type)
            .collect()
    }

    /// Find nodes by name (partial match)
    pub fn find_nodes_by_name(&self, name: &str) -> Vec<&Node> {
        let name_lower = name.to_lowercase();
        self.nodes
            .values()
            .filter(|n| n.name.to_lowercase().contains(&name_lower))
            .collect()
    }

    /// Find nodes at a specific position (point)
    pub fn find_nodes_at(&self, x: f32, y: f32) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| n.contains_point(x, y))
            .collect()
    }

    /// Find nodes within a rectangular area
    pub fn find_nodes_in_rect(&self, x: f32, y: f32, width: f32, height: f32) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| {
                let (nx, ny, nw, nh) = n.bounds();
                nx < x + width && nx + nw > x && ny < y + height && ny + nh > y
            })
            .collect()
    }

    /// Duplicate a node (creates a copy with new ID)
    pub fn duplicate_node(&mut self, id: NodeId) -> Option<NodeId> {
        let node = self.nodes.get(&id)?.duplicate();
        let new_id = node.id;
        self.nodes.insert(new_id, node);
        self.dirty = true;
        Some(new_id)
    }

    // ========== Edge Operations ==========

    /// Add an edge to the graph and return its ID
    pub fn add_edge(&mut self, edge: Edge) -> EngineResult<EdgeId> {
        // Validate that both nodes exist
        if !self.has_node(edge.from_node) {
            return Err(EngineError::NodeNotFound(edge.from_node.to_string()));
        }
        if !self.has_node(edge.to_node) {
            return Err(EngineError::NodeNotFound(edge.to_node.to_string()));
        }

        // Skip port validation for relationship and dependency edges
        // (they connect entities/nodes directly rather than specific ports)
        let skip_port_validation = matches!(
            edge.connection_type,
            ConnectionType::Relationship(_) | ConnectionType::Dependency
        );

        if !skip_port_validation {
            // Validate that ports exist on the nodes
            if let Some(from_node) = self.get_node(edge.from_node) {
                if from_node.get_output_port(&edge.from_port).is_none() {
                    return Err(EngineError::PortNotFound {
                        node_id: edge.from_node.to_string(),
                        port_id: edge.from_port.clone(),
                    });
                }
            }
            if let Some(to_node) = self.get_node(edge.to_node) {
                if to_node.get_input_port(&edge.to_port).is_none() {
                    return Err(EngineError::PortNotFound {
                        node_id: edge.to_node.to_string(),
                        port_id: edge.to_port.clone(),
                    });
                }
            }
        }

        let id = edge.id;
        self.edges.insert(id, edge);
        self.dirty = true;
        Ok(id)
    }

    /// Create and add a data flow edge between two ports
    pub fn connect(
        &mut self,
        from_node: NodeId,
        from_port: impl Into<String>,
        to_node: NodeId,
        to_port: impl Into<String>,
    ) -> EngineResult<EdgeId> {
        let edge = Edge::data_flow(from_node, from_port, to_node, to_port);
        self.add_edge(edge)
    }

    /// Create a relationship edge between two entity nodes
    pub fn add_relationship(
        &mut self,
        from_node: NodeId,
        to_node: NodeId,
        relation_type: RelationType,
    ) -> EngineResult<EdgeId> {
        let edge = Edge::relationship(from_node, to_node, relation_type);
        self.add_edge(edge)
    }

    /// Get an edge by ID
    pub fn get_edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(&id)
    }

    /// Get a mutable edge by ID
    pub fn get_edge_mut(&mut self, id: EdgeId) -> Option<&mut Edge> {
        self.dirty = true;
        self.edges.get_mut(&id)
    }

    /// Remove an edge
    pub fn remove_edge(&mut self, id: EdgeId) -> Option<Edge> {
        self.selected_edges.remove(&id);
        self.dirty = true;
        self.edges.remove(&id)
    }

    /// Remove all edges connected to a node
    pub fn remove_edges_for_node(&mut self, node_id: NodeId) {
        let edges_to_remove: Vec<EdgeId> = self
            .edges
            .values()
            .filter(|e| e.connects_to(node_id))
            .map(|e| e.id)
            .collect();

        for edge_id in edges_to_remove {
            self.remove_edge(edge_id);
        }
    }

    /// Check if an edge exists
    pub fn has_edge(&self, id: EdgeId) -> bool {
        self.edges.contains_key(&id)
    }

    /// Get the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Get all edges as an iterator
    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
    }

    /// Get all edges connected to a node (either direction)
    pub fn edges_for_node(&self, node_id: NodeId) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.connects_to(node_id))
            .collect()
    }

    /// Get incoming edges for a node
    pub fn incoming_edges(&self, node_id: NodeId) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.to_node == node_id)
            .collect()
    }

    /// Get outgoing edges from a node
    pub fn outgoing_edges(&self, node_id: NodeId) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.from_node == node_id)
            .collect()
    }

    /// Get edges between two specific nodes
    pub fn edges_between(&self, node_a: NodeId, node_b: NodeId) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.is_between(node_a, node_b))
            .collect()
    }

    /// Check if two nodes are connected (directly)
    pub fn are_connected(&self, node_a: NodeId, node_b: NodeId) -> bool {
        self.edges
            .values()
            .any(|e| e.is_between(node_a, node_b))
    }

    /// Get all nodes connected to a given node
    pub fn connected_nodes(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut connected = HashSet::new();
        for edge in self.edges.values() {
            if edge.from_node == node_id {
                connected.insert(edge.to_node);
            }
            if edge.to_node == node_id {
                connected.insert(edge.from_node);
            }
        }
        connected
    }

    /// Find nodes that provide data to the given node
    pub fn upstream_nodes(&self, node_id: NodeId) -> HashSet<NodeId> {
        self.edges
            .values()
            .filter(|e| e.to_node == node_id)
            .map(|e| e.from_node)
            .collect()
    }

    /// Find nodes that receive data from the given node
    pub fn downstream_nodes(&self, node_id: NodeId) -> HashSet<NodeId> {
        self.edges
            .values()
            .filter(|e| e.from_node == node_id)
            .map(|e| e.to_node)
            .collect()
    }

    // ========== Group Operations ==========

    /// Add a group to the graph
    pub fn add_group(&mut self, group: Group) -> Uuid {
        let id = group.id;
        self.groups.insert(id, group);
        self.dirty = true;
        id
    }

    /// Create and add a new group
    pub fn create_group(&mut self, name: impl Into<String>) -> Uuid {
        let group = Group::new(name);
        self.add_group(group)
    }

    /// Get a group by ID
    pub fn get_group(&self, id: Uuid) -> Option<&Group> {
        self.groups.get(&id)
    }

    /// Get a mutable group by ID
    pub fn get_group_mut(&mut self, id: Uuid) -> Option<&mut Group> {
        self.dirty = true;
        self.groups.get_mut(&id)
    }

    /// Remove a group (does not remove contained nodes)
    pub fn remove_group(&mut self, id: Uuid) -> Option<Group> {
        // Clear group reference from nodes
        if let Some(group) = self.groups.get(&id) {
            for node_id in group.node_ids.iter() {
                if let Some(node) = self.nodes.get_mut(node_id) {
                    if node.group_id == Some(id) {
                        node.group_id = None;
                    }
                }
            }
        }

        // Update child groups
        for group in self.groups.values_mut() {
            if group.parent_id == Some(id) {
                group.parent_id = None;
            }
        }

        self.selected_groups.remove(&id);
        self.dirty = true;
        self.groups.remove(&id)
    }

    /// Get the number of groups
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Get all groups as an iterator
    pub fn groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.values()
    }

    /// Add a node to a group
    pub fn add_node_to_group(&mut self, node_id: NodeId, group_id: Uuid) -> EngineResult<()> {
        if !self.has_node(node_id) {
            return Err(EngineError::NodeNotFound(node_id.to_string()));
        }
        if !self.groups.contains_key(&group_id) {
            return Err(EngineError::Custom(format!("Group not found: {}", group_id)));
        }

        // Update the node's group reference
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.group_id = Some(group_id);
        }

        // Add to group's node list
        if let Some(group) = self.groups.get_mut(&group_id) {
            group.add_node(node_id);
        }

        self.dirty = true;
        Ok(())
    }

    /// Remove a node from its group
    pub fn remove_node_from_group(&mut self, node_id: NodeId) -> EngineResult<()> {
        let group_id = self.nodes.get(&node_id)
            .ok_or_else(|| EngineError::NodeNotFound(node_id.to_string()))?
            .group_id;

        if let Some(gid) = group_id {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.group_id = None;
            }
            if let Some(group) = self.groups.get_mut(&gid) {
                group.remove_node(&node_id);
            }
            self.dirty = true;
        }

        Ok(())
    }

    /// Create a group from selected nodes
    pub fn group_selected_nodes(&mut self, name: impl Into<String>) -> Option<Uuid> {
        if self.selected_nodes.is_empty() {
            return None;
        }

        let mut group = Group::new(name);

        // Add all selected nodes to the group
        for node_id in &self.selected_nodes {
            group.add_node(*node_id);
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.group_id = Some(group.id);
            }
        }

        // Calculate bounds from node positions
        let node_bounds: Vec<_> = self.selected_nodes
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .map(|n| n.bounds())
            .collect();

        if let Some(bounds) = group.calculate_bounds_from_nodes(node_bounds) {
            group.fit_to_bounds(bounds);
        }

        let id = group.id;
        self.groups.insert(id, group);
        self.dirty = true;
        Some(id)
    }

    /// Ungroup a group (remove the group but keep the nodes)
    pub fn ungroup(&mut self, group_id: Uuid) -> EngineResult<Vec<NodeId>> {
        let group = self.groups.get(&group_id)
            .ok_or_else(|| EngineError::Custom(format!("Group not found: {}", group_id)))?;

        let node_ids: Vec<NodeId> = group.node_ids.iter().copied().collect();

        // Clear group reference from nodes
        for node_id in &node_ids {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.group_id = None;
            }
        }

        self.groups.remove(&group_id);
        self.selected_groups.remove(&group_id);
        self.dirty = true;

        Ok(node_ids)
    }

    // ========== Selection Operations ==========

    /// Select a node
    pub fn select_node(&mut self, id: NodeId) {
        if self.has_node(id) {
            self.selected_nodes.insert(id);
            if let Some(node) = self.nodes.get_mut(&id) {
                node.selected = true;
            }
        }
    }

    /// Deselect a node
    pub fn deselect_node(&mut self, id: NodeId) {
        self.selected_nodes.remove(&id);
        if let Some(node) = self.nodes.get_mut(&id) {
            node.selected = false;
        }
    }

    /// Toggle node selection
    pub fn toggle_node_selection(&mut self, id: NodeId) {
        if self.selected_nodes.contains(&id) {
            self.deselect_node(id);
        } else {
            self.select_node(id);
        }
    }

    /// Select an edge
    pub fn select_edge(&mut self, id: EdgeId) {
        if self.has_edge(id) {
            self.selected_edges.insert(id);
            if let Some(edge) = self.edges.get_mut(&id) {
                edge.selected = true;
            }
        }
    }

    /// Deselect an edge
    pub fn deselect_edge(&mut self, id: EdgeId) {
        self.selected_edges.remove(&id);
        if let Some(edge) = self.edges.get_mut(&id) {
            edge.selected = false;
        }
    }

    /// Select a group
    pub fn select_group(&mut self, id: Uuid) {
        if self.groups.contains_key(&id) {
            self.selected_groups.insert(id);
            if let Some(group) = self.groups.get_mut(&id) {
                group.selected = true;
            }
        }
    }

    /// Deselect a group
    pub fn deselect_group(&mut self, id: Uuid) {
        self.selected_groups.remove(&id);
        if let Some(group) = self.groups.get_mut(&id) {
            group.selected = false;
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        for id in self.selected_nodes.drain() {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.selected = false;
            }
        }
        for id in self.selected_edges.drain() {
            if let Some(edge) = self.edges.get_mut(&id) {
                edge.selected = false;
            }
        }
        for id in self.selected_groups.drain() {
            if let Some(group) = self.groups.get_mut(&id) {
                group.selected = false;
            }
        }
    }

    /// Select all nodes
    pub fn select_all_nodes(&mut self) {
        for (id, node) in self.nodes.iter_mut() {
            node.selected = true;
            self.selected_nodes.insert(*id);
        }
    }

    /// Select all items
    pub fn select_all(&mut self) {
        self.select_all_nodes();
        for (id, edge) in self.edges.iter_mut() {
            edge.selected = true;
            self.selected_edges.insert(*id);
        }
        for (id, group) in self.groups.iter_mut() {
            group.selected = true;
            self.selected_groups.insert(*id);
        }
    }

    /// Get the number of selected items
    pub fn selection_count(&self) -> usize {
        self.selected_nodes.len() + self.selected_edges.len() + self.selected_groups.len()
    }

    /// Check if anything is selected
    pub fn has_selection(&self) -> bool {
        !self.selected_nodes.is_empty()
            || !self.selected_edges.is_empty()
            || !self.selected_groups.is_empty()
    }

    /// Select nodes within a rectangular area
    pub fn select_nodes_in_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let nodes_in_rect: Vec<NodeId> = self.find_nodes_in_rect(x, y, width, height)
            .iter()
            .map(|n| n.id)
            .collect();

        for id in nodes_in_rect {
            self.select_node(id);
        }
    }

    /// Delete all selected items
    pub fn delete_selected(&mut self) {
        // Delete selected edges first (nodes deletion will also remove edges)
        let edges_to_delete: Vec<EdgeId> = self.selected_edges.iter().copied().collect();
        for id in edges_to_delete {
            self.remove_edge(id);
        }

        // Delete selected nodes
        let nodes_to_delete: Vec<NodeId> = self.selected_nodes.iter().copied().collect();
        for id in nodes_to_delete {
            self.remove_node(id);
        }

        // Delete selected groups
        let groups_to_delete: Vec<Uuid> = self.selected_groups.iter().copied().collect();
        for id in groups_to_delete {
            self.remove_group(id);
        }
    }

    /// Duplicate all selected nodes
    pub fn duplicate_selected(&mut self) -> Vec<NodeId> {
        let nodes_to_duplicate: Vec<NodeId> = self.selected_nodes.iter().copied().collect();
        let mut new_ids = Vec::new();

        // Create mapping from old to new IDs
        let mut id_mapping: HashMap<NodeId, NodeId> = HashMap::new();

        // Duplicate nodes
        for old_id in &nodes_to_duplicate {
            if let Some(new_id) = self.duplicate_node(*old_id) {
                id_mapping.insert(*old_id, new_id);
                new_ids.push(new_id);
            }
        }

        // Duplicate edges between selected nodes
        let edges_to_duplicate: Vec<Edge> = self.edges.values()
            .filter(|e| {
                nodes_to_duplicate.contains(&e.from_node)
                    && nodes_to_duplicate.contains(&e.to_node)
            })
            .cloned()
            .collect();

        for edge in edges_to_duplicate {
            if let (Some(&new_from), Some(&new_to)) = (
                id_mapping.get(&edge.from_node),
                id_mapping.get(&edge.to_node),
            ) {
                let mut new_edge = edge.duplicate();
                new_edge.from_node = new_from;
                new_edge.to_node = new_to;
                let _ = self.add_edge(new_edge);
            }
        }

        // Update selection to new nodes
        self.clear_selection();
        for id in &new_ids {
            self.select_node(*id);
        }

        new_ids
    }

    // ========== Viewport Operations ==========

    /// Set the viewport pan position
    pub fn set_pan(&mut self, x: f32, y: f32) {
        self.viewport.pan_x = x;
        self.viewport.pan_y = y;
    }

    /// Pan the viewport by a delta
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.viewport.pan_x += dx;
        self.viewport.pan_y += dy;
    }

    /// Set the viewport zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.viewport.zoom = zoom.clamp(0.1, 5.0);
    }

    /// Zoom the viewport by a factor
    pub fn zoom(&mut self, factor: f32) {
        self.set_zoom(self.viewport.zoom * factor);
    }

    /// Reset the viewport to default
    pub fn reset_viewport(&mut self) {
        self.viewport = Viewport::default();
    }

    /// Fit the viewport to show all nodes
    pub fn fit_to_content(&mut self, canvas_width: f32, canvas_height: f32) {
        if self.nodes.is_empty() {
            self.reset_viewport();
            return;
        }

        // Calculate bounds of all nodes
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for node in self.nodes.values() {
            let (x, y, w, h) = node.bounds();
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + w);
            max_y = max_y.max(y + h);
        }

        // Add padding
        let padding = 50.0;
        min_x -= padding;
        min_y -= padding;
        max_x += padding;
        max_y += padding;

        let content_width = max_x - min_x;
        let content_height = max_y - min_y;

        // Calculate zoom to fit
        let zoom_x = canvas_width / content_width;
        let zoom_y = canvas_height / content_height;
        let zoom = zoom_x.min(zoom_y).min(1.0); // Don't zoom in past 100%

        self.viewport.zoom = zoom;
        self.viewport.pan_x = -min_x * zoom + (canvas_width - content_width * zoom) / 2.0;
        self.viewport.pan_y = -min_y * zoom + (canvas_height - content_height * zoom) / 2.0;
    }

    // ========== Utility Methods ==========

    /// Check if the graph is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }

    /// Clear the entire graph
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.groups.clear();
        self.clear_selection();
        self.dirty = true;
    }

    /// Mark the graph as saved (not dirty)
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Get the maximum z-index among all nodes
    pub fn max_node_z_index(&self) -> i32 {
        self.nodes.values().map(|n| n.z_index).max().unwrap_or(0)
    }

    /// Bring a node to the front
    pub fn bring_node_to_front(&mut self, id: NodeId) {
        let max_z = self.max_node_z_index();
        if let Some(node) = self.nodes.get_mut(&id) {
            node.bring_to_front(max_z);
            self.dirty = true;
        }
    }

    /// Send a node to the back
    pub fn send_node_to_back(&mut self, id: NodeId) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.send_to_back();
            self.dirty = true;
        }
    }

    /// Move selected nodes by a delta
    pub fn move_selected(&mut self, dx: f32, dy: f32) {
        for id in &self.selected_nodes {
            if let Some(node) = self.nodes.get_mut(id) {
                node.translate(dx, dy);
            }
        }
        self.dirty = true;
    }

    /// Align selected nodes to a specific alignment
    pub fn align_selected(&mut self, alignment: Alignment) {
        if self.selected_nodes.len() < 2 {
            return;
        }

        let positions: Vec<(NodeId, f32, f32, f32, f32)> = self.selected_nodes
            .iter()
            .filter_map(|id| {
                self.nodes.get(id).map(|n| {
                    let (x, y, w, h) = n.bounds();
                    (*id, x, y, w, h)
                })
            })
            .collect();

        if positions.is_empty() {
            return;
        }

        match alignment {
            Alignment::Left => {
                let min_x = positions.iter().map(|(_, x, _, _, _)| *x).fold(f32::MAX, f32::min);
                for (id, _, _, _, _) in &positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.position.x = min_x;
                    }
                }
            }
            Alignment::Right => {
                let max_x = positions.iter().map(|(_, x, _, w, _)| x + w).fold(f32::MIN, f32::max);
                for (id, _, _, w, _) in &positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.position.x = max_x - w;
                    }
                }
            }
            Alignment::Top => {
                let min_y = positions.iter().map(|(_, _, y, _, _)| *y).fold(f32::MAX, f32::min);
                for (id, _, _, _, _) in &positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.position.y = min_y;
                    }
                }
            }
            Alignment::Bottom => {
                let max_y = positions.iter().map(|(_, _, y, _, h)| y + h).fold(f32::MIN, f32::max);
                for (id, _, _, _, h) in &positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.position.y = max_y - h;
                    }
                }
            }
            Alignment::CenterH => {
                let center_x = positions.iter().map(|(_, x, _, w, _)| x + w / 2.0).sum::<f32>() / positions.len() as f32;
                for (id, _, _, w, _) in &positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.position.x = center_x - w / 2.0;
                    }
                }
            }
            Alignment::CenterV => {
                let center_y = positions.iter().map(|(_, _, y, _, h)| y + h / 2.0).sum::<f32>() / positions.len() as f32;
                for (id, _, _, _, h) in &positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.position.y = center_y - h / 2.0;
                    }
                }
            }
        }
        self.dirty = true;
    }
}

impl Default for ProjectGraph {
    fn default() -> Self {
        Self::new(ProjectMeta::default())
    }
}

/// Viewport state for the canvas
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    /// Horizontal pan offset
    pub pan_x: f32,
    /// Vertical pan offset
    pub pan_y: f32,
    /// Zoom level (1.0 = 100%)
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
        }
    }
}

/// Alignment options for nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Left,
    Right,
    Top,
    Bottom,
    CenterH,
    CenterV,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = ProjectGraph::with_name("test_project");
        assert_eq!(graph.meta.name, "test_project");
        assert!(graph.is_empty());
        assert!(!graph.dirty);
    }

    #[test]
    fn test_add_node() {
        let mut graph = ProjectGraph::with_name("test");
        let node = Node::new_entity("User");
        let id = graph.add_node(node);

        assert!(graph.has_node(id));
        assert_eq!(graph.node_count(), 1);
        assert!(graph.dirty);
    }

    #[test]
    fn test_connect_nodes() {
        let mut graph = ProjectGraph::with_name("test");

        let user = graph.add_node(Node::new_entity("User"));
        let post = graph.add_node(Node::new_entity("Post"));

        let edge_result = graph.add_relationship(user, post, RelationType::OneToMany);
        assert!(edge_result.is_ok());
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.are_connected(user, post));
    }

    #[test]
    fn test_remove_node_removes_edges() {
        let mut graph = ProjectGraph::with_name("test");

        let user = graph.add_node(Node::new_entity("User"));
        let post = graph.add_node(Node::new_entity("Post"));
        let _ = graph.add_relationship(user, post, RelationType::OneToMany);

        assert_eq!(graph.edge_count(), 1);

        graph.remove_node(user);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_selection() {
        let mut graph = ProjectGraph::with_name("test");

        let node1 = graph.add_node(Node::new_entity("User"));
        let node2 = graph.add_node(Node::new_entity("Post"));

        graph.select_node(node1);
        assert!(graph.has_selection());
        assert_eq!(graph.selection_count(), 1);

        graph.select_node(node2);
        assert_eq!(graph.selection_count(), 2);

        graph.clear_selection();
        assert!(!graph.has_selection());
    }

    #[test]
    fn test_groups() {
        let mut graph = ProjectGraph::with_name("test");

        let node1 = graph.add_node(Node::new_entity("User"));
        let node2 = graph.add_node(Node::new_entity("Post"));

        graph.select_node(node1);
        graph.select_node(node2);

        let group_id = graph.group_selected_nodes("Entities");
        assert!(group_id.is_some());

        let group_id = group_id.unwrap();
        let group = graph.get_group(group_id).unwrap();
        assert_eq!(group.node_count(), 2);
        assert!(group.contains_node(&node1));
        assert!(group.contains_node(&node2));
    }

    #[test]
    fn test_duplicate_node() {
        let mut graph = ProjectGraph::with_name("test");
        let original = graph.add_node(Node::new_entity("User"));

        let duplicate = graph.duplicate_node(original);
        assert!(duplicate.is_some());
        assert_eq!(graph.node_count(), 2);

        let dup_id = duplicate.unwrap();
        assert_ne!(original, dup_id);
    }

    #[test]
    fn test_viewport() {
        let mut graph = ProjectGraph::with_name("test");

        graph.pan(100.0, 50.0);
        assert_eq!(graph.viewport.pan_x, 100.0);
        assert_eq!(graph.viewport.pan_y, 50.0);

        graph.set_zoom(2.0);
        assert_eq!(graph.viewport.zoom, 2.0);

        graph.zoom(0.5);
        assert_eq!(graph.viewport.zoom, 1.0);

        graph.reset_viewport();
        assert_eq!(graph.viewport.pan_x, 0.0);
        assert_eq!(graph.viewport.zoom, 1.0);
    }

    #[test]
    fn test_connected_nodes() {
        let mut graph = ProjectGraph::with_name("test");

        let user = graph.add_node(Node::new_entity("User"));
        let post = graph.add_node(Node::new_entity("Post"));
        let comment = graph.add_node(Node::new_entity("Comment"));

        let _ = graph.add_relationship(user, post, RelationType::OneToMany);
        let _ = graph.add_relationship(post, comment, RelationType::OneToMany);

        let user_connected = graph.connected_nodes(user);
        assert!(user_connected.contains(&post));
        assert!(!user_connected.contains(&comment));

        let post_connected = graph.connected_nodes(post);
        assert!(post_connected.contains(&user));
        assert!(post_connected.contains(&comment));
    }

    #[test]
    fn test_upstream_downstream() {
        let mut graph = ProjectGraph::with_name("test");

        let user = graph.add_node(Node::new_entity("User"));
        let post = graph.add_node(Node::new_entity("Post"));
        let _ = graph.add_relationship(user, post, RelationType::OneToMany);

        let upstream = graph.upstream_nodes(post);
        assert!(upstream.contains(&user));

        let downstream = graph.downstream_nodes(user);
        assert!(downstream.contains(&post));
    }
}
