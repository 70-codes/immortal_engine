//! Main Application for Immortal Engine UI
//!
//! This module provides the main application struct that ties together
//! all UI components: canvas, palette, properties panel, etc.

use eframe::egui;
use imortal_ir::{ProjectGraph, ProjectMeta, Node, Edge, Field};
use imortal_core::{DataType, NodeId};
use imortal_components::ComponentRegistry;
use std::path::PathBuf;

use crate::state::{EditorState, History};
use crate::welcome::{WelcomeScreen, WelcomeAction, NewProjectInfo};
use crate::UiConfig;

/// Application mode - either showing welcome screen or the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// Showing the welcome/start screen
    Welcome,
    /// Showing the main editor
    Editor,
}

/// The main Immortal Engine application
pub struct ImmortalApp {
    /// Current application mode
    pub mode: AppMode,

    /// Welcome screen state
    pub welcome_screen: WelcomeScreen,

    /// The project graph being edited
    pub project: ProjectGraph,

    /// Editor state (selection, tool, etc.)
    pub state: EditorState,

    /// Component registry
    pub registry: ComponentRegistry,

    /// UI configuration
    pub config: UiConfig,

    /// Whether the about dialog is open
    show_about: bool,

    /// Whether the settings dialog is open
    show_settings: bool,

    /// Whether the new project dialog is open
    show_new_project: bool,

    /// State for adding a new field
    new_field_name: String,
    new_field_type: usize,

    /// Connection drawing state
    drawing_connection: bool,
    connection_from_node: Option<NodeId>,
    connection_from_port: String,
    connection_mouse_pos: egui::Pos2,

    /// Status message to display
    status_message: Option<(String, std::time::Instant)>,

    /// Current project file path
    project_path: Option<PathBuf>,

    /// Undo/Redo history
    history: History,

    /// Database connection test result (node_id, success, message, timestamp)
    db_connection_result: Option<(NodeId, bool, String, std::time::Instant)>,
}

