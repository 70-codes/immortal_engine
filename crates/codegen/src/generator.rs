//! Code Generator for Immortal Engine
//!
//! This module provides the core code generation functionality that transforms
//! ProjectGraph (IR) into actual source code.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use imortal_core::{EngineResult, ConfigValue};
use imortal_ir::ProjectGraph;
use imortal_components::ComponentRegistry;

use crate::rust::{
    migrations::{DatabaseBackend, generate_all_migrations},
    auth::{AuthGenerator, AuthConfig, AuthFramework, generate_auth_routes},
    models::{generate_model, generate_model_impl},
    handlers::generate_router,
    config::{generate_config, generate_error},
};

/// Main code generator that orchestrates the generation process
pub struct CodeGenerator {
    /// Configuration for the generator
    config: GeneratorConfig,
    /// Component registry for looking up component definitions
    registry: ComponentRegistry,
}

impl CodeGenerator {
    /// Create a new code generator with default configuration
    pub fn new() -> Self {
        Self {
            config: GeneratorConfig::default(),
            registry: ComponentRegistry::with_builtins(),
        }
    }

    /// Create a new code generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Self {
        Self {
            config,
            registry: ComponentRegistry::with_builtins(),
        }
    }

    /// Set the component registry
    pub fn with_registry(mut self, registry: ComponentRegistry) -> Self {
        self.registry = registry;
        self
    }

    /// Generate code from a project graph
    pub fn generate(&self, graph: &ProjectGraph) -> EngineResult<GeneratedProject> {
        // Validate the graph first
        let validation_errors = imortal_ir::validation::get_all_issues(graph);
        let errors: Vec<_> = validation_errors.iter().filter(|e| e.is_error()).collect();
        if !errors.is_empty() {
            let error_msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            return Err(imortal_core::EngineError::ValidationFailed(
                error_msgs.join("; ")
            ));
        }

        // Create the generated project structure
        let mut project = GeneratedProject::new(&graph.meta.name);

        // Generate Cargo.toml
        project.add_file(
            "Cargo.toml",
            self.generate_cargo_toml(graph)?,
        );

        // Generate main.rs
        project.add_file(
            "src/main.rs",
            self.generate_main_rs(graph)?,
        );

        // Generate lib.rs (module declarations)
        project.add_file(
            "src/lib.rs",
            self.generate_lib_rs(graph)?,
        );

        // Generate config.rs
        project.add_file(
            "src/config.rs",
            generate_config(self.config.auth_framework, self.config.database_backend),
        );

        // Generate error.rs
        project.add_file(
            "src/error.rs",
            generate_error(self.config.auth_framework),
        );

        // Generate models
        let entity_nodes: Vec<_> = graph.nodes()
            .filter(|n| n.component_type == "data.entity")
            .collect();

        if !entity_nodes.is_empty() {
            // Generate models/mod.rs
            let mut model_mod = String::from("//! Data models\n\n");
            for node in &entity_nodes {
                let mod_name = crate::rust::to_snake_case(&node.name);
                model_mod.push_str(&format!("pub mod {};\n", mod_name));
                model_mod.push_str(&format!("pub use {}::{};\n", mod_name, node.name));
            }
            project.add_file("src/models/mod.rs", model_mod);

            // Generate individual model files
            for node in &entity_nodes {
                let content = generate_model(node);
                let impl_content = generate_model_impl(node);
                let full_content = format!("{}\n{}", content, impl_content);
                project.add_file(
                    format!("src/models/{}.rs", crate::rust::to_snake_case(&node.name)),
                    full_content,
                );
            }
        }

        // Generate auth module if there are auth nodes
        let has_auth = graph.nodes().any(|n| n.component_type.starts_with("auth."));
        if has_auth {
            let auth_gen = AuthGenerator::new(AuthConfig {
                framework: self.config.auth_framework,
                ..Default::default()
            });
            let auth_code = auth_gen.generate(graph)?;

            project.add_file("src/auth/mod.rs", auth_code.to_module());

            // Generate auth routes
            let auth_routes = generate_auth_routes(self.config.auth_framework);
            if !auth_routes.is_empty() {
                project.add_file("src/auth/routes.rs", auth_routes);
            }
        }

        // Generate API handlers
        let api_nodes: Vec<_> = graph.nodes()
            .filter(|n| n.component_type.starts_with("api."))
            .collect();

        if !api_nodes.is_empty() {
            project.add_file(
                "src/handlers/mod.rs",
                self.generate_handlers_mod(&api_nodes)?,
            );

            for node in &api_nodes {
                let content = self.generate_api_handler(node)?;
                project.add_file(
                    format!("src/handlers/{}.rs", crate::rust::to_snake_case(&node.name)),
                    content,
                );
            }

            // Generate router
            let router_code = generate_router(&api_nodes);
            project.add_file("src/routes.rs", router_code);
        }

        // Generate database migrations
        if self.config.generate_migrations && !entity_nodes.is_empty() {
            let migrations = generate_all_migrations(graph, self.config.database_backend)?;
            for (filename, content) in migrations {
                project.add_file(format!("migrations/{}", filename), content);
            }
        }

        // Generate .env.example
        project.add_file(".env.example", self.generate_env_example(graph));

        // Generate README.md
        project.add_file("README.md", self.generate_readme(graph));

        Ok(project)
    }

    /// Generate Cargo.toml content
    fn generate_cargo_toml(&self, graph: &ProjectGraph) -> EngineResult<String> {
        let mut deps = vec![
            ("tokio", r#"{ version = "1", features = ["full"] }"#),
            ("serde", r#"{ version = "1", features = ["derive"] }"#),
            ("serde_json", r#""1""#),
            ("thiserror", r#""1""#),
            ("tracing", r#""0.1""#),
            ("tracing-subscriber", r#"{ version = "0.3", features = ["env-filter"] }"#),
            ("uuid", r#"{ version = "1", features = ["v4", "serde"] }"#),
            ("chrono", r#"{ version = "0.4", features = ["serde"] }"#),
            ("dotenv", r#""0.15""#),
        ];

        // Add framework-specific dependencies
        match self.config.auth_framework {
            AuthFramework::Axum => {
                deps.push(("axum", r#"{ version = "0.7", features = ["macros"] }"#));
                deps.push(("tower", r#""0.4""#));
                deps.push(("tower-http", r#"{ version = "0.5", features = ["cors", "trace"] }"#));
            }
            AuthFramework::Actix => {
                deps.push(("actix-web", r#""4""#));
                deps.push(("actix-rt", r#""2""#));
            }
            AuthFramework::Custom => {}
        }

        // Add database dependencies
        match self.config.database_backend {
            DatabaseBackend::Postgres => {
                deps.push(("sqlx", r#"{ version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }"#));
            }
            DatabaseBackend::Sqlite => {
                deps.push(("sqlx", r#"{ version = "0.7", features = ["runtime-tokio", "sqlite", "uuid", "chrono"] }"#));
            }
            DatabaseBackend::Mysql => {
                deps.push(("sqlx", r#"{ version = "0.7", features = ["runtime-tokio", "mysql", "uuid", "chrono"] }"#));
            }
        }

        // Add auth dependencies
        let has_auth = graph.nodes().any(|n| n.component_type.starts_with("auth."));
        if has_auth {
            deps.push(("jsonwebtoken", r#""9""#));
            deps.push(("argon2", r#""0.5""#));
        }

        let deps_str: String = deps.iter()
            .map(|(name, version)| format!("{} = {}", name, version))
            .collect::<Vec<_>>()
            .join("\n");

        let content = format!(
            r#"[package]
name = "{}"
version = "{}"
edition = "2021"
description = "{}"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
{}

[dev-dependencies]
tokio-test = "0.4"

[[bin]]
name = "{}"
path = "src/main.rs"
"#,
            crate::rust::to_snake_case(&graph.meta.name),
            graph.meta.version,
            graph.meta.description.as_deref().unwrap_or("Generated by Immortal Engine"),
            deps_str,
            crate::rust::to_snake_case(&graph.meta.name),
        );
        Ok(content)
    }

    /// Generate main.rs content
    fn generate_main_rs(&self, graph: &ProjectGraph) -> EngineResult<String> {
        let name = &graph.meta.name;
        let snake_name = crate::rust::to_snake_case(name);

        let content = match self.config.auth_framework {
            AuthFramework::Axum => {
                format!(
                    r#"//! {} - Generated by Immortal Engine
//!
//! {}

use {}::{{create_app, Config}};
use tracing_subscriber::{{layer::SubscriberExt, util::SubscriberInitExt}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {{
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "{}=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    // Create and run the application
    let app = create_app(config.clone()).await?;

    let addr = format!("{{}}:{{}}", config.host, config.port);
    tracing::info!("Starting {} on {{}}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}}
"#,
                    name,
                    graph.meta.description.as_deref().unwrap_or(""),
                    snake_name,
                    snake_name,
                    name,
                )
            }
            AuthFramework::Actix => {
                format!(
                    r#"//! {} - Generated by Immortal Engine
//!
//! {}

use {}::{{create_app, Config}};
use tracing_subscriber::{{layer::SubscriberExt, util::SubscriberInitExt}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {{
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "{}=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load config");

    let addr = format!("{{}}:{{}}", config.host, config.port);
    tracing::info!("Starting {} on {{}}", addr);

    create_app(config).await?.await
}}
"#,
                    name,
                    graph.meta.description.as_deref().unwrap_or(""),
                    snake_name,
                    snake_name,
                    name,
                )
            }
            AuthFramework::Custom => {
                format!(
                    r#"//! {} - Generated by Immortal Engine
//!
//! {}

use {}::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {{
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::init();

    // Load configuration
    let config = Config::from_env()?;

    tracing::info!("Starting {}...");

    // TODO: Add your application logic here

    Ok(())
}}
"#,
                    name,
                    graph.meta.description.as_deref().unwrap_or(""),
                    snake_name,
                    name,
                )
            }
        };

        Ok(content)
    }

    /// Generate lib.rs with module declarations
    fn generate_lib_rs(&self, graph: &ProjectGraph) -> EngineResult<String> {
        let mut modules = vec!["config", "error"];

        let has_entities = graph.nodes().any(|n| n.component_type == "data.entity");
        let has_auth = graph.nodes().any(|n| n.component_type.starts_with("auth."));
        let has_api = graph.nodes().any(|n| n.component_type.starts_with("api."));

        if has_entities {
            modules.push("models");
        }
        if has_auth {
            modules.push("auth");
        }
        if has_api {
            modules.push("handlers");
            modules.push("routes");
        }

        let mod_declarations: String = modules.iter()
            .map(|m| format!("pub mod {};", m))
            .collect::<Vec<_>>()
            .join("\n");

        let re_exports: String = modules.iter()
            .map(|m| format!("pub use {}::*;", m))
            .collect::<Vec<_>>()
            .join("\n");

        let app_code = match self.config.auth_framework {
            AuthFramework::Axum => {
                r#"
use axum::Router;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: std::sync::Arc<DatabasePool>,
    pub config: Config,
}

/// Create the application with all routes configured
pub async fn create_app(config: Config) -> anyhow::Result<Router> {
    // Initialize database connection
    let db = DatabasePool::connect(&config.database_url).await?;

    let state = AppState {
        db: std::sync::Arc::new(db),
        config,
    };

    let app = Router::new()
        .merge(routes::create_router())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    Ok(app)
}
"#
            }
            AuthFramework::Actix => {
                r#"
use actix_web::{web, App, HttpServer};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: std::sync::Arc<DatabasePool>,
    pub config: Config,
}

/// Create the application with all routes configured
pub async fn create_app(config: Config) -> std::io::Result<actix_web::dev::Server> {
    // Initialize database connection
    let db = DatabasePool::connect(&config.database_url).await
        .expect("Failed to connect to database");

    let state = web::Data::new(AppState {
        db: std::sync::Arc::new(db),
        config: config.clone(),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run();

    Ok(server)
}
"#
            }
            AuthFramework::Custom => "",
        };

        let content = format!(
            r#"//! {} library
//!
//! Generated by Immortal Engine

{}

{}

// Re-export config
pub use config::Config;
{}
"#,
            graph.meta.name,
            mod_declarations,
            re_exports,
            app_code,
        );

        // Also generate config.rs
        Ok(content)
    }

    /// Generate handlers module
    fn generate_handlers_mod(&self, nodes: &[&imortal_ir::Node]) -> EngineResult<String> {
        let mut content = String::from("//! API handlers\n\n");

        for node in nodes {
            let mod_name = crate::rust::to_snake_case(&node.name);
            content.push_str(&format!("pub mod {};\n", mod_name));
        }

        Ok(content)
    }

    /// Generate API handler for a node
    fn generate_api_handler(&self, node: &imortal_ir::Node) -> EngineResult<String> {
        let handler_name = crate::rust::to_snake_case(&node.name);

        let content = match self.config.auth_framework {
            AuthFramework::Axum => {
                format!(
                    r#"//! {} handler

use axum::{{
    extract::{{Path, Query, State}},
    response::IntoResponse,
    Json,
}};
use serde::{{Deserialize, Serialize}};
use crate::{{AppState, error::AppError}};

/// {} handler
pub async fn {}(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {{
    // TODO: Implement handler logic
    Ok(Json(serde_json::json!({{
        "status": "ok",
        "handler": "{}"
    }})))
}}
"#,
                    node.name,
                    node.name,
                    handler_name,
                    handler_name,
                )
            }
            AuthFramework::Actix => {
                format!(
                    r#"//! {} handler

use actix_web::{{web, HttpResponse}};
use serde::{{Deserialize, Serialize}};
use crate::{{AppState, error::AppError}};

/// {} handler
pub async fn {}(
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {{
    // TODO: Implement handler logic
    Ok(HttpResponse::Ok().json(serde_json::json!({{
        "status": "ok",
        "handler": "{}"
    }})))
}}
"#,
                    node.name,
                    node.name,
                    handler_name,
                    handler_name,
                )
            }
            AuthFramework::Custom => {
                format!(
                    r#"//! {} handler

/// {} handler function
pub async fn {}() -> Result<(), Box<dyn std::error::Error>> {{
    // TODO: Implement handler logic
    Ok(())
}}
"#,
                    node.name,
                    node.name,
                    handler_name,
                )
            }
        };

        Ok(content)
    }

    /// Generate .env.example file
    fn generate_env_example(&self, graph: &ProjectGraph) -> String {
        let mut lines = vec![
            format!("# {} Environment Variables", graph.meta.name),
            "# Copy this to .env and fill in the values".to_string(),
            "".to_string(),
            "# Server".to_string(),
            "HOST=127.0.0.1".to_string(),
            "PORT=3000".to_string(),
            "".to_string(),
        ];

        // Database
        match self.config.database_backend {
            DatabaseBackend::Postgres => {
                lines.push("# Database (Postgres)".to_string());
                lines.push("DATABASE_URL=postgres://user:password@localhost:5432/dbname".to_string());
            }
            DatabaseBackend::Sqlite => {
                lines.push("# Database (SQLite)".to_string());
                lines.push("DATABASE_URL=sqlite:./data.db".to_string());
            }
            DatabaseBackend::Mysql => {
                lines.push("# Database (MySQL)".to_string());
                lines.push("DATABASE_URL=mysql://user:password@localhost:3306/dbname".to_string());
            }
        }

        // Auth
        let has_auth = graph.nodes().any(|n| n.component_type.starts_with("auth."));
        if has_auth {
            lines.push("".to_string());
            lines.push("# Authentication".to_string());
            lines.push("JWT_SECRET=your-super-secret-jwt-key-change-in-production".to_string());
            lines.push("JWT_EXPIRY_HOURS=24".to_string());
        }

        lines.push("".to_string());
        lines.push("# Logging".to_string());
        lines.push("RUST_LOG=debug".to_string());

        lines.join("\n")
    }

    /// Generate README.md
    fn generate_readme(&self, graph: &ProjectGraph) -> String {
        format!(
            r#"# {}

{}

Generated by [Immortal Engine](https://github.com/yourusername/imortal_engine)

## Getting Started

### Prerequisites

- Rust 1.70+
- {} database

### Setup

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your configuration

3. Run database migrations:
   ```bash
   sqlx migrate run
   ```

4. Start the server:
   ```bash
   cargo run
   ```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Library root with app setup
├── config.rs        # Configuration management
├── error.rs         # Error types
├── models/          # Data models
├── handlers/        # API handlers
├── routes.rs        # Route configuration
└── auth/            # Authentication (if enabled)
```

## API Endpoints

TODO: Document your API endpoints here

## Development

```bash
# Run with hot reload
cargo watch -x run

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## License

MIT
"#,
            graph.meta.name,
            graph.meta.description.as_deref().unwrap_or("A generated application"),
            match self.config.database_backend {
                DatabaseBackend::Postgres => "PostgreSQL",
                DatabaseBackend::Sqlite => "SQLite",
                DatabaseBackend::Mysql => "MySQL",
            },
        )
    }

    /// Write the generated project to disk
    pub fn write_to_disk(&self, project: &GeneratedProject, output_dir: impl AsRef<Path>) -> EngineResult<()> {
        let output_dir = output_dir.as_ref();

        for (path, content) in &project.files {
            let file_path = output_dir.join(path);

            // Create parent directories
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write file
            std::fs::write(&file_path, content)?;
        }

        Ok(())
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for code generation
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Target language (default: "rust")
    pub target_language: String,
    /// Target web framework
    pub auth_framework: AuthFramework,
    /// Target database backend
    pub database_backend: DatabaseBackend,
    /// Output directory
    pub output_dir: PathBuf,
    /// Whether to generate tests
    pub generate_tests: bool,
    /// Whether to generate documentation
    pub generate_docs: bool,
    /// Whether to generate database migrations
    pub generate_migrations: bool,
    /// Whether to format generated code
    pub format_code: bool,
    /// Custom options
    pub options: HashMap<String, ConfigValue>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            target_language: "rust".to_string(),
            auth_framework: AuthFramework::Axum,
            database_backend: DatabaseBackend::Postgres,
            output_dir: PathBuf::from("generated"),
            generate_tests: true,
            generate_docs: true,
            generate_migrations: true,
            format_code: true,
            options: HashMap::new(),
        }
    }
}

impl GeneratorConfig {
    /// Create a new configuration for Rust generation with Axum
    pub fn axum() -> Self {
        Self::default()
    }

    /// Create a new configuration for Rust generation with Actix
    pub fn actix() -> Self {
        Self {
            auth_framework: AuthFramework::Actix,
            ..Default::default()
        }
    }

    /// Set the target framework
    pub fn with_framework(mut self, framework: AuthFramework) -> Self {
        self.auth_framework = framework;
        self
    }

    /// Set the database backend
    pub fn with_database(mut self, backend: DatabaseBackend) -> Self {
        self.database_backend = backend;
        self
    }

    /// Set the output directory
    pub fn with_output_dir(mut self, dir: impl Into<PathBuf>) -> Self {
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

    /// Disable migration generation
    pub fn without_migrations(mut self) -> Self {
        self.generate_migrations = false;
        self
    }

    /// Add a custom option
    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }
}

/// A generated project containing all generated files
#[derive(Debug, Clone)]
pub struct GeneratedProject {
    /// Project name
    pub name: String,
    /// Generated files (path -> content)
    pub files: HashMap<String, String>,
    /// Any warnings generated during generation
    pub warnings: Vec<String>,
}

impl GeneratedProject {
    /// Create a new empty generated project
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            files: HashMap::new(),
            warnings: Vec::new(),
        }
    }

    /// Add a file to the project
    pub fn add_file(&mut self, path: impl Into<String>, content: impl Into<String>) {
        self.files.insert(path.into(), content.into());
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Get the number of generated files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Check if the project has warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get a file's content
    pub fn get_file(&self, path: &str) -> Option<&String> {
        self.files.get(path)
    }

    /// Get all file paths
    pub fn file_paths(&self) -> impl Iterator<Item = &String> {
        self.files.keys()
    }

    /// Get files by extension
    pub fn files_with_extension(&self, ext: &str) -> Vec<(&String, &String)> {
        self.files
            .iter()
            .filter(|(path, _)| path.ends_with(ext))
            .collect()
    }

    /// Merge another generated project into this one
    pub fn merge(&mut self, other: GeneratedProject) {
        for (path, content) in other.files {
            self.files.insert(path, content);
        }
        self.warnings.extend(other.warnings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use imortal_ir::{ProjectMeta, Node};

    #[test]
    fn test_generator_creation() {
        let generator = CodeGenerator::new();
        assert_eq!(generator.config.target_language, "rust");
    }

    #[test]
    fn test_generator_config() {
        let config = GeneratorConfig::axum()
            .with_database(DatabaseBackend::Postgres)
            .with_output_dir("output")
            .without_tests();

        assert_eq!(config.auth_framework, AuthFramework::Axum);
        assert_eq!(config.database_backend, DatabaseBackend::Postgres);
        assert_eq!(config.output_dir, PathBuf::from("output"));
        assert!(!config.generate_tests);
    }

    #[test]
    fn test_generated_project() {
        let mut project = GeneratedProject::new("test_project");
        project.add_file("src/main.rs", "fn main() {}");
        project.add_file("Cargo.toml", "[package]");

        assert_eq!(project.file_count(), 2);
        assert!(project.get_file("src/main.rs").is_some());
    }

    #[test]
    fn test_generate_simple_project() {
        let meta = ProjectMeta::new("test_app");
        let graph = ProjectGraph::new(meta);

        let generator = CodeGenerator::new();
        let result = generator.generate(&graph);

        assert!(result.is_ok());
        let project = result.unwrap();
        assert!(project.get_file("Cargo.toml").is_some());
        assert!(project.get_file("src/main.rs").is_some());
    }

    #[test]
    fn test_files_with_extension() {
        let mut project = GeneratedProject::new("test");
        project.add_file("src/main.rs", "");
        project.add_file("src/lib.rs", "");
        project.add_file("Cargo.toml", "");

        let rs_files = project.files_with_extension(".rs");
        assert_eq!(rs_files.len(), 2);
    }

    #[test]
    fn test_project_merge() {
        let mut project1 = GeneratedProject::new("test1");
        project1.add_file("a.rs", "a");

        let mut project2 = GeneratedProject::new("test2");
        project2.add_file("b.rs", "b");
        project2.add_warning("warning");

        project1.merge(project2);

        assert_eq!(project1.file_count(), 2);
        assert!(project1.has_warnings());
    }
}
