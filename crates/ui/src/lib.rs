//! Immortal Engine UI
//!
//! Visual editor for creating applications through drag-and-drop components
//! and drawing connections between them.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      ImmortalApp                            │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │                    Menu Bar                         │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! │  ┌──────────┐  ┌────────────────────────┐  ┌───────────┐   │
//! │  │ Component│  │                        │  │Properties │   │
//! │  │ Palette  │  │        Canvas          │  │  Panel    │   │
//! │  │          │  │                        │  │           │   │
//! │  │ - Auth   │  │   [Nodes & Edges]      │  │ - Fields  │   │
//! │  │ - Data   │  │                        │  │ - Config  │   │
//! │  │ - API    │  │                        │  │ - Ports   │   │
//! │  │ - etc    │  │                        │  │           │   │
//! │  └──────────┘  └────────────────────────┘  └───────────┘   │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │                   Status Bar                        │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod app;
pub mod canvas;
pub mod palette;
pub mod properties;
pub mod toolbar;
pub mod dialogs;
pub mod theme;
pub mod state;

pub use app::ImmortalApp;

use eframe::egui;
use imortal_ir::ProjectGraph;

/// Run the Immortal Engine UI application
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Immortal Engine"),
        ..Default::default()
    };

    eframe::run_native(
        "Immortal Engine",
        options,
        Box::new(|cc| {
            // Setup custom fonts and styles
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(ImmortalApp::new(cc)))
        }),
    )
}

/// Run with an existing project
pub fn run_with_project(project: ProjectGraph) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title(format!("Immortal Engine - {}", project.meta.name)),
        ..Default::default()
    };

    eframe::run_native(
        "Immortal Engine",
        options,
        Box::new(move |cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(ImmortalApp::with_project(cc, project)))
        }),
    )
}

/// Setup custom fonts for the UI
fn setup_fonts(ctx: &egui::Context) {
    let fonts = egui::FontDefinitions::default();

    // You can add custom fonts here
    // fonts.font_data.insert(
    //     "my_font".to_owned(),
    //     egui::FontData::from_static(include_bytes!("../assets/fonts/MyFont.ttf")),
    // );

    ctx.set_fonts(fonts);

    // Set default style
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    ctx.set_style(style);
}

/// UI configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiConfig {
    /// Show grid on canvas
    pub show_grid: bool,
    /// Grid size in pixels
    pub grid_size: f32,
    /// Snap to grid when moving nodes
    pub snap_to_grid: bool,
    /// Show minimap
    pub show_minimap: bool,
    /// Theme (dark/light)
    pub dark_mode: bool,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval: u32,
    /// Show component descriptions in palette
    pub show_descriptions: bool,
    /// Animation speed (0.0 - 1.0)
    pub animation_speed: f32,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_grid: true,
            grid_size: 20.0,
            snap_to_grid: true,
            show_minimap: true,
            dark_mode: true,
            auto_save_interval: 60,
            show_descriptions: true,
            animation_speed: 0.5,
        }
    }
}

/// Prelude for convenient imports
pub mod prelude {
    pub use super::{ImmortalApp, UiConfig, run, run_with_project};
    pub use super::state::EditorState;
    pub use super::canvas::CanvasWidget;
    pub use super::palette::PaletteWidget;
    pub use super::properties::PropertiesPanel;
}
