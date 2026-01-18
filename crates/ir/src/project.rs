//! Project metadata for Immortal Engine projects
//!
//! Contains the metadata and configuration for a project, including
//! name, version, description, domain configurations, and other settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use imortal_core::ConfigValue;

/// Metadata for an Immortal Engine project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeta {
    /// Project name (used for code generation)
    pub name: String,

    /// Project version (semver format)
    pub version: String,

    /// Human-readable description
    pub description: Option<String>,

    /// Project authors
    pub authors: Vec<String>,

    /// License identifier (e.g., "MIT", "Apache-2.0")
    pub license: Option<String>,

    /// Repository URL
    pub repository: Option<String>,

    /// Homepage URL
    pub homepage: Option<String>,

    /// Documentation URL
    pub documentation: Option<String>,

    /// Keywords for searchability
    pub keywords: Vec<String>,

    /// Categories (e.g., "web", "embedded", "cli")
    pub categories: Vec<String>,

    /// Enabled domains and their configurations
    pub domains: HashMap<String, DomainConfig>,

    /// Target language for code generation (default: "rust")
    pub target_language: String,

    /// Target framework (e.g., "axum", "actix", "rocket")
    pub target_framework: Option<String>,

    /// Output directory for generated code
    pub output_dir: String,

    /// Whether to generate tests
    pub generate_tests: bool,

    /// Whether to generate documentation
    pub generate_docs: bool,

    /// Custom metadata
    pub metadata: HashMap<String, ConfigValue>,

    /// IR format version used by this project
    pub ir_version: String,

    /// Timestamp when the project was created
    pub created_at: Option<String>,

    /// Timestamp when the project was last modified
    pub modified_at: Option<String>,
}

impl ProjectMeta {
    /// Create a new project with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            description: None,
            authors: Vec::new(),
            license: None,
            repository: None,
            homepage: None,
            documentation: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            domains: HashMap::new(),
            target_language: "rust".to_string(),
            target_framework: None,
            output_dir: "generated".to_string(),
            generate_tests: true,
            generate_docs: true,
            metadata: HashMap::new(),
            ir_version: crate::IR_VERSION.to_string(),
            created_at: None,
            modified_at: None,
        }
    }

    // ========== Builder Methods ==========

    /// Set the project version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Set the project description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add an author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }

    /// Set the license
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Set the repository URL
    pub fn with_repository(mut self, repository: impl Into<String>) -> Self {
        self.repository = Some(repository.into());
        self
    }

    /// Set the homepage URL
    pub fn with_homepage(mut self, homepage: impl Into<String>) -> Self {
        self.homepage = Some(homepage.into());
        self
    }

    /// Add a keyword
    pub fn with_keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keywords.push(keyword.into());
        self
    }

    /// Add a category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.categories.push(category.into());
        self
    }

    /// Enable and configure a domain
    pub fn with_domain(mut self, domain_id: impl Into<String>, config: DomainConfig) -> Self {
        self.domains.insert(domain_id.into(), config);
        self
    }

    /// Set the target language
    pub fn with_target_language(mut self, language: impl Into<String>) -> Self {
        self.target_language = language.into();
        self
    }

    /// Set the target framework
    pub fn with_target_framework(mut self, framework: impl Into<String>) -> Self {
        self.target_framework = Some(framework.into());
        self
    }

    /// Set the output directory
    pub fn with_output_dir(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    /// Disable test generation
    pub fn without_tests(mut self) -> Self {
        self.generate_tests = false;
        self
    }

    /// Disable documentation generation
    pub fn without_docs(mut self) -> Self {
        self.generate_docs = false;
        self
    }

    /// Add custom metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    // ========== Query Methods ==========

    /// Check if a domain is enabled
    pub fn has_domain(&self, domain_id: &str) -> bool {
        self.domains.contains_key(domain_id)
    }

    /// Get domain configuration
    pub fn get_domain(&self, domain_id: &str) -> Option<&DomainConfig> {
        self.domains.get(domain_id)
    }

    /// Get mutable domain configuration
    pub fn get_domain_mut(&mut self, domain_id: &str) -> Option<&mut DomainConfig> {
        self.domains.get_mut(domain_id)
    }

    /// Get all enabled domain IDs
    pub fn enabled_domains(&self) -> Vec<&String> {
        self.domains
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(id, _)| id)
            .collect()
    }

    /// Check if this is a database project
    pub fn has_database(&self) -> bool {
        self.has_domain("database")
    }

    /// Check if this is an embedded project
    pub fn has_embedded(&self) -> bool {
        self.has_domain("embedded")
    }

    /// Check if this is an API project
    pub fn has_api(&self) -> bool {
        self.has_domain("api")
    }

    // ========== Mutation Methods ==========

    /// Enable a domain with default configuration
    pub fn enable_domain(&mut self, domain_id: impl Into<String>) {
        let id = domain_id.into();
        if !self.domains.contains_key(&id) {
            self.domains.insert(id, DomainConfig::default());
        } else if let Some(config) = self.domains.get_mut(&id) {
            config.enabled = true;
        }
    }

    /// Disable a domain
    pub fn disable_domain(&mut self, domain_id: &str) {
        if let Some(config) = self.domains.get_mut(domain_id) {
            config.enabled = false;
        }
    }

    /// Update the modification timestamp
    pub fn touch(&mut self) {
        self.modified_at = Some(chrono_now());
    }
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self::new("untitled")
    }
}