impl ImmortalApp {
    /// Create a new application - starts with welcome screen
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            mode: AppMode::Welcome,
            welcome_screen: WelcomeScreen::new(),
            project: ProjectGraph::new(ProjectMeta::new("Untitled")),
            state: EditorState::new(),
            registry: ComponentRegistry::with_builtins(),
            config: UiConfig::default(),
            show_about: false,
            show_settings: false,
            show_new_project: false,
            status_message: None,
            project_path: None,
            new_field_name: String::new(),
            new_field_type: 0,
            drawing_connection: false,
            connection_from_node: None,
            connection_from_port: String::new(),
            connection_mouse_pos: egui::Pos2::ZERO,
            history: History::new(),
            db_connection_result: None,
        }
    }

    /// Create a new application with an existing project (skips welcome screen)
    pub fn with_project(_cc: &eframe::CreationContext<'_>, project: ProjectGraph) -> Self {
        Self {
            mode: AppMode::Editor,
            welcome_screen: WelcomeScreen::new(),
            project,
            state: EditorState::new(),
            registry: ComponentRegistry::with_builtins(),
            config: UiConfig::default(),
            show_about: false,
            show_settings: false,
            show_new_project: false,
            status_message: None,
            project_path: None,
            new_field_name: String::new(),
            new_field_type: 0,
            drawing_connection: false,
            connection_from_node: None,
            connection_from_port: String::new(),
            connection_mouse_pos: egui::Pos2::ZERO,
            history: History::new(),
            db_connection_result: None,
        }
    }

    /// Create a new application with a project loaded from a path
    pub fn with_project_path(_cc: &eframe::CreationContext<'_>, project: ProjectGraph, path: PathBuf) -> Self {
        let mut welcome_screen = WelcomeScreen::new();
        welcome_screen.add_recent_project(project.meta.name.clone(), path.clone());

        Self {
            mode: AppMode::Editor,
            welcome_screen,
            project,
            state: EditorState::new(),
            registry: ComponentRegistry::with_builtins(),
            config: UiConfig::default(),
            show_about: false,
            show_settings: false,
            show_new_project: false,
            status_message: None,
            project_path: Some(path),
            new_field_name: String::new(),
            new_field_type: 0,
            drawing_connection: false,
            connection_from_node: None,
            connection_from_port: String::new(),
            connection_mouse_pos: egui::Pos2::ZERO,
            history: History::new(),
            db_connection_result: None,
        }
    }

    /// Handle welcome screen actions
    fn handle_welcome_action(&mut self, action: WelcomeAction) {
        match action {
            WelcomeAction::None => {}
            WelcomeAction::CreateProject(info) => {
                self.create_new_project(info);
            }
            WelcomeAction::OpenProject(path) => {
                self.load_project_from_path(path);
            }
            WelcomeAction::OpenRecentProject(path) => {
                self.load_project_from_path(path);
            }
        }
    }

    /// Create a new project with the given info
    fn create_new_project(&mut self, info: NewProjectInfo) {
        // Create the project directory
        let project_dir = info.location.join(sanitize_project_name(&info.name));

        if let Err(e) = std::fs::create_dir_all(&project_dir) {
            self.set_status(format!("Failed to create project directory: {}", e));
            return;
        }

        // Create project metadata
        let mut meta = ProjectMeta::new(&info.name);
        if !info.description.is_empty() {
            meta = meta.with_description(&info.description);
        }

        // Create the project graph
        let project = ProjectGraph::new(meta);

        // Save the project file
        let project_file = project_dir.join(format!("{}.imortal", sanitize_project_name(&info.name)));

        match imortal_ir::save_project(&project, &project_file, imortal_ir::ProjectFormat::Json) {
            Ok(_) => {
                // Add to recent projects
                self.welcome_screen.add_recent_project(info.name.clone(), project_file.clone());

                // Switch to editor mode
                self.project = project;
                self.project_path = Some(project_file);
                self.state = EditorState::new();
                self.history = History::new();
                self.mode = AppMode::Editor;
                self.welcome_screen.close();

                self.set_status(format!("Created project: {}", info.name));
            }
            Err(e) => {
                self.set_status(format!("Failed to save project: {}", e));
            }
        }
    }

    /// Load a project from a file path
    fn load_project_from_path(&mut self, path: PathBuf) {
        match imortal_ir::load_project(&path) {
            Ok(project) => {
                // Add to recent projects
                self.welcome_screen.add_recent_project(project.meta.name.clone(), path.clone());

                // Switch to editor mode
                self.project = project;
                self.project_path = Some(path);
                self.state = EditorState::new();
                self.history = History::new();
                self.mode = AppMode::Editor;
                self.welcome_screen.close();

                self.set_status(format!("Opened project: {}", self.project.meta.name));
            }
            Err(e) => {
                self.set_status(format!("Failed to load project: {}", e));
            }
        }
    }

    /// Show the welcome screen (from menu: File -> Close Project)
    pub fn show_welcome_screen(&mut self) {
        self.mode = AppMode::Welcome;
        self.welcome_screen.open();
    }

    /// Save current state for undo
    fn save_undo_state(&mut self, action_name: &str) {
        self.history.push(action_name, self.project.clone());
    }

    /// Undo the last action
    fn undo(&mut self) {
        if let Some(previous_state) = self.history.undo(self.project.clone()) {
            self.project = previous_state;
            if let Some(action_name) = self.history.redo_action_name() {
                self.set_status(format!("Undid: {}", action_name));
            } else {
                self.set_status("Undo");
            }
        }
    }

    /// Redo the last undone action
    fn redo(&mut self) {
        if let Some(next_state) = self.history.redo(self.project.clone()) {
            self.project = next_state;
            if let Some(action_name) = self.history.undo_action_name() {
                self.set_status(format!("Redid: {}", action_name));
            } else {
                self.set_status("Redo");
            }
        }
    }

    /// Set a status message that will be displayed briefly
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some((message.into(), std::time::Instant::now()));
    }

    /// Clear the status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Render the menu bar
    fn render_menu_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // File menu
                ui.menu_button("File", |ui| {
                    if ui.button("New Project...").clicked() {
                        // Go to welcome screen in new project mode
                        self.mode = AppMode::Welcome;
                        self.welcome_screen.open();
                        ui.close_menu();
                    }
                    if ui.button("Open...").clicked() {
                        self.open_project();
                        ui.close_menu();
                    }

                    // Recent projects submenu
                    let recent_projects = self.welcome_screen.recent_projects.clone();
                    if !recent_projects.is_empty() {
                        ui.menu_button("Open Recent", |ui| {
                            for project in &recent_projects {
                                if ui.button(&project.name).clicked() {
                                    self.load_project_from_path(project.path.clone());
                                    ui.close_menu();
                                }
                            }
                            ui.separator();
                            if ui.button("Clear Recent Projects").clicked() {
                                self.welcome_screen.recent_projects.clear();
                                self.welcome_screen.save_recent_projects();
                                ui.close_menu();
                            }
                        });
                    }

                    ui.separator();
                    if ui.button("Save").clicked() {
                        self.save_project();
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        self.save_project_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Export...").clicked() {
                        // TODO: Export dialog
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Close Project").clicked() {
                        // Return to welcome screen
                        self.show_welcome_screen();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Edit menu
                ui.menu_button("Edit", |ui| {
                    let undo_text = if let Some(action) = self.history.undo_action_name() {
                        format!("Undo: {} (Ctrl+Z)", action)
                    } else {
                        "Undo (Ctrl+Z)".to_string()
                    };
                    if ui.add_enabled(self.history.can_undo(), egui::Button::new(undo_text)).clicked() {
                        self.undo();
                        ui.close_menu();
                    }

                    let redo_text = if let Some(action) = self.history.redo_action_name() {
                        format!("Redo: {} (Ctrl+Y)", action)
                    } else {
                        "Redo (Ctrl+Y)".to_string()
                    };
                    if ui.add_enabled(self.history.can_redo(), egui::Button::new(redo_text)).clicked() {
                        self.redo();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Cut").clicked() {
                        // TODO: Cut
                        ui.close_menu();
                    }
                    if ui.button("Copy").clicked() {
                        // TODO: Copy
                        ui.close_menu();
                    }
                    if ui.button("Paste").clicked() {
                        // TODO: Paste
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Select All").clicked() {
                        self.project.select_all();
                        ui.close_menu();
                    }
                    if ui.button("Deselect All").clicked() {
                        self.project.clear_selection();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Delete Selected (Del)").clicked() {
                        let count = self.project.selected_nodes.len() + self.project.selected_edges.len();
                        if count > 0 {
                            self.save_undo_state(&format!("Delete {} item(s)", count));
                            self.project.delete_selected();
                        }
                        ui.close_menu();
                    }
                });

                // View menu
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.config.show_grid, "Show Grid");
                    ui.checkbox(&mut self.config.snap_to_grid, "Snap to Grid");
                    ui.checkbox(&mut self.config.show_minimap, "Show Minimap");
                    ui.separator();
                    if ui.button("Zoom In").clicked() {
                        self.project.zoom(1.2);
                        ui.close_menu();
                    }
                    if ui.button("Zoom Out").clicked() {
                        self.project.zoom(0.8);
                        ui.close_menu();
                    }
                    if ui.button("Reset Zoom").clicked() {
                        self.project.set_zoom(1.0);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Fit to Content").clicked() {
                        // Get the available size from the UI
                        self.project.fit_to_content(800.0, 600.0);
                        ui.close_menu();
                    }
                });

                // Generate menu
                ui.menu_button("Generate", |ui| {
                    if ui.button("Generate Code...").clicked() {
                        self.generate_code();
                        ui.close_menu();
                    }
                    if ui.button("Preview Code").clicked() {
                        // TODO: Code preview
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Validate Project").clicked() {
                        self.validate_project();
                        ui.close_menu();
                    }
                });

                // Help menu
                ui.menu_button("Help", |ui| {
                    if ui.button("Documentation").clicked() {
                        // TODO: Open docs
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    /// Render the component palette (left panel)
    fn render_palette(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("palette")
            .resizable(true)
            .default_width(200.0)
            .min_width(150.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Components");
                ui.separator();

                // Search box
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.text_edit_singleline(&mut self.state.palette_search);
                });
                ui.separator();

                // Component categories
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for category in imortal_core::ComponentCategory::all() {
                        let components = self.registry.by_category(*category);
                        if components.is_empty() {
                            continue;
                        }

                        // Filter by search and collect info we need
                        let search = self.state.palette_search.to_lowercase();
                        let show_descriptions = self.config.show_descriptions;
                        let filtered: Vec<_> = components.into_iter()
                            .filter(|c| {
                                search.is_empty() ||
                                c.name.to_lowercase().contains(&search) ||
                                c.description.to_lowercase().contains(&search)
                            })
                            .map(|c| {
                                // Clone the info we need to avoid borrow issues
                                (c.id.clone(), c.icon.to_string(), c.name.clone(), c.description.clone(), c.instantiate_default())
                            })
                            .collect();

                        if filtered.is_empty() {
                            continue;
                        }

                        let header = format!("{} {}", category.icon(), category.display_name());
                        ui.collapsing(header, |ui| {
                            for (_id, icon, name, description, node_template) in &filtered {
                                let response = ui.add(
                                    egui::Button::new(format!("{} {}", icon, name))
                                        .min_size(egui::vec2(ui.available_width(), 0.0))
                                );

                                if response.clicked() {
                                    // Save state for undo, then add component
                                    self.save_undo_state(&format!("Add {}", name));
                                    self.project.add_node(node_template.clone());
                                    self.set_status(format!("Added {}", name));
                                }

                                if show_descriptions {
                                    response.on_hover_text(description);
                                }
                            }
                        });
                    }
                });
            });
    }

    /// Render the properties panel (right panel)
    fn render_properties(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("properties")
            .resizable(true)
            .default_width(280.0)
            .min_width(200.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                ui.heading("Properties");
                ui.separator();

                if self.project.selected_nodes.is_empty() {
                    ui.label("Select a component to view its properties");
                } else if self.project.selected_nodes.len() == 1 {
                    let node_id = *self.project.selected_nodes.iter().next().unwrap();
                    if let Some(node) = self.project.nodes.get(&node_id) {
                        self.render_node_properties(ui, node.clone());
                    }
                } else {
                    ui.label(format!("{} components selected", self.project.selected_nodes.len()));
                }
            });
    }

    /// Render properties for a single node
    fn render_node_properties(&mut self, ui: &mut egui::Ui, node: Node) {
        // Node name
        ui.horizontal(|ui| {
            ui.label("Name:");
            let mut name = node.name.clone();
            if ui.text_edit_singleline(&mut name).changed() {
                if let Some(n) = self.project.get_node_mut(node.id) {
                    n.name = name;
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("Type:");
            ui.label(&node.component_type);
        });

        ui.separator();

        // Delete button
        let node_id_to_delete = node.id;
        let node_name = node.name.clone();
        if ui.button("🗑 Delete Component").clicked() {
            self.save_undo_state(&format!("Delete {}", node_name));
            self.project.remove_node(node_id_to_delete);
            self.set_status("Component deleted");
            return; // Exit early since node no longer exists
        }

        ui.separator();

        // Fields section
        let node_id = node.id;
        let is_entity = node.component_type == "data.entity";

        egui::CollapsingHeader::new("Fields")
            .default_open(true)
            .show(ui, |ui| {
                // List existing fields with edit/delete options
                let mut field_to_delete: Option<uuid::Uuid> = None;
                let mut field_updates: Vec<(uuid::Uuid, String, DataType, bool)> = Vec::new();

                for field in &node.fields {
                    ui.horizontal(|ui| {
                        // Field name (editable for non-system fields)
                        let is_system_field = field.name == "id" || field.name == "created_at" || field.name == "updated_at";

                        if is_system_field {
                            ui.label(format!("🔒 {}", field.display_label()));
                        } else {
                            ui.label(&field.name);
                        }

                        // Data type display
                        ui.label(format!("{}", Self::data_type_display(&field.data_type)));

                        // Required indicator
                        if field.required {
                            ui.label("*");
                        }

                        // Delete button (not for system fields)
                        if !is_system_field && is_entity {
                            if ui.small_button("🗑").on_hover_text("Delete field").clicked() {
                                field_to_delete = Some(field.id);
                            }
                        }
                    });
                }

                // Apply field deletion
                if let Some(field_id) = field_to_delete {
                    self.save_undo_state("Delete field");
                    if let Some(n) = self.project.get_node_mut(node_id) {
                        n.fields.retain(|f| f.id != field_id);
                        self.set_status("Field deleted");
                    }
                }

                // Add new field section (only for entities)
                if is_entity {
                    ui.separator();
                    ui.label("Add New Field:");

                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_field_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        egui::ComboBox::from_id_salt("field_type")
                            .selected_text(Self::data_type_options()[self.new_field_type])
                            .show_ui(ui, |ui| {
                                for (idx, type_name) in Self::data_type_options().iter().enumerate() {
                                    ui.selectable_value(&mut self.new_field_type, idx, *type_name);
                                }
                            });
                    });

                    if ui.button("➕ Add Field").clicked() && !self.new_field_name.is_empty() {
                        let data_type = Self::index_to_data_type(self.new_field_type);
                        let new_field = Field::new(self.new_field_name.clone(), data_type);

                        self.save_undo_state(&format!("Add field: {}", self.new_field_name));
                        if let Some(n) = self.project.get_node_mut(node_id) {
                            n.fields.push(new_field);
                            self.set_status(format!("Added field: {}", self.new_field_name));
                        }
                        self.new_field_name.clear();
                        self.new_field_type = 0;
                    }
                }
            });

        // Ports section
        ui.collapsing("Ports", |ui| {
            if !node.ports.inputs.is_empty() {
                ui.label("Inputs:");
                for port in &node.ports.inputs {
                    ui.horizontal(|ui| {
                        ui.label(format!("  {} →", port.name));
                    });
                }
            }
            if !node.ports.outputs.is_empty() {
                ui.label("Outputs:");
                for port in &node.ports.outputs {
                    ui.horizontal(|ui| {
                        ui.label(format!("  → {}", port.name));
                    });
                }
            }
        });

        // Ensure database components have all required config fields
        if node.component_type == "storage.database" {
            let required_fields = [
                ("database", imortal_core::ConfigValue::String(String::new())),
                ("username", imortal_core::ConfigValue::String(String::new())),
            ];
            for (field, default_value) in required_fields {
                if !node.config.contains_key(field) {
                    if let Some(n) = self.project.get_node_mut(node_id) {
                        n.config.insert(field.to_string(), default_value);
                    }
                }
            }
        }

        // Configuration section - editable
        if !node.config.is_empty() {
            egui::CollapsingHeader::new("Configuration")
                .default_open(true)
                .show(ui, |ui| {
                    let mut config_updates: Vec<(String, imortal_core::ConfigValue)> = Vec::new();

                    // Sort config keys for consistent display, with important ones first
                    let mut config_clone: Vec<(String, imortal_core::ConfigValue)> =
                        node.config.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

                    // Custom sort: backend first, then connection settings, then others
                    config_clone.sort_by(|(a, _), (b, _)| {
                        let priority = |k: &str| -> i32 {
                            match k {
                                "backend" => 0,
                                "host" => 1,
                                "port" => 2,
                                "database" => 3,
                                "username" => 4,
                                "password" => 5,
                                "connection_string" => 6,
                                _ => 100,
                            }
                        };
                        priority(a).cmp(&priority(b))
                    });

                    // Database backends for dropdown
                    let db_backends = ["postgres", "mysql", "sqlite", "mssql", "mongodb"];

                    // Cache backends for dropdown
                    let cache_backends = ["memory", "redis", "memcached"];

                    for (key, value) in config_clone {
                        ui.horizontal(|ui| {
                            // Format key name nicely
                            let display_key = key.replace('_', " ");
                            let display_key = display_key
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars = word.chars();
                                    match chars.next() {
                                        None => String::new(),
                                        Some(c) => c.to_uppercase().chain(chars).collect(),
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(" ");

                            ui.label(format!("{}:", display_key));

                            match value.clone() {
                                imortal_core::ConfigValue::String(mut s) => {
                                    // Special handling for 'backend' - show dropdown
                                    if key == "backend" {
                                        let backends = if node.component_type.contains("cache") {
                                            &cache_backends[..]
                                        } else {
                                            &db_backends[..]
                                        };

                                        egui::ComboBox::from_id_salt(format!("config_{}", key))
                                            .selected_text(&s)
                                            .show_ui(ui, |ui| {
                                                for backend in backends {
                                                    if ui.selectable_value(&mut s, backend.to_string(), *backend).changed() {
                                                        config_updates.push((key.clone(), imortal_core::ConfigValue::String(s.clone())));
                                                    }
                                                }
                                            });
                                    } else {
                                        // Check if it's a sensitive field
                                        let is_sensitive = key.contains("password") || key.contains("secret");

                                        if is_sensitive {
                                            if ui.add(egui::TextEdit::singleline(&mut s).password(true).desired_width(150.0)).changed() {
                                                config_updates.push((key.clone(), imortal_core::ConfigValue::String(s)));
                                            }
                                        } else {
                                            if ui.add(egui::TextEdit::singleline(&mut s).desired_width(150.0)).changed() {
                                                config_updates.push((key.clone(), imortal_core::ConfigValue::String(s)));
                                            }
                                        }
                                    }
                                }
                                imortal_core::ConfigValue::Int(mut i) => {
                                    if ui.add(egui::DragValue::new(&mut i)).changed() {
                                        config_updates.push((key.clone(), imortal_core::ConfigValue::Int(i)));
                                    }
                                }
                                imortal_core::ConfigValue::Float(mut f) => {
                                    if ui.add(egui::DragValue::new(&mut f).speed(0.1)).changed() {
                                        config_updates.push((key.clone(), imortal_core::ConfigValue::Float(f)));
                                    }
                                }
                                imortal_core::ConfigValue::Bool(mut b) => {
                                    if ui.checkbox(&mut b, "").changed() {
                                        config_updates.push((key.clone(), imortal_core::ConfigValue::Bool(b)));
                                    }
                                }
                                _ => {
                                    ui.label(format!("{:?}", value));
                                }
                            }
                        });
                    }

                    // Apply config updates
                    if !config_updates.is_empty() {
                        if let Some(n) = self.project.get_node_mut(node_id) {
                            for (key, value) in config_updates {
                                n.config.insert(key, value);
                            }
                        }
                    }

                    // Test Connection button for database components
                    if node.component_type == "storage.database" {
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            if ui.button("🔌 Test Connection").clicked() {
                                let result = Self::test_database_connection(&node.config);
                                match result {
                                    Ok(msg) => {
                                        self.db_connection_result = Some((node_id, true, msg, std::time::Instant::now()));
                                    }
                                    Err(err) => {
                                        self.db_connection_result = Some((node_id, false, err, std::time::Instant::now()));
                                    }
                                }
                            }
                        });

                        // Display connection result with colors (auto-hide after 5 seconds)
                        let mut should_clear_result = false;
                        if let Some((result_node_id, success, message, timestamp)) = &self.db_connection_result {
                            if *result_node_id == node_id {
                                // Check if 5 seconds have passed
                                if timestamp.elapsed().as_secs() >= 5 {
                                    should_clear_result = true;
                                } else {
                                ui.add_space(8.0);

                                let (bg_color, text_color, icon) = if *success {
                                    (
                                        egui::Color32::from_rgb(30, 70, 30),
                                        egui::Color32::from_rgb(100, 255, 100),
                                        "✅"
                                    )
                                } else {
                                    (
                                        egui::Color32::from_rgb(70, 30, 30),
                                        egui::Color32::from_rgb(255, 100, 100),
                                        "❌"
                                    )
                                };

                                egui::Frame::none()
                                    .fill(bg_color)
                                    .inner_margin(egui::Margin::same(8.0))
                                    .rounding(egui::Rounding::same(4.0))
                                    .show(ui, |ui| {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(egui::RichText::new(icon).size(16.0));
                                            ui.add_space(4.0);
                                            ui.label(
                                                egui::RichText::new(if *success { "Connected!" } else { "Connection Failed" })
                                                    .color(text_color)
                                                    .strong()
                                            );
                                        });
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new(message)
                                                .color(if *success {
                                                    egui::Color32::from_rgb(180, 255, 180)
                                                } else {
                                                    egui::Color32::from_rgb(255, 180, 180)
                                                })
                                                .size(12.0)
                                        );
                                    });

                                    // Request repaint to update the timer
                                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(100));
                                }
                            }
                        }

                        // Clear the result after timeout
                        if should_clear_result {
                            self.db_connection_result = None;
                        }
                    }
                });
        }
    }

    /// Test database connection using the provided config
    fn test_database_connection(config: &std::collections::HashMap<String, imortal_core::ConfigValue>) -> Result<String, String> {
        // Extract connection parameters
        let backend = config.get("backend")
            .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.as_str()) } else { None })
            .unwrap_or("postgres");

        let host = config.get("host")
            .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_else(|| "localhost".to_string());

        let port = config.get("port")
            .and_then(|v| if let imortal_core::ConfigValue::Int(i) = v { Some(*i as u16) } else { None })
            .unwrap_or(5432);

        let database = config.get("database")
            .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        let username = config.get("username")
            .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        let password = config.get("password")
            .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        // Validate required fields
        if database.is_empty() {
            return Err("Database name is required".to_string());
        }
        if username.is_empty() {
            return Err("Username is required".to_string());
        }

        match backend {
            "postgres" => {
                // Build connection string
                let conn_string = format!(
                    "host={} port={} dbname={} user={} password={}",
                    host, port, database, username, password
                );

                // Try to connect
                match postgres::Client::connect(&conn_string, postgres::NoTls) {
                    Ok(mut client) => {
                        // Try a simple query to verify connection
                        match client.query_one("SELECT version()", &[]) {
                            Ok(row) => {
                                let version: String = row.get(0);
                                // Extract just the first part of the version string
                                let short_version = version.split(',').next().unwrap_or(&version);
                                Ok(format!("Connected to {}", short_version))
                            }
                            Err(e) => Err(format!("Query failed: {}", e))
                        }
                    }
                    Err(e) => Err(format!("{}", e))
                }
            }
            "mysql" => Err("MySQL connection test not yet implemented".to_string()),
            "sqlite" => Err("SQLite connection test not yet implemented".to_string()),
            "mongodb" => Err("MongoDB connection test not yet implemented".to_string()),
            _ => Err(format!("Unsupported backend: {}", backend))
        }
    }

    /// Get display string for a data type
    fn data_type_display(dt: &DataType) -> &'static str {
        match dt {
            DataType::String => "String",
            DataType::Text => "Text",
            DataType::Int32 => "Integer",
            DataType::Int64 => "BigInt",
            DataType::Float32 => "Float",
            DataType::Float64 => "Double",
            DataType::Bool => "Boolean",
            DataType::Uuid => "Uuid",
            DataType::DateTime => "DateTime",
            DataType::Date => "Date",
            DataType::Time => "Time",
            DataType::Bytes => "Bytes",
            DataType::Json => "JSON",
            DataType::Optional(_) => "Optional",
            DataType::Array(_) => "Array",
            DataType::Entity(_) => "Entity",
            DataType::Reference(_) => "Reference",
            _ => "Unknown",
        }
    }

    /// Get list of available data types for the dropdown
    fn data_type_options() -> &'static [&'static str] {
        &[
            "String",
            "Text",
            "Integer",
            "BigInt",
            "Float",
            "Double",
            "Boolean",
            "DateTime",
            "Date",
            "JSON",
        ]
    }

    /// Convert dropdown index to DataType
    fn index_to_data_type(idx: usize) -> DataType {
        match idx {
            0 => DataType::String,
            1 => DataType::Text,
            2 => DataType::Int32,
            3 => DataType::Int64,
            4 => DataType::Float32,
            5 => DataType::Float64,
            6 => DataType::Bool,
            7 => DataType::DateTime,
            8 => DataType::Date,
            9 => DataType::Json,
            _ => DataType::String,
        }
    }

    /// Get the number of content items to display for a node (for height calculation)
    fn get_node_content_items(node: &Node) -> usize {
        match node.component_type.as_str() {
            "data.entity" => node.fields.len(),
            "storage.database" => 5,  // backend, host:port, database, username, ssl
            "api.rest" => 3,          // method, path, auth
            _ => 0,
        }
    }

    /// Render the main canvas
    fn render_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Canvas background
            let (rect, response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::click_and_drag()
            );

            let painter = ui.painter_at(rect);

            // Draw background
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 30, 35));

            // Draw grid if enabled
            if self.config.show_grid {
                self.draw_grid(&painter, rect);
            }

            let zoom = self.project.viewport.zoom;
            let pan = egui::vec2(self.project.viewport.pan_x, self.project.viewport.pan_y);

            // Track mouse position for connection drawing
            if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                self.connection_mouse_pos = pos;
            }

            // Handle canvas interactions - panning with drag
            if response.dragged_by(egui::PointerButton::Middle) ||
               (response.dragged() && ui.input(|i| i.modifiers.shift)) {
                let delta = response.drag_delta();
                self.project.pan(delta.x, delta.y);
            }

            // Check for port interactions
            let mut clicked_port: Option<(NodeId, String, bool)> = None; // (node_id, port_name, is_output)
            let mut hovered_port: Option<(NodeId, String, bool)> = None;
            let mut port_was_clicked = false;

            // Get mouse state
            let mouse_clicked = ui.input(|i| i.pointer.primary_clicked());
            let pointer_pos = ui.input(|i| i.pointer.interact_pos()).or_else(|| ui.input(|i| i.pointer.hover_pos()));

            if let Some(pointer_pos) = pointer_pos {
                // Check all nodes for port hover/click
                for node in self.project.nodes.values() {
                    let node_screen_pos = rect.min + pan + egui::vec2(node.position.x * zoom, node.position.y * zoom);
                    let node_width = node.size.width * zoom;

                    // Calculate actual node height (same logic as draw_node)
                    let header_height = 25.0 * zoom;
                    let field_height_val = 18.0 * zoom;
                    let content_items = Self::get_node_content_items(node);

                    let node_height = if content_items > 0 {
                        header_height + (content_items as f32 * field_height_val) + (8.0 * zoom)
                    } else {
                        node.size.height * zoom
                    };

                    // Output port (right side)
                    let output_port_pos = egui::pos2(
                        node_screen_pos.x + node_width + 8.0,
                        node_screen_pos.y + node_height / 2.0
                    );
                    let dist_to_output = pointer_pos.distance(output_port_pos);
                    if dist_to_output < 15.0 {
                        hovered_port = Some((node.id, "output".to_string(), true));
                        if mouse_clicked {
                            clicked_port = Some((node.id, "output".to_string(), true));
                        }
                    }

                    // Input port (left side)
                    let input_port_pos = egui::pos2(
                        node_screen_pos.x - 8.0,
                        node_screen_pos.y + node_height / 2.0
                    );
                    let dist_to_input = pointer_pos.distance(input_port_pos);
                    if dist_to_input < 15.0 {
                        hovered_port = Some((node.id, "input".to_string(), false));
                        if mouse_clicked {
                            clicked_port = Some((node.id, "input".to_string(), false));
                        }
                    }
                }
            }

            // Handle port clicks for connection drawing
            if let Some((node_id, port_name, _is_output)) = clicked_port {
                port_was_clicked = true;
                if self.drawing_connection {
                    // Complete the connection
                    if let Some(from_node_id) = self.connection_from_node {
                        if from_node_id != node_id {
                            // Determine source and target based on which port was clicked first
                            let (source_id, target_id) = if self.connection_from_port == "output" {
                                (from_node_id, node_id)
                            } else {
                                (node_id, from_node_id)
                            };

                            // Get actual port names from the nodes
                            let from_port = self.project.get_node(source_id)
                                .and_then(|n| n.ports.outputs.first())
                                .map(|p| p.id.clone())
                                .unwrap_or_else(|| "out".to_string());

                            let to_port = self.project.get_node(target_id)
                                .and_then(|n| n.ports.inputs.first())
                                .map(|p| p.id.clone())
                                .unwrap_or_else(|| "in".to_string());

                            // Create dependency edge (skips port validation)
                            self.save_undo_state("Create connection");
                            let edge = Edge::dependency(source_id, target_id);
                            match self.project.add_edge(edge) {
                                Ok(_) => self.set_status("Connection created"),
                                Err(e) => self.set_status(format!("Failed: {}", e)),
                            }
                        }
                    }
                    self.drawing_connection = false;
                    self.connection_from_node = None;
                    self.connection_from_port.clear();
                } else {
                    // Start drawing connection
                    self.drawing_connection = true;
                    self.connection_from_node = Some(node_id);
                    self.connection_from_port = port_name;
                    self.set_status("Click another port to connect, or press Escape to cancel");
                }
            }

            // Cancel connection drawing with Escape or right-click
            if self.drawing_connection {
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) || response.secondary_clicked() {
                    self.drawing_connection = false;
                    self.connection_from_node = None;
                    self.connection_from_port.clear();
                    self.set_status("Connection cancelled");
                }
            }

            // Handle Delete/Backspace to delete selected nodes and edges
            let delete_pressed = ui.input(|i| {
                i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)
            });
            if delete_pressed && !self.drawing_connection {
                let selected_count = self.project.selected_nodes.len() + self.project.selected_edges.len();
                if selected_count > 0 {
                    self.save_undo_state(&format!("Delete {} item(s)", selected_count));
                    self.project.delete_selected();
                    self.set_status(format!("Deleted {} item(s)", selected_count));
                }
            }

            // Handle Undo/Redo keyboard shortcuts
            let ctrl_held = ui.input(|i| i.modifiers.ctrl || i.modifiers.mac_cmd);
            let shift_held = ui.input(|i| i.modifiers.shift);

            if ctrl_held && ui.input(|i| i.key_pressed(egui::Key::Z)) {
                if shift_held {
                    self.redo();
                } else {
                    self.undo();
                }
            }
            if ctrl_held && ui.input(|i| i.key_pressed(egui::Key::Y)) {
                self.redo();
            }

            // Handle node selection on click (only if not clicking a port)
            if mouse_clicked && !port_was_clicked && !self.drawing_connection {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    // Check if click is on any node
                    let mut clicked_node_id = None;
                    for node in self.project.nodes.values() {
                        let node_rect = egui::Rect::from_min_size(
                            rect.min + pan + egui::vec2(node.position.x * zoom, node.position.y * zoom),
                            egui::vec2(node.size.width * zoom, node.size.height * zoom)
                        );

                        if node_rect.contains(pointer_pos) {
                            clicked_node_id = Some(node.id);
                            break;
                        }
                    }

                    // Update selection
                    let shift_held = ui.input(|i| i.modifiers.shift);
                    if let Some(node_id) = clicked_node_id {
                        if shift_held {
                            // Toggle selection with shift
                            if self.project.selected_nodes.contains(&node_id) {
                                self.project.deselect_node(node_id);
                            } else {
                                self.project.select_node(node_id);
                            }
                        } else {
                            // Single select
                            self.project.clear_selection();
                            self.project.select_node(node_id);
                        }
                    } else {
                        // Clicked on empty canvas - deselect all
                        if !shift_held {
                            self.project.clear_selection();
                        }
                    }
                }
            }

            // Handle node dragging (only if not drawing connection)
            if !self.drawing_connection && response.dragged_by(egui::PointerButton::Primary) && !ui.input(|i| i.modifiers.shift) {
                let delta = response.drag_delta();

                // Move selected nodes
                if !self.project.selected_nodes.is_empty() {
                    let selected: Vec<_> = self.project.selected_nodes.iter().cloned().collect();
                    for node_id in selected {
                        if let Some(node) = self.project.get_node_mut(node_id) {
                            node.position.x += delta.x / zoom;
                            node.position.y += delta.y / zoom;
                        }
                    }
                }
            }

            // Draw edges first (below nodes)
            for edge in self.project.edges.values() {
                self.draw_edge(&painter, rect, edge);
            }

            // Draw nodes with ports
            for node in self.project.nodes.values() {
                let is_selected = self.project.selected_nodes.contains(&node.id);
                self.draw_node(&painter, rect, node, is_selected);

                // Draw ports on node
                let node_screen_pos = rect.min + pan + egui::vec2(node.position.x * zoom, node.position.y * zoom);
                let node_width = node.size.width * zoom;

                // Calculate actual node height (same logic as draw_node)
                let header_height = 25.0 * zoom;
                let field_height = 18.0 * zoom;
                let content_items = Self::get_node_content_items(node);

                let node_height = if content_items > 0 {
                    header_height + (content_items as f32 * field_height) + (8.0 * zoom)
                } else {
                    node.size.height * zoom
                };

                // Output port (right side) - green circle
                let output_port_pos = egui::pos2(
                    node_screen_pos.x + node_width + 8.0,
                    node_screen_pos.y + node_height / 2.0
                );
                let output_hovered = hovered_port.as_ref().map_or(false, |(nid, _, is_out)| *nid == node.id && *is_out);
                let output_color = if output_hovered {
                    egui::Color32::from_rgb(100, 255, 100)
                } else {
                    egui::Color32::from_rgb(80, 200, 80)
                };
                painter.circle_filled(output_port_pos, if output_hovered { 8.0 } else { 6.0 }, output_color);
                painter.circle_stroke(output_port_pos, if output_hovered { 8.0 } else { 6.0 }, egui::Stroke::new(1.0, egui::Color32::WHITE));

                // Input port (left side) - blue circle
                let input_port_pos = egui::pos2(
                    node_screen_pos.x - 8.0,
                    node_screen_pos.y + node_height / 2.0
                );
                let input_hovered = hovered_port.as_ref().map_or(false, |(nid, _, is_out)| *nid == node.id && !*is_out);
                let input_color = if input_hovered {
                    egui::Color32::from_rgb(100, 150, 255)
                } else {
                    egui::Color32::from_rgb(80, 120, 200)
                };
                painter.circle_filled(input_port_pos, if input_hovered { 8.0 } else { 6.0 }, input_color);
                painter.circle_stroke(input_port_pos, if input_hovered { 8.0 } else { 6.0 }, egui::Stroke::new(1.0, egui::Color32::WHITE));
            }

            // Draw connection being drawn
            if self.drawing_connection {
                if let Some(from_node_id) = self.connection_from_node {
                    if let Some(from_node) = self.project.get_node(from_node_id) {
                        let from_screen_pos = rect.min + pan + egui::vec2(from_node.position.x * zoom, from_node.position.y * zoom);
                        let from_width = from_node.size.width * zoom;
                        let from_height = from_node.size.height * zoom;

                        let start_pos = if self.connection_from_port == "output" {
                            egui::pos2(from_screen_pos.x + from_width + 8.0, from_screen_pos.y + from_height / 2.0)
                        } else {
                            egui::pos2(from_screen_pos.x - 8.0, from_screen_pos.y + from_height / 2.0)
                        };

                        // Draw line to mouse
                        painter.line_segment(
                            [start_pos, self.connection_mouse_pos],
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 100))
                        );

                        // Draw circle at mouse position
                        painter.circle_filled(self.connection_mouse_pos, 5.0, egui::Color32::from_rgb(255, 200, 100));
                    }
                }
            }

            // Draw groups
            for group in self.project.groups.values() {
                self.draw_group(&painter, rect, group);
            }

            // Status bar info
            ui.put(
                egui::Rect::from_min_size(
                    rect.min + egui::vec2(10.0, rect.height() - 25.0),
                    egui::vec2(200.0, 20.0)
                ),
                egui::Label::new(format!(
                    "Zoom: {:.0}% | Nodes: {} | Edges: {}",
                    self.project.viewport.zoom * 100.0,
                    self.project.node_count(),
                    self.project.edge_count()
                ))
            );
        });
    }

    /// Draw the background grid
    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let grid_size = self.config.grid_size * self.project.viewport.zoom;
        let color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20);

        let start_x = (rect.min.x / grid_size).floor() * grid_size;
        let start_y = (rect.min.y / grid_size).floor() * grid_size;

        let mut x = start_x;
        while x < rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, color)
            );
            x += grid_size;
        }

        let mut y = start_y;
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, color)
            );
            y += grid_size;
        }
    }

    /// Draw a node on the canvas
    fn draw_node(&self, painter: &egui::Painter, canvas_rect: egui::Rect, node: &Node, is_selected: bool) {
        let zoom = self.project.viewport.zoom;
        let pan = egui::vec2(self.project.viewport.pan_x, self.project.viewport.pan_y);

        // Calculate node height based on content
        let header_height = 25.0 * zoom;
        let field_height = 18.0 * zoom;
        let content_items = Self::get_node_content_items(node);

        let calculated_height = if content_items > 0 {
            header_height + (content_items as f32 * field_height) + (8.0 * zoom)
        } else {
            node.size.height * zoom
        };

        let node_rect = egui::Rect::from_min_size(
            canvas_rect.min + pan + egui::vec2(node.position.x * zoom, node.position.y * zoom),
            egui::vec2(node.size.width * zoom, calculated_height)
        );

        // Node background
        let bg_color = if is_selected {
            egui::Color32::from_rgb(60, 80, 120)
        } else {
            egui::Color32::from_rgb(50, 50, 55)
        };

        painter.rect_filled(node_rect, 5.0, bg_color);

        // Node border
        let border_color = if is_selected {
            egui::Color32::from_rgb(100, 150, 255)
        } else {
            egui::Color32::from_rgb(80, 80, 85)
        };
        painter.rect_stroke(node_rect, 5.0, egui::Stroke::new(2.0, border_color));

        // Node header
        let header_rect = egui::Rect::from_min_size(
            node_rect.min,
            egui::vec2(node_rect.width(), header_height)
        );

        let header_color = match node.category {
            imortal_core::ComponentCategory::Auth => egui::Color32::from_rgb(200, 80, 80),
            imortal_core::ComponentCategory::Data => egui::Color32::from_rgb(80, 150, 200),
            imortal_core::ComponentCategory::Api => egui::Color32::from_rgb(150, 200, 80),
            imortal_core::ComponentCategory::Storage => egui::Color32::from_rgb(200, 150, 80),
            imortal_core::ComponentCategory::Logic => egui::Color32::from_rgb(200, 80, 200),
            _ => egui::Color32::from_rgb(100, 100, 100),
        };

        painter.rect_filled(
            egui::Rect::from_min_size(header_rect.min, egui::vec2(header_rect.width(), header_rect.height())),
            egui::Rounding { nw: 5.0, ne: 5.0, sw: 0.0, se: 0.0 },
            header_color
        );

        // Node title
        let icon = node.icon.as_deref().unwrap_or("📦");
        painter.text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{} {}", icon, node.name),
            egui::FontId::proportional(14.0 * zoom),
            egui::Color32::WHITE
        );

        // Draw fields for entity nodes
        let is_entity = node.component_type == "data.entity";
        let is_database = node.component_type == "storage.database";
        let is_rest_endpoint = node.component_type == "api.rest";

        if is_entity && !node.fields.is_empty() {
            let field_start_y = header_rect.max.y + (4.0 * zoom);
            let text_color = egui::Color32::from_rgb(200, 200, 200);
            let type_color = egui::Color32::from_rgb(150, 150, 150);
            let pk_color = egui::Color32::from_rgb(255, 200, 100);

            for (idx, field) in node.fields.iter().enumerate() {
                let field_y = field_start_y + (idx as f32 * field_height);

                // Field name
                let field_name = if field.is_primary_key() {
                    format!("🔑 {}", field.name)
                } else {
                    field.name.clone()
                };

                let name_color = if field.is_primary_key() { pk_color } else { text_color };

                painter.text(
                    egui::pos2(node_rect.min.x + (8.0 * zoom), field_y),
                    egui::Align2::LEFT_TOP,
                    &field_name,
                    egui::FontId::proportional(12.0 * zoom),
                    name_color
                );

                // Field type (right-aligned)
                let type_str = Self::data_type_display(&field.data_type);
                painter.text(
                    egui::pos2(node_rect.max.x - (8.0 * zoom), field_y),
                    egui::Align2::RIGHT_TOP,
                    type_str,
                    egui::FontId::proportional(10.0 * zoom),
                    type_color
                );
            }
        }

        // Draw database connection details for storage.database nodes
        if is_database {
            let field_start_y = header_rect.max.y + (4.0 * zoom);
            let label_color = egui::Color32::from_rgb(180, 180, 180);
            let value_color = egui::Color32::from_rgb(220, 220, 220);
            let highlight_color = egui::Color32::from_rgb(255, 200, 100);

            // Get config values
            let backend = node.config.get("backend")
                .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.as_str()) } else { None })
                .unwrap_or("postgres");
            let host = node.config.get("host")
                .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.as_str()) } else { None })
                .unwrap_or("localhost");
            let port = node.config.get("port")
                .and_then(|v| if let imortal_core::ConfigValue::Int(i) = v { Some(*i) } else { None })
                .unwrap_or(5432);
            let database = node.config.get("database")
                .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "—".to_string());
            let ssl = node.config.get("ssl")
                .and_then(|v| if let imortal_core::ConfigValue::Bool(b) = v { Some(*b) } else { None })
                .unwrap_or(true);

            // Backend icon based on type
            let backend_icon = match backend {
                "postgres" => "🐘",
                "mysql" => "🐬",
                "sqlite" => "📁",
                "mongodb" => "🍃",
                "mssql" => "🪟",
                _ => "💾",
            };

            // Row 1: Backend type
            let row_y = field_start_y;
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                format!("{} {}", backend_icon, backend.to_uppercase()),
                egui::FontId::proportional(12.0 * zoom),
                highlight_color
            );

            // Row 2: Host and port
            let row_y = field_start_y + field_height;
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                "Host:",
                egui::FontId::proportional(11.0 * zoom),
                label_color
            );
            painter.text(
                egui::pos2(node_rect.max.x - (8.0 * zoom), row_y),
                egui::Align2::RIGHT_TOP,
                format!("{}:{}", host, port),
                egui::FontId::proportional(11.0 * zoom),
                value_color
            );

            // Row 3: Database name
            let row_y = field_start_y + (field_height * 2.0);
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                "Database:",
                egui::FontId::proportional(11.0 * zoom),
                label_color
            );
            painter.text(
                egui::pos2(node_rect.max.x - (8.0 * zoom), row_y),
                egui::Align2::RIGHT_TOP,
                &database,
                egui::FontId::proportional(11.0 * zoom),
                value_color
            );

            // Row 4: Username
            let username = node.config.get("username")
                .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "—".to_string());
            let row_y = field_start_y + (field_height * 3.0);
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                "User:",
                egui::FontId::proportional(11.0 * zoom),
                label_color
            );
            painter.text(
                egui::pos2(node_rect.max.x - (8.0 * zoom), row_y),
                egui::Align2::RIGHT_TOP,
                &username,
                egui::FontId::proportional(11.0 * zoom),
                value_color
            );

            // Row 5: SSL status
            let row_y = field_start_y + (field_height * 4.0);
            let ssl_icon = if ssl { "🔒" } else { "🔓" };
            let ssl_text = if ssl { "SSL Enabled" } else { "SSL Disabled" };
            let ssl_color = if ssl { egui::Color32::from_rgb(100, 200, 100) } else { egui::Color32::from_rgb(200, 150, 100) };
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                format!("{} {}", ssl_icon, ssl_text),
                egui::FontId::proportional(11.0 * zoom),
                ssl_color
            );
        }

        // Draw REST endpoint details for api.rest nodes
        if is_rest_endpoint {
            let field_start_y = header_rect.max.y + (4.0 * zoom);
            let label_color = egui::Color32::from_rgb(180, 180, 180);
            let value_color = egui::Color32::from_rgb(220, 220, 220);

            // Get config values
            let method = node.config.get("method")
                .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.as_str()) } else { None })
                .unwrap_or("GET");
            let path = node.config.get("path")
                .and_then(|v| if let imortal_core::ConfigValue::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "/api".to_string());
            let auth_required = node.config.get("auth_required")
                .and_then(|v| if let imortal_core::ConfigValue::Bool(b) = v { Some(*b) } else { None })
                .unwrap_or(false);

            // Method color based on HTTP method
            let method_color = match method {
                "GET" => egui::Color32::from_rgb(100, 200, 100),
                "POST" => egui::Color32::from_rgb(100, 150, 255),
                "PUT" => egui::Color32::from_rgb(255, 200, 100),
                "DELETE" => egui::Color32::from_rgb(255, 100, 100),
                "PATCH" => egui::Color32::from_rgb(200, 150, 255),
                _ => egui::Color32::from_rgb(180, 180, 180),
            };

            // Row 1: HTTP Method
            let row_y = field_start_y;
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                method,
                egui::FontId::proportional(12.0 * zoom),
                method_color
            );

            // Row 2: Path
            let row_y = field_start_y + field_height;
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                "Path:",
                egui::FontId::proportional(11.0 * zoom),
                label_color
            );
            painter.text(
                egui::pos2(node_rect.max.x - (8.0 * zoom), row_y),
                egui::Align2::RIGHT_TOP,
                &path,
                egui::FontId::proportional(11.0 * zoom),
                value_color
            );

            // Row 3: Auth status
            let row_y = field_start_y + (field_height * 2.0);
            let auth_icon = if auth_required { "🔐" } else { "🔓" };
            let auth_text = if auth_required { "Auth Required" } else { "Public" };
            let auth_color = if auth_required { egui::Color32::from_rgb(255, 200, 100) } else { egui::Color32::from_rgb(150, 150, 150) };
            painter.text(
                egui::pos2(node_rect.min.x + (8.0 * zoom), row_y),
                egui::Align2::LEFT_TOP,
                format!("{} {}", auth_icon, auth_text),
                egui::FontId::proportional(11.0 * zoom),
                auth_color
            );
        }
    }

    /// Calculate the actual rendered height of a node (accounting for content)
    fn calculate_node_height(&self, node: &Node, zoom: f32) -> f32 {
        let header_height = 25.0 * zoom;
        let field_height = 18.0 * zoom;
        let content_items = Self::get_node_content_items(node);

        if content_items > 0 {
            header_height + (content_items as f32 * field_height) + (8.0 * zoom)
        } else {
            node.size.height * zoom
        }
    }

    /// Draw an edge on the canvas
    fn draw_edge(&self, painter: &egui::Painter, canvas_rect: egui::Rect, edge: &Edge) {
        let zoom = self.project.viewport.zoom;
        let pan = egui::vec2(self.project.viewport.pan_x, self.project.viewport.pan_y);

        // Get source and target nodes
        let from_node = match self.project.get_node(edge.from_node) {
            Some(n) => n,
            None => return,
        };
        let to_node = match self.project.get_node(edge.to_node) {
            Some(n) => n,
            None => return,
        };

        // Calculate actual node heights (accounting for entity fields)
        let from_height = self.calculate_node_height(from_node, zoom);
        let to_height = self.calculate_node_height(to_node, zoom);

        // Calculate screen positions of nodes
        let from_screen_pos = canvas_rect.min + pan + egui::vec2(from_node.position.x * zoom, from_node.position.y * zoom);
        let to_screen_pos = canvas_rect.min + pan + egui::vec2(to_node.position.x * zoom, to_node.position.y * zoom);

        let from_width = from_node.size.width * zoom;

        // Output port position (right side of from_node, +8 pixels outside)
        let start = egui::pos2(
            from_screen_pos.x + from_width + 8.0,
            from_screen_pos.y + from_height / 2.0
        );

        // Input port position (left side of to_node, -8 pixels outside)
        let end = egui::pos2(
            to_screen_pos.x - 8.0,
            to_screen_pos.y + to_height / 2.0
        );

        // Draw bezier curve
        let control_offset = ((end.x - start.x).abs() / 2.0).max(50.0);
        let control1 = egui::pos2(start.x + control_offset, start.y);
        let control2 = egui::pos2(end.x - control_offset, end.y);

        let color = if edge.selected {
            egui::Color32::from_rgb(100, 200, 255)
        } else {
            let (r, g, b) = edge.style.color.rgb();
            egui::Color32::from_rgb(r, g, b)
        };

        // Draw the curve using line segments
        let points = bezier_points(start, control1, control2, end, 20);
        for i in 0..points.len() - 1 {
            painter.line_segment(
                [points[i], points[i + 1]],
                egui::Stroke::new(2.0 * zoom, color)
            );
        }

        // Draw arrow head
        let arrow_size = 10.0 * zoom;
        let direction = (end - points[points.len() - 2]).normalized();
        let perpendicular = egui::vec2(-direction.y, direction.x);

        let arrow_points = vec![
            end,
            end - direction * arrow_size + perpendicular * arrow_size * 0.5,
            end - direction * arrow_size - perpendicular * arrow_size * 0.5,
        ];

        painter.add(egui::Shape::convex_polygon(
            arrow_points,
            color,
            egui::Stroke::NONE
        ));
    }

    /// Draw a group on the canvas
    fn draw_group(&self, painter: &egui::Painter, canvas_rect: egui::Rect, group: &imortal_ir::Group) {
        let zoom = self.project.viewport.zoom;
        let pan = egui::vec2(self.project.viewport.pan_x, self.project.viewport.pan_y);

        let group_rect = egui::Rect::from_min_size(
            canvas_rect.min + pan + egui::vec2(group.position.x * zoom, group.position.y * zoom),
            egui::vec2(group.size.width * zoom, group.size.height * zoom)
        );

        let (r, g, b) = group.color.rgb();
        let bg_color = egui::Color32::from_rgba_unmultiplied(r, g, b, (group.opacity * 255.0) as u8);

        painter.rect_filled(group_rect, 8.0, bg_color);
        painter.rect_stroke(group_rect, 8.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(r, g, b)));

        // Group title
        if group.show_header {
            painter.text(
                group_rect.min + egui::vec2(10.0, 15.0),
                egui::Align2::LEFT_CENTER,
                &group.name,
                egui::FontId::proportional(12.0 * zoom),
                egui::Color32::from_rgb(r, g, b)
            );
        }
    }

    /// Render the status bar
    fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Status message
                if let Some((msg, time)) = &self.status_message {
                    if time.elapsed().as_secs() < 5 {
                        ui.label(msg);
                    } else {
                        self.status_message = None;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "{} | {} nodes | {} edges",
                        self.project.meta.name,
                        self.project.node_count(),
                        self.project.edge_count()
                    ));
                });
            });
        });
    }

    /// Render dialogs
    fn render_dialogs(&mut self, ctx: &egui::Context) {
        // About dialog
        if self.show_about {
            egui::Window::new("About Immortal Engine")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Immortal Engine");
                        ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                        ui.add_space(10.0);
                        ui.label("Visual Prototyping System");
                        ui.label("Build applications by dragging components and drawing connections");
                        ui.add_space(20.0);
                        if ui.button("Close").clicked() {
                            self.show_about = false;
                        }
                    });
                });
        }

        // Settings dialog
        if self.show_settings {
            egui::Window::new("Settings")
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.checkbox(&mut self.config.dark_mode, "Dark Mode");
                    ui.checkbox(&mut self.config.show_grid, "Show Grid");
                    ui.checkbox(&mut self.config.snap_to_grid, "Snap to Grid");
                    ui.checkbox(&mut self.config.show_minimap, "Show Minimap");

                    ui.horizontal(|ui| {
                        ui.label("Grid Size:");
                        ui.add(egui::Slider::new(&mut self.config.grid_size, 10.0..=50.0));
                    });

                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }

    // File operations
    fn open_project(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Immortal Project", &["imortal"])
            .pick_file()
        {
            self.load_project_from_path(path);
        }
    }

    fn save_project(&mut self) {
        if let Some(path) = &self.project_path.clone() {
            match imortal_ir::save_project(&self.project, &path, imortal_ir::ProjectFormat::Json) {
                Ok(_) => self.set_status("Project saved"),
                Err(e) => self.set_status(format!("Failed to save: {}", e)),
            }
        } else {
            self.save_project_as();
        }
    }

    fn save_project_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Immortal Project", &["imortal"])
            .set_file_name(&format!("{}.imortal", self.project.meta.name))
            .save_file()
        {
            self.project_path = Some(path.clone());

            // Update recent projects
            self.welcome_screen.add_recent_project(self.project.meta.name.clone(), path.clone());

            match imortal_ir::save_project(&self.project, &path, imortal_ir::ProjectFormat::Json) {
                Ok(_) => self.set_status("Project saved"),
                Err(e) => self.set_status(format!("Failed to save: {}", e)),
            }
        }
    }

    fn generate_code(&mut self) {
        self.set_status("Code generation not yet implemented");
    }

    fn validate_project(&mut self) {
        match imortal_ir::validation::validate(&self.project) {
            Ok(_) => self.set_status("✅ Project is valid"),
            Err(errors) => self.set_status(format!("❌ {} validation errors found", errors.len())),
        }
    }
}

impl eframe::App for ImmortalApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Set dark/light mode
        if self.config.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        match self.mode {
            AppMode::Welcome => {
                // Show welcome screen
                let action = self.welcome_screen.show(ctx);
                self.handle_welcome_action(action);
            }
            AppMode::Editor => {
                // Render all editor UI components
                self.render_menu_bar(ctx, frame);
                self.render_status_bar(ctx);
                self.render_palette(ctx);
                self.render_properties(ctx);
                self.render_canvas(ctx);
                self.render_dialogs(ctx);
            }
        }
    }
}

/// Sanitize a project name for use as a directory/file name
fn sanitize_project_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c == ' ' {
                '_'
            } else {
                '_'
            }
        })
        .collect()
}

/// Calculate points along a cubic bezier curve
fn bezier_points(
    p0: egui::Pos2,
    p1: egui::Pos2,
    p2: egui::Pos2,
    p3: egui::Pos2,
    segments: usize
) -> Vec<egui::Pos2> {
    let mut points = Vec::with_capacity(segments + 1);

    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        let x = mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x;
        let y = mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y;

        points.push(egui::pos2(x, y));
    }

    points
}
