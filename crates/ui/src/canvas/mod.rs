//! Canvas Module for Immortal Engine UI
//!
//! This module provides the main canvas widget where nodes and edges are rendered.
//! It handles:
//! - Node rendering and layout
//! - Edge/connection rendering
//! - Pan and zoom
//! - Selection and interaction
//! - Grid rendering

use eframe::egui;
use imortal_ir::{ProjectGraph, Node, Edge, Group, NodeId, EdgeId};

/// The main canvas widget for the visual editor
pub struct CanvasWidget {
    /// Canvas configuration
    pub config: CanvasConfig,
}

impl CanvasWidget {
    /// Create a new canvas widget with default configuration
    pub fn new() -> Self {
        Self {
            config: CanvasConfig::default(),
        }
    }

    /// Create a canvas widget with custom configuration
    pub fn with_config(config: CanvasConfig) -> Self {
        Self { config }
    }

    /// Render the canvas
    pub fn show(&mut self, ui: &mut egui::Ui, project: &mut ProjectGraph) -> CanvasResponse {
        let (rect, response) = ui.allocate_exact_size(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );

        let painter = ui.painter_at(rect);

        // Draw background
        painter.rect_filled(rect, 0.0, self.config.background_color);

        // Draw grid if enabled
        if self.config.show_grid {
            self.draw_grid(&painter, rect, project);
        }

        // Draw groups (behind nodes)
        for group in project.groups.values() {
            self.draw_group(&painter, rect, project, group);
        }

        // Draw edges
        for edge in project.edges.values() {
            self.draw_edge(&painter, rect, project, edge);
        }

        // Draw nodes
        for node in project.nodes.values() {
            self.draw_node(&painter, rect, project, node);
        }

        // Handle interactions
        let canvas_response = self.handle_interactions(ui, &response, project, rect);

        canvas_response
    }

