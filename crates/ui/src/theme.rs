//! UI Theme for Immortal Engine
//!
//! This module provides theming and styling utilities for the visual editor.

use eframe::egui;

/// Theme preset options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemePreset {
    /// Dark theme (default)
    #[default]
    Dark,
    /// Light theme
    Light,
    /// High contrast dark
    HighContrastDark,
    /// High contrast light
    HighContrastLight,
}

impl ThemePreset {
    /// Get all available presets
    pub fn all() -> &'static [ThemePreset] {
        &[
            ThemePreset::Dark,
            ThemePreset::Light,
            ThemePreset::HighContrastDark,
            ThemePreset::HighContrastLight,
        ]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            ThemePreset::Dark => "Dark",
            ThemePreset::Light => "Light",
            ThemePreset::HighContrastDark => "High Contrast Dark",
            ThemePreset::HighContrastLight => "High Contrast Light",
        }
    }
}

/// Theme configuration for the editor
#[derive(Debug, Clone)]
pub struct Theme {
    /// Current preset
    pub preset: ThemePreset,

    /// Canvas colors
    pub canvas: CanvasColors,

    /// Node colors
    pub node: NodeColors,

    /// Edge colors
    pub edge: EdgeColors,

    /// Port colors
    pub port: PortColors,

    /// UI colors
    pub ui: UiColors,
}