/// Configuration for a specific domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainConfig {
    /// Whether this domain is enabled
    pub enabled: bool,

    /// Domain-specific settings
    pub settings: HashMap<String, ConfigValue>,

    /// Priority/order for code generation (lower = earlier)
    pub priority: i32,

    /// Additional features to enable for this domain
    pub features: Vec<String>,

    /// Custom templates directory for this domain
    pub templates_dir: Option<String>,
}

impl DomainConfig {
    /// Create a new enabled domain configuration
    pub fn new() -> Self {
        Self {
            enabled: true,
            settings: HashMap::new(),
            priority: 0,
            features: Vec::new(),
            templates_dir: None,
        }
    }

    /// Create a disabled domain configuration
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::new()
        }
    }

    // ========== Builder Methods ==========

    /// Set a configuration value
    pub fn with_setting(mut self, key: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.settings.insert(key.into(), value.into());
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Add a feature
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    /// Set the templates directory
    pub fn with_templates_dir(mut self, dir: impl Into<String>) -> Self {
        self.templates_dir = Some(dir.into());
        self
    }

    // ========== Query Methods ==========

    /// Get a setting value
    pub fn get_setting(&self, key: &str) -> Option<&ConfigValue> {
        self.settings.get(key)
    }

    /// Get a setting value as a string
    pub fn get_setting_str(&self, key: &str) -> Option<&str> {
        self.settings.get(key).and_then(|v| v.as_str())
    }

    /// Get a setting value as a bool
    pub fn get_setting_bool(&self, key: &str) -> Option<bool> {
        self.settings.get(key).and_then(|v| v.as_bool())
    }

    /// Get a setting value as an integer
    pub fn get_setting_int(&self, key: &str) -> Option<i64> {
        self.settings.get(key).and_then(|v| v.as_int())
    }

    /// Check if a feature is enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }

    // ========== Mutation Methods ==========

    /// Set a configuration value
    pub fn set_setting(&mut self, key: impl Into<String>, value: impl Into<ConfigValue>) {
        self.settings.insert(key.into(), value.into());
    }

    /// Remove a configuration value
    pub fn remove_setting(&mut self, key: &str) -> Option<ConfigValue> {
        self.settings.remove(key)
    }

    /// Add a feature
    pub fn add_feature(&mut self, feature: impl Into<String>) {
        let f = feature.into();
        if !self.features.contains(&f) {
            self.features.push(f);
        }
    }

    /// Remove a feature
    pub fn remove_feature(&mut self, feature: &str) {
        self.features.retain(|f| f != feature);
    }
}

impl Default for DomainConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset database configurations
pub struct DatabasePresets;

impl DatabasePresets {
    /// PostgreSQL configuration
    pub fn postgres() -> DomainConfig {
        DomainConfig::new()
            .with_setting("backend", "postgres")
            .with_setting("generate_migrations", true)
            .with_setting("pool_size", 10i64)
            .with_feature("uuid")
            .with_feature("json")
    }

    /// SQLite configuration
    pub fn sqlite() -> DomainConfig {
        DomainConfig::new()
            .with_setting("backend", "sqlite")
            .with_setting("generate_migrations", true)
            .with_setting("path", "data.db")
    }

    /// MySQL configuration
    pub fn mysql() -> DomainConfig {
        DomainConfig::new()
            .with_setting("backend", "mysql")
            .with_setting("generate_migrations", true)
            .with_setting("pool_size", 10i64)
    }
}

/// Preset API configurations
pub struct ApiPresets;

impl ApiPresets {
    /// REST API with Axum
    pub fn axum_rest() -> DomainConfig {
        DomainConfig::new()
            .with_setting("framework", "axum")
            .with_setting("style", "rest")
            .with_setting("port", 3000i64)
            .with_feature("cors")
            .with_feature("tracing")
    }