    /// Draw the background grid
    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect, project: &ProjectGraph) {
        let zoom = project.viewport.zoom;
        let pan_x = project.viewport.pan_x;
        let pan_y = project.viewport.pan_y;

        let grid_size = self.config.grid_size * zoom;
        let grid_color = self.config.grid_color;

        // Calculate visible grid lines
        let start_x = ((rect.min.x - pan_x) / grid_size).floor() * grid_size + pan_x;
        let start_y = ((rect.min.y - pan_y) / grid_size).floor() * grid_size + pan_y;

        // Draw vertical lines
        let mut x = start_x;
        while x < rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, grid_color),
            );
            x += grid_size;
        }

        // Draw horizontal lines
        let mut y = start_y;
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, grid_color),
            );
            y += grid_size;
        }
    }

    /// Draw a node on the canvas
    fn draw_node(&self, painter: &egui::Painter, rect: egui::Rect, project: &ProjectGraph, node: &Node) {
        let zoom = project.viewport.zoom;
        let pan = egui::vec2(project.viewport.pan_x, project.viewport.pan_y);

        let node_rect = egui::Rect::from_min_size(
            rect.min + pan + egui::vec2(node.position.x * zoom, node.position.y * zoom),
            egui::vec2(node.size.width * zoom, node.size.height * zoom),
        );

        // Skip if not visible
        if !rect.intersects(node_rect) {
            return;
        }

        // Node background
        let bg_color = if node.selected {
            self.config.node_selected_color
        } else {
            self.config.node_background_color
        };

        painter.rect_filled(node_rect, self.config.node_corner_radius, bg_color);

        // Node border
        let border_color = if node.selected {
            self.config.node_selected_border_color
        } else {
            self.config.node_border_color
        };

        painter.rect_stroke(
            node_rect,
            self.config.node_corner_radius,
            egui::Stroke::new(self.config.node_border_width, border_color),
        );

        // Node header
        let header_height = self.config.node_header_height * zoom;
        let header_rect = egui::Rect::from_min_size(
            node_rect.min,
            egui::vec2(node_rect.width(), header_height),
        );

        let header_color = get_category_color(node.category);
        painter.rect_filled(
            header_rect,
            egui::Rounding {
                nw: self.config.node_corner_radius,
                ne: self.config.node_corner_radius,
                sw: 0.0,
                se: 0.0,
            },
            header_color,
        );

        // Node title
        let icon = node.icon.as_deref().unwrap_or("📦");
        let title = format!("{} {}", icon, node.name);
        painter.text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(14.0 * zoom),
            egui::Color32::WHITE,
        );

        // Draw ports
        self.draw_node_ports(painter, node_rect, project, node);
    }

    /// Draw ports for a node
    fn draw_node_ports(
        &self,
        painter: &egui::Painter,
        node_rect: egui::Rect,
        project: &ProjectGraph,
        node: &Node,
    ) {
        let zoom = project.viewport.zoom;
        let port_radius = self.config.port_radius * zoom;
        let header_height = self.config.node_header_height * zoom;

        // Input ports (left side)
        let input_count = node.ports.inputs.len();
        for (i, port) in node.ports.inputs.iter().enumerate() {
            let y_offset = header_height + (i as f32 + 1.0) * (node_rect.height() - header_height) / (input_count as f32 + 1.0);
            let port_pos = egui::pos2(node_rect.min.x, node_rect.min.y + y_offset);

            let color = get_port_color(&port.data_type);
            painter.circle_filled(port_pos, port_radius, color);
            painter.circle_stroke(port_pos, port_radius, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }

        // Output ports (right side)
        let output_count = node.ports.outputs.len();
        for (i, port) in node.ports.outputs.iter().enumerate() {
            let y_offset = header_height + (i as f32 + 1.0) * (node_rect.height() - header_height) / (output_count as f32 + 1.0);
            let port_pos = egui::pos2(node_rect.max.x, node_rect.min.y + y_offset);

            let color = get_port_color(&port.data_type);
            painter.circle_filled(port_pos, port_radius, color);
            painter.circle_stroke(port_pos, port_radius, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }

    /// Draw an edge on the canvas
    fn draw_edge(&self, painter: &egui::Painter, rect: egui::Rect, project: &ProjectGraph, edge: &Edge) {
        let from_node = match project.get_node(edge.from_node) {
            Some(n) => n,
            None => return,
        };
        let to_node = match project.get_node(edge.to_node) {
            Some(n) => n,
            None => return,
        };

        let zoom = project.viewport.zoom;
        let pan = egui::vec2(project.viewport.pan_x, project.viewport.pan_y);

        // Calculate start and end points
        let start = rect.min + pan + egui::vec2(
            (from_node.position.x + from_node.size.width) * zoom,
            (from_node.position.y + from_node.size.height / 2.0) * zoom,
        );
        let end = rect.min + pan + egui::vec2(
            to_node.position.x * zoom,
            (to_node.position.y + to_node.size.height / 2.0) * zoom,
        );

        // Draw bezier curve
        let color = if edge.selected {
            self.config.edge_selected_color
        } else {
            let (r, g, b) = edge.style.color.rgb();
            egui::Color32::from_rgb(r, g, b)
        };

        let control_offset = ((end.x - start.x).abs() / 2.0).max(50.0 * zoom);
        let control1 = egui::pos2(start.x + control_offset, start.y);
        let control2 = egui::pos2(end.x - control_offset, end.y);

        // Draw using line segments approximating the bezier curve
        let segments = 20;
        let mut prev_point = start;
        for i in 1..=segments {
            let t = i as f32 / segments as f32;
            let point = cubic_bezier(start, control1, control2, end, t);
            painter.line_segment(
                [prev_point, point],
                egui::Stroke::new(self.config.edge_width * zoom, color),
            );
            prev_point = point;
        }

        // Draw arrow head
        let arrow_size = self.config.arrow_size * zoom;
        let direction = (end - prev_point).normalized();
        let perpendicular = egui::vec2(-direction.y, direction.x);

        let arrow_points = vec![
            end,
            end - direction * arrow_size + perpendicular * arrow_size * 0.5,
            end - direction * arrow_size - perpendicular * arrow_size * 0.5,
        ];

        painter.add(egui::Shape::convex_polygon(
            arrow_points,
            color,
            egui::Stroke::NONE,
        ));
    }

    /// Draw a group on the canvas
    fn draw_group(&self, painter: &egui::Painter, rect: egui::Rect, project: &ProjectGraph, group: &Group) {
        let zoom = project.viewport.zoom;
        let pan = egui::vec2(project.viewport.pan_x, project.viewport.pan_y);

        let group_rect = egui::Rect::from_min_size(
            rect.min + pan + egui::vec2(group.position.x * zoom, group.position.y * zoom),
            egui::vec2(group.size.width * zoom, group.size.height * zoom),
        );

        let (r, g, b) = group.color.rgb();
        let bg_color = egui::Color32::from_rgba_unmultiplied(r, g, b, (group.opacity * 255.0) as u8);

        painter.rect_filled(group_rect, 8.0, bg_color);
        painter.rect_stroke(
            group_rect,
            8.0,
            egui::Stroke::new(1.0, egui::Color32::from_rgb(r, g, b)),
        );

        // Group title
        if group.show_header {
            painter.text(
                group_rect.min + egui::vec2(10.0, 15.0),
                egui::Align2::LEFT_CENTER,
                &group.name,
                egui::FontId::proportional(12.0 * zoom),
                egui::Color32::from_rgb(r, g, b),
            );
        }
    }

    /// Handle user interactions with the canvas
    fn handle_interactions(
        &self,
        _ui: &mut egui::Ui,
        response: &egui::Response,
        project: &mut ProjectGraph,
        _rect: egui::Rect,
    ) -> CanvasResponse {
        let mut canvas_response = CanvasResponse::default();

        // Handle panning
        if response.dragged_by(egui::PointerButton::Middle) ||
           (response.dragged() && response.ctx.input(|i| i.modifiers.shift)) {
            let delta = response.drag_delta();
            project.pan(delta.x, delta.y);
            canvas_response.panned = true;
        }

        // Handle zooming with scroll
        if response.hovered() {
            let scroll_delta = response.ctx.input(|i| i.raw_scroll_delta.y);
            if scroll_delta != 0.0 {
                let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
                project.zoom(zoom_factor);
                canvas_response.zoomed = true;
            }
        }

        canvas_response
    }
}

impl Default for CanvasWidget {
    fn default() -> Self {
        Self::new()
    }
}

/// Canvas configuration
#[derive(Debug, Clone)]
pub struct CanvasConfig {
    /// Background color
    pub background_color: egui::Color32,
    /// Whether to show grid
    pub show_grid: bool,
    /// Grid size in pixels
    pub grid_size: f32,
    /// Grid color
    pub grid_color: egui::Color32,
    /// Node background color
    pub node_background_color: egui::Color32,
    /// Node selected background color
    pub node_selected_color: egui::Color32,
    /// Node border color
    pub node_border_color: egui::Color32,
    /// Node selected border color
    pub node_selected_border_color: egui::Color32,
    /// Node border width
    pub node_border_width: f32,
    /// Node corner radius
    pub node_corner_radius: f32,
    /// Node header height
    pub node_header_height: f32,
    /// Port radius
    pub port_radius: f32,
    /// Edge width
    pub edge_width: f32,
    /// Edge selected color
    pub edge_selected_color: egui::Color32,
    /// Arrow size
    pub arrow_size: f32,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            background_color: egui::Color32::from_rgb(30, 30, 35),
            show_grid: true,
            grid_size: 20.0,
            grid_color: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            node_background_color: egui::Color32::from_rgb(50, 50, 55),
            node_selected_color: egui::Color32::from_rgb(60, 80, 120),
            node_border_color: egui::Color32::from_rgb(80, 80, 85),
            node_selected_border_color: egui::Color32::from_rgb(100, 150, 255),
            node_border_width: 2.0,
            node_corner_radius: 5.0,
            node_header_height: 25.0,
            port_radius: 6.0,
            edge_width: 2.0,
            edge_selected_color: egui::Color32::from_rgb(100, 200, 255),
            arrow_size: 10.0,
        }
    }
}

