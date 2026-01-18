//! Properties Panel for Immortal Engine UI
//!
//! This module provides the properties panel that displays and allows editing
//! of the selected component's fields, configuration, and ports.

use eframe::egui;
use imortal_ir::{Node, Edge, Field, Port, ProjectGraph};
use imortal_core::ConfigValue;

/// Properties panel widget
pub struct PropertiesPanel {
    /// Search filter for properties
    pub search_filter: String,
    /// Whether to show advanced properties
    pub show_advanced: bool,
    /// Collapsed sections
    collapsed_sections: std::collections::HashSet<String>,
}

impl PropertiesPanel {
    /// Create a new properties panel
    pub fn new() -> Self {
        Self {
            search_filter: String::new(),
            show_advanced: false,
            collapsed_sections: std::collections::HashSet::new(),
        }
    }

    /// Check if a name matches the current filter
    fn filter_matches(&self, name: &str) -> bool {
        if self.search_filter.is_empty() {
            return true;
        }
        name.to_lowercase().contains(&self.search_filter.to_lowercase())
    }

    /// Render the properties panel
    pub fn show(&mut self, ui: &mut egui::Ui, graph: &mut ProjectGraph) {
        ui.heading("Properties");
        ui.separator();

        // Search box
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_filter);
        });
        ui.checkbox(&mut self.show_advanced, "Show Advanced");
        ui.separator();

        // Get selection info
        let selected_nodes: Vec<_> = graph.selected_nodes.iter().copied().collect();
        let selected_edges: Vec<_> = graph.selected_edges.iter().copied().collect();

        if selected_nodes.is_empty() && selected_edges.is_empty() {
            ui.label("Select a component to view its properties");
            return;
        }

        if selected_nodes.len() == 1 {
            let node_id = selected_nodes[0];
            if let Some(node) = graph.get_node(node_id).cloned() {
                self.render_node_properties(ui, &node, graph);
            }
        } else if selected_edges.len() == 1 {
            let edge_id = selected_edges[0];
            if let Some(edge) = graph.get_edge(edge_id).cloned() {
                self.render_edge_properties(ui, &edge, graph);
            }
        } else {
            ui.label(format!(
                "{} nodes and {} edges selected",
                selected_nodes.len(),
                selected_edges.len()
            ));
            self.render_multi_selection_properties(ui, &selected_nodes, &selected_edges, graph);
        }
    }

    /// Render a section header and return whether it's expanded
    fn render_section_header(&mut self, ui: &mut egui::Ui, title: &str) -> bool {
        let is_collapsed = self.collapsed_sections.contains(title);
        let icon = if is_collapsed { "▶" } else { "▼" };

        if ui.selectable_label(false, format!("{} {}", icon, title)).clicked() {
            if is_collapsed {
                self.collapsed_sections.remove(title);
            } else {
                self.collapsed_sections.insert(title.to_string());
            }
        }

        !is_collapsed
    }

    /// Render properties for a single node
    fn render_node_properties(&mut self, ui: &mut egui::Ui, node: &Node, graph: &mut ProjectGraph) {
        // Basic info section
        if self.render_section_header(ui, "Basic Info") {
            ui.indent("basic_info", |ui| {
                // Node name
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    let mut name = node.name.clone();
                    if ui.text_edit_singleline(&mut name).changed() {
                        if let Some(n) = graph.get_node_mut(node.id) {
                            n.name = name;
                        }
                    }
                });

                // Component type (read-only)
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.label(&node.component_type);
                });

                // Category
                ui.horizontal(|ui| {
                    ui.label("Category:");
                    ui.label(format!("{} {}", node.category.icon(), node.category.display_name()));
                });

                // Description
                if let Some(desc) = &node.description {
                    ui.horizontal(|ui| {
                        ui.label("Description:");
                        ui.label(desc);
                    });
                }
            });
        }
        ui.separator();

        // Fields section
        if !node.fields.is_empty() {
            let filter = self.search_filter.clone();
            if self.render_section_header(ui, "Fields") {
                ui.indent("fields", |ui| {
                    for field in &node.fields {
                        if !filter.is_empty() && !field.name.to_lowercase().contains(&filter.to_lowercase()) {
                            continue;
                        }
                        Self::render_field_editor_static(ui, field);
                    }
                });
            }
            ui.separator();
        }

        // Ports section
        if !node.ports.inputs.is_empty() || !node.ports.outputs.is_empty() {
            if self.render_section_header(ui, "Ports") {
                ui.indent("ports", |ui| {
                    if !node.ports.inputs.is_empty() {
                        ui.label("Inputs:");
                        for port in &node.ports.inputs {
                            Self::render_port_info_static(ui, port, true);
                        }
                    }
                    if !node.ports.outputs.is_empty() {
                        ui.label("Outputs:");
                        for port in &node.ports.outputs {
                            Self::render_port_info_static(ui, port, false);
                        }
                    }
                });
            }
            ui.separator();
        }

        // Configuration section
        if !node.config.is_empty() {
            let filter = self.search_filter.clone();
            if self.render_section_header(ui, "Configuration") {
                ui.indent("config", |ui| {
                    for (key, value) in &node.config {
                        if !filter.is_empty() && !key.to_lowercase().contains(&filter.to_lowercase()) {
                            continue;
                        }
                        Self::render_config_editor_static(ui, key, value);
                    }
                });
            }
            ui.separator();
        }

        // Position section (advanced)
        if self.show_advanced {
            if self.render_section_header(ui, "Position & Size") {
                ui.indent("position", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        let mut x = node.position.x;
                        if ui.add(egui::DragValue::new(&mut x).speed(1.0)).changed() {
                            if let Some(n) = graph.get_node_mut(node.id) {
                                n.position.x = x;
                            }
                        }
                        ui.label("Y:");
                        let mut y = node.position.y;
                        if ui.add(egui::DragValue::new(&mut y).speed(1.0)).changed() {
                            if let Some(n) = graph.get_node_mut(node.id) {
                                n.position.y = y;
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Width:");
                        let mut w = node.size.width;
                        if ui.add(egui::DragValue::new(&mut w).speed(1.0).range(50.0..=1000.0)).changed() {
                            if let Some(n) = graph.get_node_mut(node.id) {
                                n.size.width = w;
                            }
                        }
                        ui.label("Height:");
                        let mut h = node.size.height;
                        if ui.add(egui::DragValue::new(&mut h).speed(1.0).range(30.0..=1000.0)).changed() {
                            if let Some(n) = graph.get_node_mut(node.id) {
                                n.size.height = h;
                            }
                        }
                    });
                });
            }
            ui.separator();
        }
    }

    /// Render properties for an edge
    fn render_edge_properties(&mut self, ui: &mut egui::Ui, edge: &Edge, graph: &mut ProjectGraph) {
        if self.render_section_header(ui, "Connection") {
            ui.indent("connection", |ui| {
                // From node
                ui.horizontal(|ui| {
                    ui.label("From:");
                    if let Some(from_node) = graph.get_node(edge.from_node) {
                        ui.label(format!("{}.{}", from_node.name, edge.from_port));
                    }
                });

                // To node
                ui.horizontal(|ui| {
                    ui.label("To:");
                    if let Some(to_node) = graph.get_node(edge.to_node) {
                        ui.label(format!("{}.{}", to_node.name, edge.to_port));
                    }
                });

                // Connection type
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.label(format!("{:?}", edge.connection_type));
                });

                // Label
                if let Some(label) = &edge.label {
                    ui.horizontal(|ui| {
                        ui.label("Label:");
                        ui.label(label);
                    });
                }
            });
        }
        ui.separator();

        // Style section
        if self.show_advanced {
            if self.render_section_header(ui, "Style") {
                ui.indent("style", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        let (r, g, b) = edge.style.color.rgb();
                        let color = egui::Color32::from_rgb(r, g, b);
                        ui.colored_label(color, "●");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Line Style:");
                        ui.label(format!("{:?}", edge.style.line_style));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Thickness:");
                        ui.label(format!("{:.1}", edge.style.thickness));
                    });
                });
            }
            ui.separator();
        }
    }

    /// Render properties for multiple selected items
    fn render_multi_selection_properties(
        &mut self,
        ui: &mut egui::Ui,
        nodes: &[imortal_core::NodeId],
        edges: &[imortal_core::EdgeId],
        _graph: &mut ProjectGraph,
    ) {
        ui.label("Multiple items selected");
        ui.separator();

        if !nodes.is_empty() {
            ui.label(format!("Nodes: {}", nodes.len()));
        }
        if !edges.is_empty() {
            ui.label(format!("Edges: {}", edges.len()));
        }

        ui.separator();
        ui.label("Bulk operations:");

        ui.horizontal(|ui| {
            if ui.button("Delete All").clicked() {
                // Would trigger delete_selected on graph
            }
            if ui.button("Group").clicked() {
                // Would trigger group_selected_nodes on graph
            }
        });
    }

    /// Render a field editor (static version to avoid borrow issues)
    fn render_field_editor_static(ui: &mut egui::Ui, field: &Field) {
        ui.horizontal(|ui| {
            // Label
            let label = field.label.as_deref().unwrap_or(&field.name);
            ui.label(format!("{}:", label));

            // Required indicator
            if field.required {
                ui.label(egui::RichText::new("*").color(egui::Color32::RED));
            }

            // Type indicator
            ui.label(egui::RichText::new(format!("({:?})", field.data_type)).weak());
        });

        // Help text
        if let Some(help) = &field.ui_hints.help {
            ui.label(egui::RichText::new(help).weak().small());
        }
    }

    /// Render port information (static version to avoid borrow issues)
    fn render_port_info_static(ui: &mut egui::Ui, port: &Port, is_input: bool) {
        let (r, g, b) = port.color_hint().rgb();
        let color = egui::Color32::from_rgb(r, g, b);

        ui.horizontal(|ui| {
            if is_input {
                ui.colored_label(color, "●");
                ui.label(&port.name);
            } else {
                ui.label(&port.name);
                ui.colored_label(color, "●");
            }

            ui.label(egui::RichText::new(format!("{:?}", port.data_type)).weak().small());

            if port.required {
                ui.label(egui::RichText::new("required").weak().small());
            }
        });

        if let Some(desc) = &port.description {
            ui.label(egui::RichText::new(desc).weak().small());
        }
    }

    /// Render a configuration value editor (static version to avoid borrow issues)
    fn render_config_editor_static(ui: &mut egui::Ui, key: &str, value: &ConfigValue) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", key));

            match value {
                ConfigValue::Bool(b) => {
                    ui.label(if *b { "✓" } else { "✗" });
                }
                ConfigValue::Int(i) => {
                    ui.label(format!("{}", i));
                }
                ConfigValue::Float(f) => {
                    ui.label(format!("{:.2}", f));
                }
                ConfigValue::String(s) => {
                    ui.label(s);
                }
                ConfigValue::Null => {
                    ui.label(egui::RichText::new("null").weak());
                }
                ConfigValue::Array(arr) => {
                    ui.label(format!("[{} items]", arr.len()));
                }
                ConfigValue::Object(obj) => {
                    ui.label(format!("{{{} keys}}", obj.len()));
                }
            }
        });
    }
}

impl Default for PropertiesPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_panel_creation() {
        let panel = PropertiesPanel::new();
        assert!(panel.search_filter.is_empty());
        assert!(!panel.show_advanced);
    }

    #[test]
    fn test_filter_matches() {
        let mut panel = PropertiesPanel::new();

        assert!(panel.filter_matches("anything"));

        panel.search_filter = "email".to_string();
        assert!(panel.filter_matches("email"));
        assert!(panel.filter_matches("Email"));
        assert!(panel.filter_matches("user_email"));
        assert!(!panel.filter_matches("password"));
    }
}