    /// REST API with Actix
    pub fn actix_rest() -> DomainConfig {
        DomainConfig::new()
            .with_setting("framework", "actix")
            .with_setting("style", "rest")
            .with_setting("port", 8080i64)
            .with_feature("cors")
    }

    /// GraphQL API
    pub fn graphql() -> DomainConfig {
        DomainConfig::new()
            .with_setting("framework", "async-graphql")
            .with_setting("style", "graphql")
            .with_setting("port", 3000i64)
            .with_feature("playground")
    }
}

/// Preset embedded configurations
pub struct EmbeddedPresets;

impl EmbeddedPresets {
    /// STM32 embedded configuration
    pub fn stm32() -> DomainConfig {
        DomainConfig::new()
            .with_setting("target", "stm32")
            .with_setting("chip", "stm32f4")
            .with_feature("hal")
            .with_feature("rtic")
    }

    /// ESP32 embedded configuration
    pub fn esp32() -> DomainConfig {
        DomainConfig::new()
            .with_setting("target", "esp32")
            .with_setting("framework", "esp-idf")
            .with_feature("wifi")
            .with_feature("bluetooth")
    }

    /// Raspberry Pi Pico configuration
    pub fn rp2040() -> DomainConfig {
        DomainConfig::new()
            .with_setting("target", "rp2040")
            .with_feature("hal")
            .with_feature("pio")
    }
}

/// Get current timestamp as ISO 8601 string
fn chrono_now() -> String {
    // Simple implementation without chrono dependency
    // In production, you'd use chrono or time crate
    "2024-01-01T00:00:00Z".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_meta_creation() {
        let meta = ProjectMeta::new("my_project")
            .with_version("1.0.0")
            .with_description("A test project")
            .with_author("Test Author");

        assert_eq!(meta.name, "my_project");
        assert_eq!(meta.version, "1.0.0");
        assert_eq!(meta.description, Some("A test project".to_string()));
        assert_eq!(meta.authors, vec!["Test Author".to_string()]);
    }

    #[test]
    fn test_project_domains() {
        let meta = ProjectMeta::new("my_project")
            .with_domain("database", DatabasePresets::postgres())
            .with_domain("api", ApiPresets::axum_rest());

        assert!(meta.has_database());
        assert!(meta.has_api());
        assert!(!meta.has_embedded());
        assert_eq!(meta.enabled_domains().len(), 2);
    }

    #[test]
    fn test_domain_config() {
        let config = DomainConfig::new()
            .with_setting("key", "value")
            .with_setting("count", 42i64)
            .with_feature("feature1")
            .with_priority(10);

        assert_eq!(config.get_setting_str("key"), Some("value"));
        assert_eq!(config.get_setting_int("count"), Some(42));
        assert!(config.has_feature("feature1"));
        assert_eq!(config.priority, 10);
    }

    #[test]
    fn test_database_presets() {
        let postgres = DatabasePresets::postgres();
        assert_eq!(postgres.get_setting_str("backend"), Some("postgres"));
        assert!(postgres.has_feature("uuid"));

        let sqlite = DatabasePresets::sqlite();
        assert_eq!(sqlite.get_setting_str("backend"), Some("sqlite"));
    }

    #[test]
    fn test_api_presets() {
        let axum = ApiPresets::axum_rest();
        assert_eq!(axum.get_setting_str("framework"), Some("axum"));
        assert!(axum.has_feature("cors"));
    }

    #[test]
    fn test_embedded_presets() {
        let stm32 = EmbeddedPresets::stm32();
        assert_eq!(stm32.get_setting_str("target"), Some("stm32"));
        assert!(stm32.has_feature("hal"));
    }

    #[test]
    fn test_enable_disable_domain() {
        let mut meta = ProjectMeta::new("test");

        meta.enable_domain("database");
        assert!(meta.has_domain("database"));

        meta.disable_domain("database");
        assert!(meta.has_domain("database")); // Still exists
        assert!(!meta.get_domain("database").unwrap().enabled); // But disabled
    }

    #[test]
    fn test_domain_config_mutation() {
        let mut config = DomainConfig::new();

        config.set_setting("key", "value");
        assert_eq!(config.get_setting_str("key"), Some("value"));

        config.add_feature("new_feature");
        assert!(config.has_feature("new_feature"));

        config.remove_feature("new_feature");
        assert!(!config.has_feature("new_feature"));

        config.remove_setting("key");
        assert!(config.get_setting("key").is_none());
    }
}
