//! Editor State Management
//!
//! This module provides state management for the Immortal Engine visual editor.
//! It tracks the current project, selection, interaction state, and undo/redo history.

use std::collections::HashSet;
use uuid::Uuid;

use imortal_ir::{ProjectGraph, NodeId, EdgeId};
use imortal_components::ComponentRegistry;

/// The main editor state
#[derive(Debug)]
pub struct EditorState {
    /// The current project graph
    pub project: ProjectGraph,

    /// Component registry for available components
    pub registry: ComponentRegistry,

    /// Current selection state
    pub selection: SelectionState,

    /// Current interaction state (dragging, connecting, etc.)
    pub interaction: InteractionState,

    /// View/viewport state
    pub view: ViewState,

    /// Undo/Redo history
    pub history: History,

    /// Current tool being used
    pub active_tool: Tool,

    /// Path to the current project file (if saved)
    pub project_path: Option<String>,

    /// Whether the project has unsaved changes
    pub is_dirty: bool,

    /// Clipboard contents
    pub clipboard: Option<ClipboardContent>,

    /// UI panel visibility
    pub panels: PanelVisibility,

    /// Search text for the component palette
    pub palette_search: String,

    /// Expanded categories in the palette
    pub palette_expanded: std::collections::HashSet<imortal_core::ComponentCategory>,
}

impl EditorState {
    /// Create a new editor state with an empty project
    pub fn new() -> Self {
        let mut palette_expanded = std::collections::HashSet::new();
        for category in imortal_core::ComponentCategory::all() {
            palette_expanded.insert(*category);
        }

        Self {
            project: ProjectGraph::default(),
            registry: ComponentRegistry::with_builtins(),
            selection: SelectionState::default(),
            interaction: InteractionState::None,
            view: ViewState::default(),
            history: History::new(),
            active_tool: Tool::Select,
            project_path: None,
            is_dirty: false,
            clipboard: None,
            panels: PanelVisibility::default(),
            palette_search: String::new(),
            palette_expanded,
        }
    }

    /// Create a new editor state with an existing project
    pub fn with_project(project: ProjectGraph) -> Self {
        let mut palette_expanded = std::collections::HashSet::new();
        for category in imortal_core::ComponentCategory::all() {
            palette_expanded.insert(*category);
        }

        Self {
            project,
            registry: ComponentRegistry::with_builtins(),
            selection: SelectionState::default(),
            interaction: InteractionState::None,
            view: ViewState::default(),
            history: History::new(),
            active_tool: Tool::Select,
            project_path: None,
            is_dirty: false,
            clipboard: None,
            panels: PanelVisibility::default(),
            palette_search: String::new(),
            palette_expanded,
        }
    }