impl Theme {
    /// Create a theme from a preset
    pub fn from_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::Dark => Self::dark(),
            ThemePreset::Light => Self::light(),
            ThemePreset::HighContrastDark => Self::high_contrast_dark(),
            ThemePreset::HighContrastLight => Self::high_contrast_light(),
        }
    }

    /// Create the default dark theme
    pub fn dark() -> Self {
        Self {
            preset: ThemePreset::Dark,
            canvas: CanvasColors {
                background: egui::Color32::from_rgb(30, 30, 35),
                grid: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
                grid_major: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40),
                selection_box: egui::Color32::from_rgba_unmultiplied(100, 150, 255, 50),
                selection_border: egui::Color32::from_rgb(100, 150, 255),
            },
            node: NodeColors {
                background: egui::Color32::from_rgb(50, 50, 55),
                background_selected: egui::Color32::from_rgb(60, 80, 120),
                border: egui::Color32::from_rgb(80, 80, 85),
                border_selected: egui::Color32::from_rgb(100, 150, 255),
                border_hovered: egui::Color32::from_rgb(120, 120, 130),
                header_text: egui::Color32::WHITE,
                body_text: egui::Color32::from_rgb(200, 200, 200),
                shadow: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
            },
            edge: EdgeColors {
                default: egui::Color32::from_rgb(150, 150, 150),
                selected: egui::Color32::from_rgb(100, 200, 255),
                hovered: egui::Color32::from_rgb(180, 180, 180),
                invalid: egui::Color32::from_rgb(255, 100, 100),
                drawing: egui::Color32::from_rgba_unmultiplied(100, 200, 255, 150),
            },
            port: PortColors {
                string: egui::Color32::from_rgb(255, 200, 100),
                integer: egui::Color32::from_rgb(100, 200, 255),
                float: egui::Color32::from_rgb(100, 255, 200),
                boolean: egui::Color32::from_rgb(255, 100, 100),
                entity: egui::Color32::from_rgb(200, 100, 255),
                trigger: egui::Color32::from_rgb(255, 255, 100),
                any: egui::Color32::from_rgb(200, 200, 200),
            },
            ui: UiColors {
                panel_background: egui::Color32::from_rgb(40, 40, 45),
                panel_border: egui::Color32::from_rgb(60, 60, 65),
                text: egui::Color32::from_rgb(220, 220, 220),
                text_muted: egui::Color32::from_rgb(150, 150, 150),
                accent: egui::Color32::from_rgb(100, 150, 255),
                success: egui::Color32::from_rgb(100, 200, 100),
                warning: egui::Color32::from_rgb(255, 180, 50),
                error: egui::Color32::from_rgb(255, 100, 100),
            },
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            preset: ThemePreset::Light,
            canvas: CanvasColors {
                background: egui::Color32::from_rgb(245, 245, 248),
                grid: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 15),
                grid_major: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 30),
                selection_box: egui::Color32::from_rgba_unmultiplied(50, 100, 200, 50),
                selection_border: egui::Color32::from_rgb(50, 100, 200),
            },
            node: NodeColors {
                background: egui::Color32::WHITE,
                background_selected: egui::Color32::from_rgb(230, 240, 255),
                border: egui::Color32::from_rgb(200, 200, 205),
                border_selected: egui::Color32::from_rgb(50, 100, 200),
                border_hovered: egui::Color32::from_rgb(150, 150, 160),
                header_text: egui::Color32::WHITE,
                body_text: egui::Color32::from_rgb(50, 50, 50),
                shadow: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 30),
            },
            edge: EdgeColors {
                default: egui::Color32::from_rgb(100, 100, 100),
                selected: egui::Color32::from_rgb(50, 100, 200),
                hovered: egui::Color32::from_rgb(80, 80, 80),
                invalid: egui::Color32::from_rgb(200, 50, 50),
                drawing: egui::Color32::from_rgba_unmultiplied(50, 100, 200, 150),
            },
            port: PortColors {
                string: egui::Color32::from_rgb(200, 150, 50),
                integer: egui::Color32::from_rgb(50, 150, 200),
                float: egui::Color32::from_rgb(50, 200, 150),
                boolean: egui::Color32::from_rgb(200, 50, 50),
                entity: egui::Color32::from_rgb(150, 50, 200),
                trigger: egui::Color32::from_rgb(200, 200, 50),
                any: egui::Color32::from_rgb(100, 100, 100),
            },
            ui: UiColors {
                panel_background: egui::Color32::from_rgb(250, 250, 252),
                panel_border: egui::Color32::from_rgb(220, 220, 225),
                text: egui::Color32::from_rgb(30, 30, 30),
                text_muted: egui::Color32::from_rgb(100, 100, 100),
                accent: egui::Color32::from_rgb(50, 100, 200),
                success: egui::Color32::from_rgb(50, 150, 50),
                warning: egui::Color32::from_rgb(200, 140, 30),
                error: egui::Color32::from_rgb(200, 50, 50),
            },
        }
    }

    /// Create a high contrast dark theme
    pub fn high_contrast_dark() -> Self {
        Self {
            preset: ThemePreset::HighContrastDark,
            canvas: CanvasColors {
                background: egui::Color32::BLACK,
                grid: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40),
                grid_major: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 80),
                selection_box: egui::Color32::from_rgba_unmultiplied(0, 200, 255, 80),
                selection_border: egui::Color32::from_rgb(0, 200, 255),
            },
            node: NodeColors {
                background: egui::Color32::from_rgb(20, 20, 20),
                background_selected: egui::Color32::from_rgb(30, 50, 80),
                border: egui::Color32::from_rgb(200, 200, 200),
                border_selected: egui::Color32::from_rgb(0, 200, 255),
                border_hovered: egui::Color32::WHITE,
                header_text: egui::Color32::WHITE,
                body_text: egui::Color32::WHITE,
                shadow: egui::Color32::TRANSPARENT,
            },
            edge: EdgeColors {
                default: egui::Color32::WHITE,
                selected: egui::Color32::from_rgb(0, 255, 255),
                hovered: egui::Color32::from_rgb(255, 255, 0),
                invalid: egui::Color32::from_rgb(255, 0, 0),
                drawing: egui::Color32::from_rgb(0, 255, 255),
            },
            port: PortColors {
                string: egui::Color32::from_rgb(255, 200, 0),
                integer: egui::Color32::from_rgb(0, 200, 255),
                float: egui::Color32::from_rgb(0, 255, 200),
                boolean: egui::Color32::from_rgb(255, 0, 0),
                entity: egui::Color32::from_rgb(255, 0, 255),
                trigger: egui::Color32::from_rgb(255, 255, 0),
                any: egui::Color32::WHITE,
            },
            ui: UiColors {
                panel_background: egui::Color32::BLACK,
                panel_border: egui::Color32::WHITE,
                text: egui::Color32::WHITE,
                text_muted: egui::Color32::from_rgb(180, 180, 180),
                accent: egui::Color32::from_rgb(0, 200, 255),
                success: egui::Color32::from_rgb(0, 255, 0),
                warning: egui::Color32::from_rgb(255, 255, 0),
                error: egui::Color32::from_rgb(255, 0, 0),
            },
        }
    }

    /// Create a high contrast light theme
    pub fn high_contrast_light() -> Self {
        Self {
            preset: ThemePreset::HighContrastLight,
            canvas: CanvasColors {
                background: egui::Color32::WHITE,
                grid: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 40),
                grid_major: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80),
                selection_box: egui::Color32::from_rgba_unmultiplied(0, 0, 200, 80),
                selection_border: egui::Color32::from_rgb(0, 0, 200),
            },
            node: NodeColors {
                background: egui::Color32::WHITE,
                background_selected: egui::Color32::from_rgb(220, 230, 255),
                border: egui::Color32::BLACK,
                border_selected: egui::Color32::from_rgb(0, 0, 200),
                border_hovered: egui::Color32::from_rgb(50, 50, 50),
                header_text: egui::Color32::WHITE,
                body_text: egui::Color32::BLACK,
                shadow: egui::Color32::TRANSPARENT,
            },
            edge: EdgeColors {
                default: egui::Color32::BLACK,
                selected: egui::Color32::from_rgb(0, 0, 200),
                hovered: egui::Color32::from_rgb(100, 100, 100),
                invalid: egui::Color32::from_rgb(200, 0, 0),
                drawing: egui::Color32::from_rgb(0, 0, 200),
            },
            port: PortColors {
                string: egui::Color32::from_rgb(180, 130, 0),
                integer: egui::Color32::from_rgb(0, 100, 180),
                float: egui::Color32::from_rgb(0, 150, 100),
                boolean: egui::Color32::from_rgb(180, 0, 0),
                entity: egui::Color32::from_rgb(150, 0, 150),
                trigger: egui::Color32::from_rgb(150, 150, 0),
                any: egui::Color32::BLACK,
            },
            ui: UiColors {
                panel_background: egui::Color32::WHITE,
                panel_border: egui::Color32::BLACK,
                text: egui::Color32::BLACK,
                text_muted: egui::Color32::from_rgb(80, 80, 80),
                accent: egui::Color32::from_rgb(0, 0, 200),
                success: egui::Color32::from_rgb(0, 150, 0),
                warning: egui::Color32::from_rgb(180, 130, 0),
                error: egui::Color32::from_rgb(200, 0, 0),
            },
        }
    }

    /// Apply this theme to an egui context
    pub fn apply(&self, ctx: &egui::Context) {
        let visuals = match self.preset {
            ThemePreset::Dark | ThemePreset::HighContrastDark => egui::Visuals::dark(),
            ThemePreset::Light | ThemePreset::HighContrastLight => egui::Visuals::light(),
        };
        ctx.set_visuals(visuals);
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Canvas-related colors
#[derive(Debug, Clone)]
pub struct CanvasColors {
    /// Canvas background color
    pub background: egui::Color32,
    /// Minor grid line color
    pub grid: egui::Color32,
    /// Major grid line color
    pub grid_major: egui::Color32,
    /// Selection box fill color
    pub selection_box: egui::Color32,
    /// Selection box border color
    pub selection_border: egui::Color32,
}

/// Node-related colors
#[derive(Debug, Clone)]
pub struct NodeColors {
    /// Node background color
    pub background: egui::Color32,
    /// Selected node background color
    pub background_selected: egui::Color32,
    /// Node border color
    pub border: egui::Color32,
    /// Selected node border color
    pub border_selected: egui::Color32,
    /// Hovered node border color
    pub border_hovered: egui::Color32,
    /// Header text color
    pub header_text: egui::Color32,
    /// Body text color
    pub body_text: egui::Color32,
    /// Shadow color
    pub shadow: egui::Color32,
}

/// Edge-related colors
#[derive(Debug, Clone)]
pub struct EdgeColors {
    /// Default edge color
    pub default: egui::Color32,
    /// Selected edge color
    pub selected: egui::Color32,
    /// Hovered edge color
    pub hovered: egui::Color32,
    /// Invalid connection color
    pub invalid: egui::Color32,
    /// Color while drawing a new connection
    pub drawing: egui::Color32,
}

/// Port-related colors by data type
#[derive(Debug, Clone)]
pub struct PortColors {
    /// String type color
    pub string: egui::Color32,
    /// Integer type color
    pub integer: egui::Color32,
    /// Float type color
    pub float: egui::Color32,
    /// Boolean type color
    pub boolean: egui::Color32,
    /// Entity/reference type color
    pub entity: egui::Color32,
    /// Trigger/event type color
    pub trigger: egui::Color32,
    /// Any/unknown type color
    pub any: egui::Color32,
}

impl PortColors {
    /// Get color for a data type
    pub fn for_data_type(&self, data_type: &imortal_core::DataType) -> egui::Color32 {
        use imortal_core::DataType;
        match data_type {
            DataType::String | DataType::Text => self.string,
            DataType::Int32 | DataType::Int64 => self.integer,
            DataType::Float32 | DataType::Float64 => self.float,
            DataType::Bool => self.boolean,
            DataType::Entity(_) | DataType::Reference(_) => self.entity,
            DataType::Trigger => self.trigger,
            DataType::Any => self.any,
            _ => self.any,
        }
    }
}

/// UI element colors
#[derive(Debug, Clone)]
pub struct UiColors {
    /// Panel background color
    pub panel_background: egui::Color32,
    /// Panel border color
    pub panel_border: egui::Color32,
    /// Primary text color
    pub text: egui::Color32,
    /// Muted/secondary text color
    pub text_muted: egui::Color32,
    /// Accent color for highlights
    pub accent: egui::Color32,
    /// Success indicator color
    pub success: egui::Color32,
    /// Warning indicator color
    pub warning: egui::Color32,
    /// Error indicator color
    pub error: egui::Color32,
}

/// Category-specific header colors
pub fn category_color(category: imortal_core::ComponentCategory) -> egui::Color32 {
    use imortal_core::ComponentCategory;
    match category {
        ComponentCategory::Auth => egui::Color32::from_rgb(200, 80, 80),
        ComponentCategory::Data => egui::Color32::from_rgb(80, 150, 200),
        ComponentCategory::Api => egui::Color32::from_rgb(150, 200, 80),
        ComponentCategory::Storage => egui::Color32::from_rgb(200, 150, 80),
        ComponentCategory::Logic => egui::Color32::from_rgb(200, 80, 200),
        ComponentCategory::Ui => egui::Color32::from_rgb(80, 200, 200),
        ComponentCategory::Embedded => egui::Color32::from_rgb(200, 200, 80),
        ComponentCategory::Custom => egui::Color32::from_rgb(150, 150, 150),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_presets() {
        let dark = Theme::dark();
        assert_eq!(dark.preset, ThemePreset::Dark);

        let light = Theme::light();
        assert_eq!(light.preset, ThemePreset::Light);
    }

    #[test]
    fn test_theme_from_preset() {
        let theme = Theme::from_preset(ThemePreset::Light);
        assert_eq!(theme.preset, ThemePreset::Light);
    }

    #[test]
    fn test_preset_names() {
        assert_eq!(ThemePreset::Dark.name(), "Dark");
        assert_eq!(ThemePreset::Light.name(), "Light");
    }

    #[test]
    fn test_category_colors() {
        let auth_color = category_color(imortal_core::ComponentCategory::Auth);
        let data_color = category_color(imortal_core::ComponentCategory::Data);
        assert_ne!(auth_color, data_color);
    }
}
