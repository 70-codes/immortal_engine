//! Serialization module for loading and saving Immortal Engine projects
//!
//! This module provides functionality to serialize and deserialize ProjectGraph
//! instances to various formats (JSON, TOML) and handle file I/O operations.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use imortal_core::{EngineError, EngineResult};

use crate::graph::ProjectGraph;
use crate::{IR_VERSION, PROJECT_EXTENSION};

/// Supported project file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProjectFormat {
    /// JSON format (default, human-readable)
    #[default]
    Json,
    /// Compact JSON (minified, smaller file size)
    JsonCompact,
    /// TOML format (human-readable, good for config-like data)
    Toml,
}

impl ProjectFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ProjectFormat::Json | ProjectFormat::JsonCompact => "json",
            ProjectFormat::Toml => "toml",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(ProjectFormat::Json),
            "toml" => Some(ProjectFormat::Toml),
            PROJECT_EXTENSION => Some(ProjectFormat::Json), // .imortal files are JSON
            _ => None,
        }
    }

    /// Detect format from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

/// Wrapper for project files that includes format version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    /// IR format version
    pub ir_version: String,
    /// File format identifier
    pub format: String,
    /// The actual project graph
    pub project: ProjectGraph,
}

impl ProjectFile {
    /// Create a new project file wrapper
    pub fn new(project: ProjectGraph) -> Self {
        Self {
            ir_version: IR_VERSION.to_string(),
            format: "imortal".to_string(),
            project,
        }
    }

    /// Check if this file is compatible with the current IR version
    pub fn is_compatible(&self) -> bool {
        // For now, just check major version
        let current_major = IR_VERSION.split('.').next().unwrap_or("1");
        let file_major = self.ir_version.split('.').next().unwrap_or("0");
        current_major == file_major
    }

    /// Get the project, consuming the wrapper
    pub fn into_project(self) -> ProjectGraph {
        self.project
    }
}

/// Load a project from a file path
pub fn load_project(path: impl AsRef<Path>) -> EngineResult<ProjectGraph> {
    let path = path.as_ref();

    // Check file exists
    if !path.exists() {
        return Err(EngineError::FileNotFound(path.display().to_string()));
    }

    // Detect format from extension
    let format = ProjectFormat::from_path(path)
        .unwrap_or(ProjectFormat::Json);

    // Read file contents
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse based on format
    let project_file: ProjectFile = match format {
        ProjectFormat::Json | ProjectFormat::JsonCompact => {
            serde_json::from_str(&contents)
                .map_err(|e| EngineError::Deserialization(format!("JSON parse error: {}", e)))?
        }
        ProjectFormat::Toml => {
            toml::from_str(&contents)
                .map_err(|e| EngineError::Deserialization(format!("TOML parse error: {}", e)))?
        }
    };

    // Check compatibility
    if !project_file.is_compatible() {
        return Err(EngineError::Deserialization(format!(
            "Incompatible IR version: file is {}, current is {}",
            project_file.ir_version, IR_VERSION
        )));
    }

    Ok(project_file.into_project())
}

/// Save a project to a file path
pub fn save_project(
    project: &ProjectGraph,
    path: impl AsRef<Path>,
    format: ProjectFormat,
) -> EngineResult<()> {
    let path = path.as_ref();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // Create wrapper
    let project_file = ProjectFile::new(project.clone());

    // Serialize based on format
    let contents = match format {
        ProjectFormat::Json => {
            serde_json::to_string_pretty(&project_file)
                .map_err(|e| EngineError::Serialization(format!("JSON serialize error: {}", e)))?
        }
        ProjectFormat::JsonCompact => {
            serde_json::to_string(&project_file)
                .map_err(|e| EngineError::Serialization(format!("JSON serialize error: {}", e)))?
        }
        ProjectFormat::Toml => {
            toml::to_string_pretty(&project_file)
                .map_err(|e| EngineError::Serialization(format!("TOML serialize error: {}", e)))?
        }
    };

    // Write to file
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    file.flush()?;

    Ok(())
}

/// Load a project from a JSON string
pub fn load_from_json(json: &str) -> EngineResult<ProjectGraph> {
    let project_file: ProjectFile = serde_json::from_str(json)
        .map_err(|e| EngineError::Deserialization(format!("JSON parse error: {}", e)))?;

    if !project_file.is_compatible() {
        return Err(EngineError::Deserialization(format!(
            "Incompatible IR version: file is {}, current is {}",
            project_file.ir_version, IR_VERSION
        )));
    }

    Ok(project_file.into_project())
}

/// Save a project to a JSON string
pub fn save_to_json(project: &ProjectGraph, pretty: bool) -> EngineResult<String> {
    let project_file = ProjectFile::new(project.clone());

    if pretty {
        serde_json::to_string_pretty(&project_file)
            .map_err(|e| EngineError::Serialization(format!("JSON serialize error: {}", e)))
    } else {
        serde_json::to_string(&project_file)
            .map_err(|e| EngineError::Serialization(format!("JSON serialize error: {}", e)))
    }
}

/// Load a project from a TOML string
pub fn load_from_toml(toml_str: &str) -> EngineResult<ProjectGraph> {
    let project_file: ProjectFile = toml::from_str(toml_str)
        .map_err(|e| EngineError::Deserialization(format!("TOML parse error: {}", e)))?;

    if !project_file.is_compatible() {
        return Err(EngineError::Deserialization(format!(
            "Incompatible IR version: file is {}, current is {}",
            project_file.ir_version, IR_VERSION
        )));
    }

    Ok(project_file.into_project())
}

