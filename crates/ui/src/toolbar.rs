//! Toolbar UI Component
//!
//! This module provides the toolbar widget for the Immortal Engine visual editor.
//! The toolbar contains quick access buttons for common actions like save, undo/redo,
//! zoom controls, tool selection, and more.

use eframe::egui;

use crate::state::{EditorState, Tool};

/// Toolbar widget for the editor
pub struct Toolbar {
    /// Whether to show tooltips
    pub show_tooltips: bool,
    /// Icon size
    pub icon_size: f32,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            show_tooltips: true,
            icon_size: 20.0,
        }
    }
}

impl Toolbar {
    /// Create a new toolbar
    pub fn new() -> Self {
        Self::default()
    }

    /// Set icon size
    pub fn with_icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Render the toolbar
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut EditorState) -> ToolbarResponse {
        let mut response = ToolbarResponse::default();

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // File operations
            self.file_section(ui, &mut response);

            ui.separator();

            // Edit operations
            self.edit_section(ui, state, &mut response);

            ui.separator();

            // Tool selection
            self.tool_section(ui, state, &mut response);

            ui.separator();

            // View controls
            self.view_section(ui, state, &mut response);

            ui.separator();

            // Generate/Run
            self.generate_section(ui, &mut response);
        });

        response
    }

    /// File operations section
    fn file_section(&self, ui: &mut egui::Ui, response: &mut ToolbarResponse) {
        if self.toolbar_button(ui, "📄", "New Project").clicked() {
            response.new_project = true;
        }
        if self.toolbar_button(ui, "📂", "Open Project").clicked() {
            response.open_project = true;
        }
        if self.toolbar_button(ui, "💾", "Save Project").clicked() {
            response.save_project = true;
        }
    }

    /// Edit operations section
    fn edit_section(&self, ui: &mut egui::Ui, state: &EditorState, response: &mut ToolbarResponse) {
        let can_undo = state.can_undo();
        let can_redo = state.can_redo();

        ui.add_enabled_ui(can_undo, |ui| {
            let undo_tip = state.undo_action_name()
                .map(|name| format!("Undo {}", name))
                .unwrap_or_else(|| "Undo".to_string());

            if self.toolbar_button(ui, "↩", &undo_tip).clicked() {
                response.undo = true;
            }
        });

        ui.add_enabled_ui(can_redo, |ui| {
            let redo_tip = state.redo_action_name()
                .map(|name| format!("Redo {}", name))
                .unwrap_or_else(|| "Redo".to_string());

            if self.toolbar_button(ui, "↪", &redo_tip).clicked() {
                response.redo = true;
            }
        });

        ui.add_space(4.0);

        if self.toolbar_button(ui, "✂", "Cut").clicked() {
            response.cut = true;
        }
        if self.toolbar_button(ui, "📋", "Copy").clicked() {
            response.copy = true;
        }
        if self.toolbar_button(ui, "📌", "Paste").clicked() {
            response.paste = true;
        }
    }

    /// Tool selection section
    fn tool_section(&self, ui: &mut egui::Ui, state: &mut EditorState, response: &mut ToolbarResponse) {
        let tools = [
            (Tool::Select, "🖱", "Select Tool (V)"),
            (Tool::Pan, "✋", "Pan Tool (H)"),
            (Tool::Connect, "🔗", "Connect Tool (C)"),
            (Tool::Comment, "💬", "Comment Tool (N)"),
        ];

        for (tool, icon, tooltip) in tools {
            let is_selected = state.active_tool == tool;

            let button = egui::Button::new(icon)
                .min_size(egui::vec2(self.icon_size + 8.0, self.icon_size + 8.0))
                .selected(is_selected);

            let btn_response = ui.add(button);

            if self.show_tooltips {
                btn_response.clone().on_hover_text(tooltip);
            }

            if btn_response.clicked() {
                state.active_tool = tool;
                response.tool_changed = Some(tool);
            }
        }
    }

    /// View controls section
    fn view_section(&self, ui: &mut egui::Ui, state: &mut EditorState, response: &mut ToolbarResponse) {
        if self.toolbar_button(ui, "🔍-", "Zoom Out").clicked() {
            state.view.zoom_out();
            response.zoom_changed = true;
        }

        // Zoom level display
        ui.label(format!("{:.0}%", state.view.zoom * 100.0));

        if self.toolbar_button(ui, "🔍+", "Zoom In").clicked() {
            state.view.zoom_in();
            response.zoom_changed = true;
        }

        if self.toolbar_button(ui, "⊡", "Fit to Content").clicked() {
            response.fit_to_content = true;
        }

        ui.add_space(4.0);

        // Grid toggle
        let grid_icon = if state.view.show_grid { "▦" } else { "▢" };
        if self.toolbar_button(ui, grid_icon, "Toggle Grid").clicked() {
            state.view.show_grid = !state.view.show_grid;
        }

        // Snap toggle
        let snap_icon = if state.view.snap_to_grid { "🧲" } else { "🚫" };
        if self.toolbar_button(ui, snap_icon, "Toggle Snap to Grid").clicked() {
            state.view.snap_to_grid = !state.view.snap_to_grid;
        }
    }

    /// Generate/Run section
    fn generate_section(&self, ui: &mut egui::Ui, response: &mut ToolbarResponse) {
        if self.toolbar_button(ui, "✓", "Validate Project").clicked() {
            response.validate = true;
        }
        if self.toolbar_button(ui, "⚡", "Generate Code").clicked() {
            response.generate = true;
        }
        if self.toolbar_button(ui, "▶", "Preview").clicked() {
            response.preview = true;
        }
    }

    /// Create a toolbar button with consistent styling
    fn toolbar_button(&self, ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
        let button = egui::Button::new(icon)
            .min_size(egui::vec2(self.icon_size + 8.0, self.icon_size + 8.0));

        let response = ui.add(button);

        if self.show_tooltips {
            response.clone().on_hover_text(tooltip);
        }

        response
    }
}

