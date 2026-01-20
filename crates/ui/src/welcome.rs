//! Welcome Screen for Immortal Engine
//!
//! This module provides the welcome/start screen that appears when the application
//! launches. It allows users to:
//! - Create a new project
//! - Open an existing project
//! - Access recent projects

use eframe::egui;
use std::path::PathBuf;

/// The result of the welcome screen interaction
#[derive(Debug, Clone)]
pub enum WelcomeAction {
    /// User hasn't made a choice yet
    None,
    /// User wants to create a new project
    CreateProject(NewProjectInfo),
    /// User wants to open an existing project
    OpenProject(PathBuf),
    /// User selected a recent project
    OpenRecentProject(PathBuf),
}

/// Information for creating a new project
#[derive(Debug, Clone)]
pub struct NewProjectInfo {
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Directory where the project will be created
    pub location: PathBuf,
    /// Template to use
    pub template: String,
}

/// Recent project entry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecentProject {
    /// Project name
    pub name: String,
    /// Full path to the project file
    pub path: PathBuf,
    /// Last opened timestamp
    pub last_opened: u64,
}

/// Welcome screen state
pub struct WelcomeScreen {
    /// Whether the welcome screen is visible
    pub visible: bool,
    /// Current view mode
    mode: WelcomeMode,
    /// New project form state
    new_project: NewProjectForm,
    /// Recent projects list
    pub recent_projects: Vec<RecentProject>,
    /// Error message to display
    error_message: Option<String>,
}

/// Welcome screen mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WelcomeMode {
    /// Main welcome view with options
    Main,
    /// Creating a new project
    NewProject,
}

/// Form state for creating a new project
struct NewProjectForm {
    name: String,
    description: String,
    location: String,
    template: String,
    templates: Vec<(String, String)>, // (id, display_name)
}