/// Save a project to a TOML string
pub fn save_to_toml(project: &ProjectGraph) -> EngineResult<String> {
    let project_file = ProjectFile::new(project.clone());

    toml::to_string_pretty(&project_file)
        .map_err(|e| EngineError::Serialization(format!("TOML serialize error: {}", e)))
}

/// Export only the project graph (without wrapper) to JSON
pub fn export_graph_json(project: &ProjectGraph, pretty: bool) -> EngineResult<String> {
    if pretty {
        serde_json::to_string_pretty(project)
            .map_err(|e| EngineError::Serialization(format!("JSON serialize error: {}", e)))
    } else {
        serde_json::to_string(project)
            .map_err(|e| EngineError::Serialization(format!("JSON serialize error: {}", e)))
    }
}

/// Import a project graph from raw JSON (without wrapper)
pub fn import_graph_json(json: &str) -> EngineResult<ProjectGraph> {
    serde_json::from_str(json)
        .map_err(|e| EngineError::Deserialization(format!("JSON parse error: {}", e)))
}

/// Auto-save configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSaveConfig {
    /// Whether auto-save is enabled
    pub enabled: bool,
    /// Interval in seconds between auto-saves
    pub interval_secs: u64,
    /// Maximum number of auto-save files to keep
    pub max_files: usize,
    /// Directory for auto-save files
    pub directory: Option<String>,
}

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 60,
            max_files: 5,
            directory: None,
        }
    }
}

/// Recent projects list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecentProjects {
    /// List of recent project paths
    pub paths: Vec<String>,
    /// Maximum number of recent projects to track
    pub max_entries: usize,
}

impl RecentProjects {
    /// Create a new recent projects list
    pub fn new(max_entries: usize) -> Self {
        Self {
            paths: Vec::new(),
            max_entries,
        }
    }

    /// Add a project path to the list
    pub fn add(&mut self, path: impl Into<String>) {
        let path = path.into();

        // Remove if already exists (to move to front)
        self.paths.retain(|p| p != &path);

        // Add to front
        self.paths.insert(0, path);

        // Trim to max
        self.paths.truncate(self.max_entries);
    }

    /// Remove a project path from the list
    pub fn remove(&mut self, path: &str) {
        self.paths.retain(|p| p != path);
    }

    /// Clear the list
    pub fn clear(&mut self) {
        self.paths.clear();
    }

    /// Get the most recent project path
    pub fn most_recent(&self) -> Option<&str> {
        self.paths.first().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;
    use crate::project::ProjectMeta;

    fn create_test_project() -> ProjectGraph {
        let mut project = ProjectGraph::new(ProjectMeta::new("test_project"));
        project.add_node(Node::new_entity("User"));
        project.add_node(Node::new_entity("Post"));
        project
    }

    #[test]
    fn test_format_detection() {
        assert_eq!(ProjectFormat::from_extension("json"), Some(ProjectFormat::Json));
        assert_eq!(ProjectFormat::from_extension("toml"), Some(ProjectFormat::Toml));
        assert_eq!(ProjectFormat::from_extension("imortal"), Some(ProjectFormat::Json));
        assert_eq!(ProjectFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_json_roundtrip() {
        let project = create_test_project();

        let json = save_to_json(&project, true).unwrap();
        let loaded = load_from_json(&json).unwrap();

        assert_eq!(project.meta.name, loaded.meta.name);
        assert_eq!(project.node_count(), loaded.node_count());
    }

    #[test]
    fn test_json_compact() {
        let project = create_test_project();

        let pretty = save_to_json(&project, true).unwrap();
        let compact = save_to_json(&project, false).unwrap();

        // Compact should be shorter
        assert!(compact.len() < pretty.len());
    }

    #[test]
    fn test_toml_roundtrip() {
        let project = create_test_project();

        let toml_str = save_to_toml(&project).unwrap();
        let loaded = load_from_toml(&toml_str).unwrap();

        assert_eq!(project.meta.name, loaded.meta.name);
        assert_eq!(project.node_count(), loaded.node_count());
    }

    #[test]
    fn test_project_file_compatibility() {
        let project = create_test_project();
        let file = ProjectFile::new(project);

        assert!(file.is_compatible());
        assert_eq!(file.ir_version, IR_VERSION);
    }

    #[test]
    fn test_export_import_graph() {
        let project = create_test_project();

        let json = export_graph_json(&project, false).unwrap();
        let imported = import_graph_json(&json).unwrap();

        assert_eq!(project.meta.name, imported.meta.name);
    }

    #[test]
    fn test_recent_projects() {
        let mut recent = RecentProjects::new(3);

        recent.add("/path/to/project1.imortal");
        recent.add("/path/to/project2.imortal");
        recent.add("/path/to/project3.imortal");

        assert_eq!(recent.most_recent(), Some("/path/to/project3.imortal"));

        // Adding existing should move to front
        recent.add("/path/to/project1.imortal");
        assert_eq!(recent.most_recent(), Some("/path/to/project1.imortal"));

        // Should not exceed max
        recent.add("/path/to/project4.imortal");
        assert_eq!(recent.paths.len(), 3);
    }

    #[test]
    fn test_autosave_config_default() {
        let config = AutoSaveConfig::default();

        assert!(config.enabled);
        assert_eq!(config.interval_secs, 60);
        assert_eq!(config.max_files, 5);
    }
}