/// Response from canvas interactions
#[derive(Debug, Clone, Default)]
pub struct CanvasResponse {
    /// Whether the canvas was panned
    pub panned: bool,
    /// Whether the canvas was zoomed
    pub zoomed: bool,
    /// Node that was clicked
    pub clicked_node: Option<NodeId>,
    /// Edge that was clicked
    pub clicked_edge: Option<EdgeId>,
    /// Whether a connection drag was started
    pub connection_started: bool,
    /// Whether a connection was completed
    pub connection_completed: bool,
}

/// Calculate a point on a cubic bezier curve
fn cubic_bezier(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, p3: egui::Pos2, t: f32) -> egui::Pos2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    egui::pos2(
        mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x,
        mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y,
    )
}

/// Get color for a component category
fn get_category_color(category: imortal_core::ComponentCategory) -> egui::Color32 {
    match category {
        imortal_core::ComponentCategory::Auth => egui::Color32::from_rgb(200, 80, 80),
        imortal_core::ComponentCategory::Data => egui::Color32::from_rgb(80, 150, 200),
        imortal_core::ComponentCategory::Api => egui::Color32::from_rgb(150, 200, 80),
        imortal_core::ComponentCategory::Storage => egui::Color32::from_rgb(200, 150, 80),
        imortal_core::ComponentCategory::Logic => egui::Color32::from_rgb(200, 80, 200),
        imortal_core::ComponentCategory::Ui => egui::Color32::from_rgb(80, 200, 200),
        imortal_core::ComponentCategory::Embedded => egui::Color32::from_rgb(200, 200, 80),
        imortal_core::ComponentCategory::Custom => egui::Color32::from_rgb(100, 100, 100),
    }
}

