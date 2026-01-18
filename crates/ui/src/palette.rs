//! Component Palette Widget
//!
//! This module provides the component palette UI widget, which displays
//! available components organized by category that users can drag onto the canvas.

use eframe::egui;
use imortal_core::ComponentCategory;
use imortal_components::{ComponentRegistry, ComponentDefinition};
use imortal_ir::Node;

/// The component palette widget
pub struct PaletteWidget;

impl PaletteWidget {
    /// Show the palette widget
    pub fn show(
        ui: &mut egui::Ui,
        registry: &ComponentRegistry,
        search_text: &mut String,
        expanded_categories: &mut std::collections::HashSet<ComponentCategory>,
    ) -> PaletteResponse {
        let mut response = PaletteResponse::default();

        // Search box
        ui.horizontal(|ui| {
            ui.label("🔍");
            let search_response = ui.add(
                egui::TextEdit::singleline(search_text)
                    .hint_text("Search components...")
                    .desired_width(ui.available_width())
            );

            if search_response.changed() {
                response.search_changed = true;
            }
        });

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        // Component list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for category in ComponentCategory::all() {
                    // Filter components for this category
                    let components: Vec<&ComponentDefinition> = if search_text.is_empty() {
                        registry.by_category(*category)
                    } else {
                        let search_lower = search_text.to_lowercase();
                        registry.by_category(*category)
                            .into_iter()
                            .filter(|c| {
                                c.name.to_lowercase().contains(&search_lower)
                                    || c.description.to_lowercase().contains(&search_lower)
                                    || c.tags.iter().any(|t| t.to_lowercase().contains(&search_lower))
                            })
                            .collect()
                    };

                    if components.is_empty() {
                        continue;
                    }

                    let is_expanded = expanded_categories.contains(category);
                    let header = format!("{} {} ({})",
                        category.icon(),
                        category.display_name(),
                        components.len()
                    );

                    // Collect component info for the closure
                    let component_info: Vec<(String, String, String)> = components.iter()
                        .map(|c| (c.id.clone(), c.icon.to_string(), c.name.clone()))
                        .collect();

                    let header_response = ui.collapsing(header, |ui| {
                        for (idx, (id, icon, name)) in component_info.iter().enumerate() {
                            let component = &components[idx];

                            let button = egui::Button::new(format!("{} {}", icon, name))
                                .min_size(egui::vec2(ui.available_width() - 20.0, 28.0));

                            let button_response = ui.add(button);

                            if button_response.clicked() {
                                response.selected_component = Some(id.clone());
                            }

                            if button_response.drag_started() {
                                response.dragged_component = Some(id.clone());
                            }

                            // Tooltip with description
                            button_response.on_hover_ui(|ui| {
                                ui.vertical(|ui| {
                                    ui.strong(&component.name);
                                    ui.label(&component.description);

                                    if !component.tags.is_empty() {
                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            for tag in &component.tags {
                                                ui.label(egui::RichText::new(format!("#{}", tag)).small().color(egui::Color32::GRAY));
                                            }
                                        });
                                    }
                                });
                            });
                        }
                    });

                    // Track expanded state
                    if header_response.header_response.clicked() {
                        if is_expanded {
                            expanded_categories.remove(category);
                        } else {
                            expanded_categories.insert(*category);
                        }
                    }
                }
            });

        response
    }
}

/// Response from the palette widget
#[derive(Debug, Clone, Default)]
pub struct PaletteResponse {
    /// ID of the selected component (if any)
    pub selected_component: Option<String>,
    /// ID of the dragged component (if any)
    pub dragged_component: Option<String>,
    /// Whether the search text changed
    pub search_changed: bool,
}

impl PaletteResponse {
    /// Check if a component was selected
    pub fn has_selection(&self) -> bool {
        self.selected_component.is_some()
    }

    /// Check if a component is being dragged
    pub fn is_dragging(&self) -> bool {
        self.dragged_component.is_some()
    }
}

/// State for the palette widget
#[derive(Debug, Clone, Default)]
pub struct PaletteState {
    /// Current search text
    pub search_text: String,
    /// Which categories are expanded
    pub expanded_categories: std::collections::HashSet<ComponentCategory>,
    /// Currently dragging component ID
    pub dragging_component: Option<String>,
}

impl PaletteState {
    /// Create a new palette state with all categories expanded
    pub fn new() -> Self {
        let mut expanded = std::collections::HashSet::new();
        for category in ComponentCategory::all() {
            expanded.insert(*category);
        }

        Self {
            search_text: String::new(),
            expanded_categories: expanded,
            dragging_component: None,
        }
    }

    /// Clear the search text
    pub fn clear_search(&mut self) {
        self.search_text.clear();
    }

    /// Collapse all categories
    pub fn collapse_all(&mut self) {
        self.expanded_categories.clear();
    }

    /// Expand all categories
    pub fn expand_all(&mut self) {
        for category in ComponentCategory::all() {
            self.expanded_categories.insert(*category);
        }
    }

    /// Toggle a category's expanded state
    pub fn toggle_category(&mut self, category: ComponentCategory) {
        if self.expanded_categories.contains(&category) {
            self.expanded_categories.remove(&category);
        } else {
            self.expanded_categories.insert(category);
        }
    }
}

/// Create a node from a component definition at a given position
pub fn create_node_at(component: &ComponentDefinition, x: f32, y: f32) -> Node {
    let mut node = component.instantiate_default();
    node.position.x = x;
    node.position.y = y;
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_state_creation() {
        let state = PaletteState::new();
        assert!(state.search_text.is_empty());
        assert!(!state.expanded_categories.is_empty());
    }

    #[test]
    fn test_palette_state_toggle() {
        let mut state = PaletteState::new();

        // Initially expanded
        assert!(state.expanded_categories.contains(&ComponentCategory::Auth));

        // Toggle to collapse
        state.toggle_category(ComponentCategory::Auth);
        assert!(!state.expanded_categories.contains(&ComponentCategory::Auth));

        // Toggle to expand again
        state.toggle_category(ComponentCategory::Auth);
        assert!(state.expanded_categories.contains(&ComponentCategory::Auth));
    }

    #[test]
    fn test_palette_state_collapse_expand_all() {
        let mut state = PaletteState::new();

        state.collapse_all();
        assert!(state.expanded_categories.is_empty());

        state.expand_all();
        assert!(!state.expanded_categories.is_empty());
    }

    #[test]
    fn test_palette_response_default() {
        let response = PaletteResponse::default();
        assert!(!response.has_selection());
        assert!(!response.is_dragging());
        assert!(!response.search_changed);
    }
}