/// Response from toolbar interactions
#[derive(Debug, Clone, Default)]
pub struct ToolbarResponse {
    // File operations
    pub new_project: bool,
    pub open_project: bool,
    pub save_project: bool,

    // Edit operations
    pub undo: bool,
    pub redo: bool,
    pub cut: bool,
    pub copy: bool,
    pub paste: bool,

    // Tool selection
    pub tool_changed: Option<Tool>,

    // View controls
    pub zoom_changed: bool,
    pub fit_to_content: bool,

    // Generate/Run
    pub validate: bool,
    pub generate: bool,
    pub preview: bool,
}

impl ToolbarResponse {
    /// Check if any action was triggered
    pub fn has_action(&self) -> bool {
        self.new_project
            || self.open_project
            || self.save_project
            || self.undo
            || self.redo
            || self.cut
            || self.copy
            || self.paste
            || self.tool_changed.is_some()
            || self.zoom_changed
            || self.fit_to_content
            || self.validate
            || self.generate
            || self.preview
    }
}

/// Toolbar separator with optional label
pub fn labeled_separator(ui: &mut egui::Ui, label: &str) {
    ui.separator();
    if !label.is_empty() {
        ui.label(egui::RichText::new(label).small().weak());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_response_default() {
        let response = ToolbarResponse::default();
        assert!(!response.has_action());
    }

    #[test]
    fn test_toolbar_response_with_action() {
        let mut response = ToolbarResponse::default();
        response.save_project = true;
        assert!(response.has_action());
    }

    #[test]
    fn test_toolbar_creation() {
        let toolbar = Toolbar::new().with_icon_size(24.0);
        assert_eq!(toolbar.icon_size, 24.0);
        assert!(toolbar.show_tooltips);
    }
}