impl Default for NewProjectForm {
    fn default() -> Self {
        let default_location = dirs::document_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ImmortalEngineProjects");

        Self {
            name: "MyProject".to_string(),
            description: String::new(),
            location: default_location.to_string_lossy().to_string(),
            template: "default".to_string(),
            templates: vec![
                ("default".to_string(), "Empty Project".to_string()),
                ("web-api".to_string(), "Web API (REST)".to_string()),
                ("web-app".to_string(), "Full Web Application".to_string()),
                ("crud".to_string(), "CRUD Application".to_string()),
            ],
        }
    }
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl WelcomeScreen {
    /// Create a new welcome screen
    pub fn new() -> Self {
        let recent_projects = Self::load_recent_projects();

        Self {
            visible: true,
            mode: WelcomeMode::Main,
            new_project: NewProjectForm::default(),
            recent_projects,
            error_message: None,
        }
    }

    /// Load recent projects from config file
    fn load_recent_projects() -> Vec<RecentProject> {
        let config_path = Self::recent_projects_path();
        if let Some(path) = config_path {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(projects) = serde_json::from_str::<Vec<RecentProject>>(&content) {
                        // Filter out projects that no longer exist
                        return projects
                            .into_iter()
                            .filter(|p| p.path.exists())
                            .collect();
                    }
                }
            }
        }
        Vec::new()
    }

    /// Get the path to the recent projects config file
    fn recent_projects_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("immortal-engine").join("recent_projects.json"))
    }

    /// Save recent projects to config file
    pub fn save_recent_projects(&self) {
        if let Some(path) = Self::recent_projects_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(content) = serde_json::to_string_pretty(&self.recent_projects) {
                let _ = std::fs::write(&path, content);
            }
        }
    }

    /// Add a project to recent projects list
    pub fn add_recent_project(&mut self, name: String, path: PathBuf) {
        // Remove if already exists
        self.recent_projects.retain(|p| p.path != path);

        // Add to front
        self.recent_projects.insert(
            0,
            RecentProject {
                name,
                path,
                last_opened: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            },
        );

        // Keep only last 10
        self.recent_projects.truncate(10);

        self.save_recent_projects();
    }

    /// Remove a project from recent projects list
    pub fn remove_recent_project(&mut self, path: &PathBuf) {
        self.recent_projects.retain(|p| &p.path != path);
        self.save_recent_projects();
    }

    /// Show the welcome screen
    pub fn open(&mut self) {
        self.visible = true;
        self.mode = WelcomeMode::Main;
        self.error_message = None;
        self.recent_projects = Self::load_recent_projects();
    }

    /// Hide the welcome screen
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Reset new project form
    fn reset_new_project_form(&mut self) {
        self.new_project = NewProjectForm::default();
    }

    /// Render the welcome screen and return any action to perform
    pub fn show(&mut self, ctx: &egui::Context) -> WelcomeAction {
        if !self.visible {
            return WelcomeAction::None;
        }

        let mut action = WelcomeAction::None;

        // Full-screen central panel for welcome screen
        egui::CentralPanel::default().show(ctx, |ui| {
            // Center content vertically and horizontally
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.1);

                match self.mode {
                    WelcomeMode::Main => {
                        action = self.render_main_view(ui);
                    }
                    WelcomeMode::NewProject => {
                        action = self.render_new_project_view(ui);
                    }
                }
            });
        });

        action
    }

    /// Render the main welcome view
    fn render_main_view(&mut self, ui: &mut egui::Ui) -> WelcomeAction {
        let mut action = WelcomeAction::None;

        // Logo and title
        ui.heading(egui::RichText::new("🔮 Immortal Engine").size(48.0));
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("Visual Prototyping System for Building Applications in Rust")
                .size(16.0)
                .color(egui::Color32::GRAY),
        );
        ui.add_space(40.0);

        // Action buttons in a horizontal layout
        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2.0 - 200.0);

            // New Project button
            let new_btn = egui::Button::new(
                egui::RichText::new("📁  New Project").size(18.0),
            )
            .min_size(egui::vec2(180.0, 50.0));

            if ui.add(new_btn).clicked() {
                self.mode = WelcomeMode::NewProject;
                self.reset_new_project_form();
            }

            ui.add_space(20.0);

            // Open Project button
            let open_btn = egui::Button::new(
                egui::RichText::new("📂  Open Project").size(18.0),
            )
            .min_size(egui::vec2(180.0, 50.0));

            if ui.add(open_btn).clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Immortal Project", &["imortal"])
                    .pick_file()
                {
                    action = WelcomeAction::OpenProject(path);
                }
            }
        });

        ui.add_space(50.0);

        // Recent Projects section
        if !self.recent_projects.is_empty() {
            ui.separator();
            ui.add_space(20.0);

            ui.label(
                egui::RichText::new("Recent Projects")
                    .size(20.0)
                    .strong(),
            );
            ui.add_space(15.0);

            // Show recent projects in a scrollable area
            egui::ScrollArea::vertical()
                .max_height(250.0)
                .show(ui, |ui| {
                    let mut project_to_remove: Option<PathBuf> = None;

                    for project in &self.recent_projects {
                        ui.horizontal(|ui| {
                            // Project icon and name
                            let project_btn = ui.add(
                                egui::Button::new(
                                    egui::RichText::new(format!("📊 {}", project.name)).size(14.0),
                                )
                                .min_size(egui::vec2(200.0, 30.0))
                                .frame(false),
                            );

                            if project_btn.clicked() {
                                action = WelcomeAction::OpenRecentProject(project.path.clone());
                            }

                            // Path display
                            ui.label(
                                egui::RichText::new(project.path.to_string_lossy().to_string())
                                    .size(12.0)
                                    .color(egui::Color32::DARK_GRAY),
                            );

                            // Remove button
                            if ui.small_button("✕").clicked() {
                                project_to_remove = Some(project.path.clone());
                            }
                        });
                        ui.add_space(5.0);
                    }

                    // Remove project if requested (outside the iteration)
                    if let Some(path) = project_to_remove {
                        self.remove_recent_project(&path);
                    }
                });
        }

        // Version info at bottom
        ui.add_space(30.0);
        ui.label(
            egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                .size(12.0)
                .color(egui::Color32::DARK_GRAY),
        );

        action
    }

    /// Render the new project creation view
    fn render_new_project_view(&mut self, ui: &mut egui::Ui) -> WelcomeAction {
        let mut action = WelcomeAction::None;

        ui.heading(egui::RichText::new("Create New Project").size(32.0));
        ui.add_space(30.0);

        // Form container
        egui::Frame::none()
            .inner_margin(egui::Margin::same(20.0))
            .show(ui, |ui| {
                ui.set_min_width(500.0);

                // Project name
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Project Name:").size(14.0));
                    ui.add_space(20.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_project.name)
                            .desired_width(300.0)
                            .hint_text("Enter project name"),
                    );
                });
                ui.add_space(15.0);

                // Description
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Description:").size(14.0));
                    ui.add_space(32.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_project.description)
                            .desired_width(300.0)
                            .hint_text("Optional description"),
                    );
                });
                ui.add_space(15.0);

                // Location
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Location:").size(14.0));
                    ui.add_space(50.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_project.location)
                            .desired_width(250.0),
                    );
                    if ui.button("Browse...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.new_project.location = path.to_string_lossy().to_string();
                        }
                    }
                });
                ui.add_space(15.0);

                // Template selection
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Template:").size(14.0));
                    ui.add_space(52.0);

                    let current_template = self
                        .new_project
                        .templates
                        .iter()
                        .find(|(id, _)| id == &self.new_project.template)
                        .map(|(_, name)| name.as_str())
                        .unwrap_or("Empty Project");

                    egui::ComboBox::from_id_salt("template_selector")
                        .width(300.0)
                        .selected_text(current_template)
                        .show_ui(ui, |ui| {
                            for (id, display_name) in &self.new_project.templates {
                                ui.selectable_value(
                                    &mut self.new_project.template,
                                    id.clone(),
                                    display_name,
                                );
                            }
                        });
                });

                // Error message
                if let Some(error) = &self.error_message {
                    ui.add_space(15.0);
                    ui.colored_label(egui::Color32::RED, error);
                }

                ui.add_space(30.0);

                // Buttons
                ui.horizontal(|ui| {
                    ui.add_space(100.0);

                    let create_btn = egui::Button::new(
                        egui::RichText::new("Create Project").size(16.0),
                    )
                    .min_size(egui::vec2(140.0, 40.0));

                    if ui.add(create_btn).clicked() {
                        // Validate input
                        if self.new_project.name.trim().is_empty() {
                            self.error_message = Some("Project name cannot be empty".to_string());
                        } else if self.new_project.location.trim().is_empty() {
                            self.error_message = Some("Location cannot be empty".to_string());
                        } else {
                            // Sanitize project name for filesystem
                            let sanitized_name = sanitize_filename(&self.new_project.name);
                            let location = PathBuf::from(&self.new_project.location);

                            action = WelcomeAction::CreateProject(NewProjectInfo {
                                name: self.new_project.name.clone(),
                                description: self.new_project.description.clone(),
                                location,
                                template: self.new_project.template.clone(),
                            });
                        }
                    }

                    ui.add_space(20.0);

                    let cancel_btn = egui::Button::new(
                        egui::RichText::new("Cancel").size(16.0),
                    )
                    .min_size(egui::vec2(100.0, 40.0));

                    if ui.add(cancel_btn).clicked() {
                        self.mode = WelcomeMode::Main;
                        self.error_message = None;
                    }
                });
            });

        action
    }
}

/// Sanitize a string for use as a filename/directory name
fn sanitize_filename(name: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_screen_creation() {
        let screen = WelcomeScreen::new();
        assert!(screen.visible);
        assert_eq!(screen.mode, WelcomeMode::Main);
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("My Project"), "My_Project");
        assert_eq!(sanitize_filename("test-project"), "test-project");
        assert_eq!(sanitize_filename("project_1"), "project_1");
        assert_eq!(sanitize_filename("bad/name"), "bad_name");
    }

    #[test]
    fn test_new_project_info() {
        let info = NewProjectInfo {
            name: "Test".to_string(),
            description: "A test project".to_string(),
            location: PathBuf::from("/tmp"),
            template: "default".to_string(),
        };
        assert_eq!(info.name, "Test");
    }

    #[test]
    fn test_recent_project() {
        let project = RecentProject {
            name: "Test Project".to_string(),
            path: PathBuf::from("/home/user/test.imortal"),
            last_opened: 12345,
        };
        assert_eq!(project.name, "Test Project");
    }
}
