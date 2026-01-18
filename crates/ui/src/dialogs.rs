//! Dialog components for Immortal Engine UI
//!
//! This module provides various dialog windows used throughout the application:
//! - New Project dialog
//! - Open Project dialog
//! - Save Project dialog
//! - Settings dialog
//! - About dialog
//! - Confirmation dialogs

use eframe::egui;

/// Result of a dialog interaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogResult {
    /// Dialog is still open, no result yet
    Open,
    /// User confirmed/accepted
    Confirmed,
    /// User cancelled
    Cancelled,
}

/// New Project dialog
pub struct NewProjectDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Project name input
    pub name: String,
    /// Project description
    pub description: String,
    /// Selected template
    pub template: String,
    /// Available templates
    pub templates: Vec<String>,
}

impl Default for NewProjectDialog {
    fn default() -> Self {
        Self {
            visible: false,
            name: "MyProject".to_string(),
            description: String::new(),
            template: "default".to_string(),
            templates: vec![
                "default".to_string(),
                "web-api".to_string(),
                "web-app".to_string(),
                "embedded".to_string(),
            ],
        }
    }
}

impl NewProjectDialog {
    /// Create a new dialog
    pub fn new() -> Self {
        Self::default()
    }

    /// Show the dialog
    pub fn open(&mut self) {
        self.visible = true;
        self.name = "MyProject".to_string();
        self.description = String::new();
        self.template = "default".to_string();
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Render the dialog and return the result
    pub fn show(&mut self, ctx: &egui::Context) -> DialogResult {
        if !self.visible {
            return DialogResult::Open;
        }

        let mut result = DialogResult::Open;

        egui::Window::new("New Project")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Project Name:");
                        ui.text_edit_singleline(&mut self.name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Description:");
                        ui.text_edit_singleline(&mut self.description);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Template:");
                        egui::ComboBox::from_id_salt("template_select")
                            .selected_text(&self.template)
                            .show_ui(ui, |ui| {
                                for template in &self.templates {
                                    ui.selectable_value(&mut self.template, template.clone(), template);
                                }
                            });
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() && !self.name.is_empty() {
                            result = DialogResult::Confirmed;
                            self.visible = false;
                        }
                        if ui.button("Cancel").clicked() {
                            result = DialogResult::Cancelled;
                            self.visible = false;
                        }
                    });
                });
            });

        result
    }
}

/// Settings dialog
pub struct SettingsDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Dark mode setting
    pub dark_mode: bool,
    /// Show grid setting
    pub show_grid: bool,
    /// Snap to grid setting
    pub snap_to_grid: bool,
    /// Grid size
    pub grid_size: f32,
    /// Show minimap
    pub show_minimap: bool,
    /// Auto-save interval (seconds, 0 = disabled)
    pub auto_save_interval: u32,
    /// Show component descriptions in palette
    pub show_descriptions: bool,
}

impl Default for SettingsDialog {
    fn default() -> Self {
        Self {
            visible: false,
            dark_mode: true,
            show_grid: true,
            snap_to_grid: true,
            grid_size: 20.0,
            show_minimap: true,
            auto_save_interval: 60,
            show_descriptions: true,
        }
    }
}

impl SettingsDialog {
    /// Create a new settings dialog
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog
    pub fn open(&mut self) {
        self.visible = true;
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Render the dialog and return the result
    pub fn show(&mut self, ctx: &egui::Context) -> DialogResult {
        if !self.visible {
            return DialogResult::Open;
        }

        let mut result = DialogResult::Open;

        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("Appearance");
                ui.checkbox(&mut self.dark_mode, "Dark Mode");
                ui.add_space(10.0);

                ui.heading("Canvas");
                ui.checkbox(&mut self.show_grid, "Show Grid");
                ui.checkbox(&mut self.snap_to_grid, "Snap to Grid");
                ui.horizontal(|ui| {
                    ui.label("Grid Size:");
                    ui.add(egui::Slider::new(&mut self.grid_size, 10.0..=50.0));
                });
                ui.checkbox(&mut self.show_minimap, "Show Minimap");
                ui.add_space(10.0);

                ui.heading("Editor");
                ui.checkbox(&mut self.show_descriptions, "Show Component Descriptions");
                ui.horizontal(|ui| {
                    ui.label("Auto-save interval (seconds, 0 = disabled):");
                    ui.add(egui::DragValue::new(&mut self.auto_save_interval).range(0..=600));
                });
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        result = DialogResult::Confirmed;
                        self.visible = false;
                    }
                    if ui.button("Cancel").clicked() {
                        result = DialogResult::Cancelled;
                        self.visible = false;
                    }
                });
            });

        result
    }
}

/// About dialog
pub struct AboutDialog {
    /// Whether the dialog is visible
    pub visible: bool,
}

impl Default for AboutDialog {
    fn default() -> Self {
        Self { visible: false }
    }
}