    /// Mark the project as modified
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    /// Mark the project as saved
    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
        self.project.mark_saved();
    }

    /// Clear the current selection
    pub fn clear_selection(&mut self) {
        self.selection.clear();
        self.project.clear_selection();
    }

    /// Select a single node
    pub fn select_node(&mut self, id: NodeId, add_to_selection: bool) {
        if !add_to_selection {
            self.clear_selection();
        }
        self.selection.nodes.insert(id);
        self.project.select_node(id);
    }

    /// Select a single edge
    pub fn select_edge(&mut self, id: EdgeId, add_to_selection: bool) {
        if !add_to_selection {
            self.clear_selection();
        }
        self.selection.edges.insert(id);
        self.project.select_edge(id);
    }

    /// Delete selected items
    pub fn delete_selected(&mut self) {
        // Save state for undo
        self.save_undo_state("Delete");

        self.project.delete_selected();
        self.selection.clear();
        self.mark_dirty();
    }

    /// Save current state for undo
    pub fn save_undo_state(&mut self, action_name: &str) {
        let snapshot = self.project.clone();
        self.history.push(action_name, snapshot);
    }

    /// Undo the last action
    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.history.undo(self.project.clone()) {
            self.project = snapshot;
            self.selection.clear();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self) -> bool {
        if let Some(snapshot) = self.history.redo(self.project.clone()) {
            self.project = snapshot;
            self.selection.clear();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    /// Copy selected items to clipboard
    pub fn copy_selection(&mut self) {
        let nodes: Vec<_> = self.selection.nodes.iter()
            .filter_map(|id| self.project.get_node(*id).cloned())
            .collect();

        let edges: Vec<_> = self.selection.edges.iter()
            .filter_map(|id| self.project.get_edge(*id).cloned())
            .collect();

        if !nodes.is_empty() || !edges.is_empty() {
            self.clipboard = Some(ClipboardContent { nodes, edges });
        }
    }

    /// Paste items from clipboard
    pub fn paste(&mut self) {
        // Clone clipboard content to avoid borrow issues
        let clipboard = match &self.clipboard {
            Some(c) => c.clone(),
            None => return,
        };

        self.save_undo_state("Paste");

        // Clear current selection
        self.clear_selection();

        // Clone and add nodes with new IDs
        let mut id_mapping = std::collections::HashMap::new();

        for node in &clipboard.nodes {
                let mut new_node = node.clone();
                let old_id = new_node.id;
                new_node.id = Uuid::new_v4();
                new_node.position.x += 20.0;
                new_node.position.y += 20.0;

                id_mapping.insert(old_id, new_node.id);

                let new_id = self.project.add_node(new_node);
                self.selection.nodes.insert(new_id);
                self.project.select_node(new_id);
            }

        // Clone and add edges between pasted nodes
        for edge in &clipboard.edges {
            if let (Some(&new_from), Some(&new_to)) = (
                id_mapping.get(&edge.from_node),
                id_mapping.get(&edge.to_node),
            ) {
                let mut new_edge = edge.clone();
                new_edge.id = Uuid::new_v4();
                new_edge.from_node = new_from;
                new_edge.to_node = new_to;
                let _ = self.project.add_edge(new_edge);
            }
        }

        self.mark_dirty();
    }

    /// Duplicate selected items
    pub fn duplicate_selection(&mut self) {
        self.copy_selection();
        self.paste();
    }

    /// Check if there's anything to undo
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Check if there's anything to redo
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Get the undo action name
    pub fn undo_action_name(&self) -> Option<&str> {
        self.history.undo_action_name()
    }

    /// Get the redo action name
    pub fn redo_action_name(&self) -> Option<&str> {
        self.history.redo_action_name()
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

/// Current selection state
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Selected node IDs
    pub nodes: HashSet<NodeId>,
    /// Selected edge IDs
    pub edges: HashSet<EdgeId>,
    /// Selected group IDs
    pub groups: HashSet<Uuid>,
}

impl SelectionState {
    /// Clear all selections
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.groups.clear();
    }

    /// Check if anything is selected
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty() && self.groups.is_empty()
    }

    /// Get total selection count
    pub fn count(&self) -> usize {
        self.nodes.len() + self.edges.len() + self.groups.len()
    }

    /// Check if a specific node is selected
    pub fn has_node(&self, id: NodeId) -> bool {
        self.nodes.contains(&id)
    }

    /// Check if a specific edge is selected
    pub fn has_edge(&self, id: EdgeId) -> bool {
        self.edges.contains(&id)
    }
}

/// Current interaction state
#[derive(Debug, Clone)]
pub enum InteractionState {
    /// No active interaction
    None,
    /// Dragging selected nodes
    DraggingNodes {
        start_pos: (f32, f32),
        current_pos: (f32, f32),
    },
    /// Drawing a connection from a port
    DrawingConnection {
        from_node: NodeId,
        from_port: String,
        current_pos: (f32, f32),
    },
    /// Selection box being drawn
    SelectionBox {
        start_pos: (f32, f32),
        current_pos: (f32, f32),
    },
    /// Panning the canvas
    Panning {
        start_pan: (f32, f32),
        start_mouse: (f32, f32),
    },
    /// Resizing a node
    ResizingNode {
        node_id: NodeId,
        start_size: (f32, f32),
        handle: ResizeHandle,
    },
    /// Hovering over a port
    HoveringPort {
        node_id: NodeId,
        port_id: String,
    },
}

/// Resize handle position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

/// View/viewport state
#[derive(Debug, Clone)]
pub struct ViewState {
    /// Pan offset X
    pub pan_x: f32,
    /// Pan offset Y
    pub pan_y: f32,
    /// Zoom level (1.0 = 100%)
    pub zoom: f32,
    /// Show grid
    pub show_grid: bool,
    /// Grid size in pixels
    pub grid_size: f32,
    /// Snap to grid
    pub snap_to_grid: bool,
    /// Show minimap
    pub show_minimap: bool,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
            show_grid: true,
            grid_size: 20.0,
            snap_to_grid: true,
            show_minimap: true,
        }
    }
}

impl ViewState {
    /// Convert screen coordinates to canvas coordinates
    pub fn screen_to_canvas(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let canvas_x = (screen_x - self.pan_x) / self.zoom;
        let canvas_y = (screen_y - self.pan_y) / self.zoom;
        (canvas_x, canvas_y)
    }

    /// Convert canvas coordinates to screen coordinates
    pub fn canvas_to_screen(&self, canvas_x: f32, canvas_y: f32) -> (f32, f32) {
        let screen_x = canvas_x * self.zoom + self.pan_x;
        let screen_y = canvas_y * self.zoom + self.pan_y;
        (screen_x, screen_y)
    }

    /// Snap a position to the grid
    pub fn snap_to_grid_pos(&self, x: f32, y: f32) -> (f32, f32) {
        if self.snap_to_grid {
            let snapped_x = (x / self.grid_size).round() * self.grid_size;
            let snapped_y = (y / self.grid_size).round() * self.grid_size;
            (snapped_x, snapped_y)
        } else {
            (x, y)
        }
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.2).min(5.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.2).max(0.1);
    }

    /// Reset zoom to 100%
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    /// Reset view to origin
    pub fn reset_view(&mut self) {
        self.pan_x = 0.0;
        self.pan_y = 0.0;
        self.zoom = 1.0;
    }
}