/// Get color for a port based on data type
fn get_port_color(data_type: &imortal_core::DataType) -> egui::Color32 {
    match data_type {
        imortal_core::DataType::String | imortal_core::DataType::Text => egui::Color32::from_rgb(255, 200, 100),
        imortal_core::DataType::Int32 | imortal_core::DataType::Int64 => egui::Color32::from_rgb(100, 200, 255),
        imortal_core::DataType::Float32 | imortal_core::DataType::Float64 => egui::Color32::from_rgb(100, 255, 200),
        imortal_core::DataType::Bool => egui::Color32::from_rgb(255, 100, 100),
        imortal_core::DataType::Entity(_) | imortal_core::DataType::Reference(_) => egui::Color32::from_rgb(200, 100, 255),
        imortal_core::DataType::Array(_) => egui::Color32::from_rgb(255, 150, 200),
        imortal_core::DataType::Trigger => egui::Color32::from_rgb(255, 255, 100),
        imortal_core::DataType::Any => egui::Color32::WHITE,
        _ => egui::Color32::from_rgb(150, 150, 150),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_config_default() {
        let config = CanvasConfig::default();
        assert!(config.show_grid);
        assert_eq!(config.grid_size, 20.0);
    }

    #[test]
    fn test_cubic_bezier() {
        let p0 = egui::pos2(0.0, 0.0);
        let p1 = egui::pos2(1.0, 0.0);
        let p2 = egui::pos2(1.0, 1.0);
        let p3 = egui::pos2(2.0, 1.0);

        let start = cubic_bezier(p0, p1, p2, p3, 0.0);
        assert!((start.x - p0.x).abs() < 0.001);
        assert!((start.y - p0.y).abs() < 0.001);

        let end = cubic_bezier(p0, p1, p2, p3, 1.0);
        assert!((end.x - p3.x).abs() < 0.001);
        assert!((end.y - p3.y).abs() < 0.001);
    }
}