impl AboutDialog {
    /// Create a new about dialog
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog
    pub fn open(&mut self) {
        self.visible = true;
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Render the dialog
    pub fn show(&mut self, ctx: &egui::Context) -> DialogResult {
        if !self.visible {
            return DialogResult::Open;
        }

        let mut result = DialogResult::Open;

        egui::Window::new("About Immortal Engine")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🔧 Immortal Engine");
                    ui.add_space(10.0);
                    ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(10.0);
                    ui.label("Visual Prototyping System");
                    ui.label("Build applications by dragging components");
                    ui.label("and drawing connections between them.");
                    ui.add_space(20.0);
                    ui.label("Licensed under MIT");
                    ui.add_space(10.0);
                    ui.hyperlink_to("GitHub Repository", "https://github.com/yourusername/imortal_engine");
                    ui.add_space(20.0);

                    if ui.button("Close").clicked() {
                        result = DialogResult::Confirmed;
                        self.visible = false;
                    }
                });
            });

        result
    }
}

/// Confirmation dialog for destructive actions
pub struct ConfirmDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Dialog title
    pub title: String,
    /// Dialog message
    pub message: String,
    /// Confirm button text
    pub confirm_text: String,
    /// Cancel button text
    pub cancel_text: String,
}

impl Default for ConfirmDialog {
    fn default() -> Self {
        Self {
            visible: false,
            title: "Confirm".to_string(),
            message: "Are you sure?".to_string(),
            confirm_text: "Yes".to_string(),
            cancel_text: "No".to_string(),
        }
    }
}

impl ConfirmDialog {
    /// Create a new confirmation dialog
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog with a custom message
    pub fn open(&mut self, title: &str, message: &str) {
        self.visible = true;
        self.title = title.to_string();
        self.message = message.to_string();
        self.confirm_text = "Yes".to_string();
        self.cancel_text = "No".to_string();
    }

    /// Open with custom button text
    pub fn open_with_buttons(&mut self, title: &str, message: &str, confirm: &str, cancel: &str) {
        self.visible = true;
        self.title = title.to_string();
        self.message = message.to_string();
        self.confirm_text = confirm.to_string();
        self.cancel_text = cancel.to_string();
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Render the dialog
    pub fn show(&mut self, ctx: &egui::Context) -> DialogResult {
        if !self.visible {
            return DialogResult::Open;
        }

        let mut result = DialogResult::Open;

        egui::Window::new(&self.title)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(&self.message);
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button(&self.confirm_text).clicked() {
                            result = DialogResult::Confirmed;
                            self.visible = false;
                        }
                        if ui.button(&self.cancel_text).clicked() {
                            result = DialogResult::Cancelled;
                            self.visible = false;
                        }
                    });
                });
            });

        result
    }
}

/// Export dialog
pub struct ExportDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Output path
    pub output_path: String,
    /// Export format
    pub format: ExportFormat,
    /// Include tests
    pub include_tests: bool,
    /// Include documentation
    pub include_docs: bool,
}

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    #[default]
    RustProject,
    JsonSchema,
    OpenApi,
}

impl Default for ExportDialog {
    fn default() -> Self {
        Self {
            visible: false,
            output_path: "./generated".to_string(),
            format: ExportFormat::RustProject,
            include_tests: true,
            include_docs: true,
        }
    }
}

impl ExportDialog {
    /// Create a new export dialog
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog
    pub fn open(&mut self) {
        self.visible = true;
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Render the dialog
    pub fn show(&mut self, ctx: &egui::Context) -> DialogResult {
        if !self.visible {
            return DialogResult::Open;
        }

        let mut result = DialogResult::Open;

        egui::Window::new("Export Project")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Output Path:");
                    ui.text_edit_singleline(&mut self.output_path);
                    if ui.button("Browse...").clicked() {
                        // TODO: File dialog
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Format:");
                    egui::ComboBox::from_id_salt("export_format")
                        .selected_text(match self.format {
                            ExportFormat::RustProject => "Rust Project",
                            ExportFormat::JsonSchema => "JSON Schema",
                            ExportFormat::OpenApi => "OpenAPI Spec",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.format, ExportFormat::RustProject, "Rust Project");
                            ui.selectable_value(&mut self.format, ExportFormat::JsonSchema, "JSON Schema");
                            ui.selectable_value(&mut self.format, ExportFormat::OpenApi, "OpenAPI Spec");
                        });
                });

                ui.checkbox(&mut self.include_tests, "Generate Tests");
                ui.checkbox(&mut self.include_docs, "Generate Documentation");

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Export").clicked() {
                        result = DialogResult::Confirmed;
                        self.visible = false;
                    }
                    if ui.button("Cancel").clicked() {
                        result = DialogResult::Cancelled;
                        self.visible = false;
                    }
                });
            });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_project_dialog() {
        let mut dialog = NewProjectDialog::new();
        assert!(!dialog.visible);

        dialog.open();
        assert!(dialog.visible);
        assert!(!dialog.name.is_empty());

        dialog.close();
        assert!(!dialog.visible);
    }

    #[test]
    fn test_confirm_dialog() {
        let mut dialog = ConfirmDialog::new();
        dialog.open("Delete", "Are you sure you want to delete?");

        assert!(dialog.visible);
        assert_eq!(dialog.title, "Delete");
    }

    #[test]
    fn test_settings_dialog_defaults() {
        let dialog = SettingsDialog::new();
        assert!(dialog.dark_mode);
        assert!(dialog.show_grid);
        assert!(dialog.snap_to_grid);
    }
}