/// Currently active tool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tool {
    /// Selection/move tool
    #[default]
    Select,
    /// Pan tool
    Pan,
    /// Connection drawing tool
    Connect,
    /// Add comment/annotation
    Comment,
}

/// Undo/Redo history
#[derive(Debug)]
pub struct History {
    /// Undo stack
    undo_stack: Vec<HistoryEntry>,
    /// Redo stack
    redo_stack: Vec<HistoryEntry>,
    /// Maximum history size
    max_size: usize,
}

#[derive(Debug, Clone)]
struct HistoryEntry {
    /// Action name for display
    action_name: String,
    /// Snapshot of the project state
    snapshot: ProjectGraph,
}

impl History {
    /// Create a new history with default size
    pub fn new() -> Self {
        Self::with_max_size(50)
    }

    /// Create a new history with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Push a new state onto the history
    pub fn push(&mut self, action_name: &str, snapshot: ProjectGraph) {
        // Clear redo stack when new action is performed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(HistoryEntry {
            action_name: action_name.to_string(),
            snapshot,
        });

        // Trim if exceeds max size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last action, returns the previous state
    pub fn undo(&mut self, current_state: ProjectGraph) -> Option<ProjectGraph> {
        if let Some(entry) = self.undo_stack.pop() {
            // Push current state to redo stack
            self.redo_stack.push(HistoryEntry {
                action_name: entry.action_name.clone(),
                snapshot: current_state,
            });

            Some(entry.snapshot)
        } else {
            None
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self, current_state: ProjectGraph) -> Option<ProjectGraph> {
        if let Some(entry) = self.redo_stack.pop() {
            // Push current state to undo stack
            self.undo_stack.push(HistoryEntry {
                action_name: entry.action_name.clone(),
                snapshot: current_state,
            });

            Some(entry.snapshot)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the name of the action that would be undone
    pub fn undo_action_name(&self) -> Option<&str> {
        self.undo_stack.last().map(|e| e.action_name.as_str())
    }

    /// Get the name of the action that would be redone
    pub fn redo_action_name(&self) -> Option<&str> {
        self.redo_stack.last().map(|e| e.action_name.as_str())
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

/// Clipboard content for copy/paste
#[derive(Debug, Clone)]
pub struct ClipboardContent {
    /// Copied nodes
    pub nodes: Vec<imortal_ir::Node>,
    /// Copied edges
    pub edges: Vec<imortal_ir::Edge>,
}

/// Panel visibility settings
#[derive(Debug, Clone)]
pub struct PanelVisibility {
    /// Show component palette
    pub palette: bool,
    /// Show properties panel
    pub properties: bool,
    /// Show code preview panel
    pub code_preview: bool,
    /// Show validation panel
    pub validation: bool,
    /// Show minimap
    pub minimap: bool,
}

impl Default for PanelVisibility {
    fn default() -> Self {
        Self {
            palette: true,
            properties: true,
            code_preview: false,
            validation: true,
            minimap: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_creation() {
        let state = EditorState::new();
        assert!(!state.is_dirty);
        assert!(state.selection.is_empty());
    }

    #[test]
    fn test_selection_state() {
        let mut selection = SelectionState::default();
        assert!(selection.is_empty());

        let node_id = Uuid::new_v4();
        selection.nodes.insert(node_id);
        assert!(!selection.is_empty());
        assert_eq!(selection.count(), 1);
        assert!(selection.has_node(node_id));
    }

    #[test]
    fn test_view_state_coordinates() {
        let view = ViewState {
            pan_x: 100.0,
            pan_y: 50.0,
            zoom: 2.0,
            ..Default::default()
        };

        let (canvas_x, canvas_y) = view.screen_to_canvas(200.0, 150.0);
        assert_eq!(canvas_x, 50.0);
        assert_eq!(canvas_y, 50.0);

        let (screen_x, screen_y) = view.canvas_to_screen(50.0, 50.0);
        assert_eq!(screen_x, 200.0);
        assert_eq!(screen_y, 150.0);
    }

    #[test]
    fn test_history_undo_redo() {
        let mut history = History::new();

        let initial = ProjectGraph::with_name("initial");
        history.push("Create", initial.clone());

        assert!(history.can_undo());
        assert!(!history.can_redo());

        let current = ProjectGraph::with_name("current");
        let undone = history.undo(current.clone());

        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_snap_to_grid() {
        let view = ViewState {
            grid_size: 20.0,
            snap_to_grid: true,
            ..Default::default()
        };

        let (x, y) = view.snap_to_grid_pos(25.0, 32.0);
        assert_eq!(x, 20.0);
        assert_eq!(y, 40.0);
    }
}
