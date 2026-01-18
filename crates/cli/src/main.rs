//! Immortal Engine CLI
//!
//! Command-line interface for the Immortal Engine prototyping system.

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Immortal Engine - Visual Prototyping System
#[derive(Parser)]
#[command(name = "imortal")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Immortal Engine project
    New {
        /// Project name
        name: String,

        /// Directory to create the project in
        #[arg(short, long)]
        path: Option<String>,

        /// Project template to use
        #[arg(short, long, default_value = "default")]
        template: String,
    },

    /// Open the visual editor
    Editor {
        /// Project file to open
        #[arg(short, long)]
        project: Option<String>,

        /// Port for the editor server
        #[arg(long, default_value = "3000")]
        port: u16,
    },

    /// Generate code from a project file
    Generate {
        /// Project file to generate from
        project: String,

        /// Output directory
        #[arg(short, long, default_value = "generated")]
        output: String,

        /// Target language
        #[arg(short, long, default_value = "rust")]
        target: String,

        /// Watch for changes and regenerate
        #[arg(short, long)]
        watch: bool,
    },

    /// Validate a project file
    Validate {
        /// Project file to validate
        project: String,

        /// Output format for validation results
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// List available components
    Components {
        /// Filter by category
        #[arg(short = 'C', long)]
        category: Option<String>,

        /// Search query
        #[arg(short, long)]
        search: Option<String>,
    },

    /// Export a project to different formats
    Export {
        /// Project file to export
        project: String,

        /// Output file path
        output: String,

        /// Export format (json, toml, yaml)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Import a project from external formats
    Import {
        /// Input file to import
        input: String,

        /// Output project file
        #[arg(short, long)]
        output: Option<String>,

        /// Input format (json, toml, openapi, prisma)
        #[arg(short, long)]
        format: Option<String>,
    },

    /// Show information about the engine
    Info,
}

fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "imortal=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    if cli.verbose {
        tracing::info!("Verbose mode enabled");
    }

    match cli.command {
        Commands::New { name, path, template } => {
            cmd_new(&name, path.as_deref(), &template)?;
        }
        Commands::Editor { project, port } => {
            cmd_editor(project.as_deref(), port)?;
        }
        Commands::Generate { project, output, target, watch } => {
            cmd_generate(&project, &output, &target, watch)?;
        }
        Commands::Validate { project, format } => {
            cmd_validate(&project, &format)?;
        }
        Commands::Components { category, search } => {
            cmd_components(category.as_deref(), search.as_deref())?;
        }
        Commands::Export { project, output, format } => {
            cmd_export(&project, &output, &format)?;
        }
        Commands::Import { input, output, format } => {
            cmd_import(&input, output.as_deref(), format.as_deref())?;
        }
        Commands::Info => {
            cmd_info()?;
        }
    }

    Ok(())
}

fn cmd_new(name: &str, path: Option<&str>, template: &str) -> Result<()> {
    use imortal_ir::{ProjectGraph, ProjectMeta, save_project, ProjectFormat};
    use std::path::PathBuf;

    let project_dir = path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(name));

    println!("📦 Creating new project: {}", name);
    println!("   Template: {}", template);
    println!("   Location: {}", project_dir.display());

    // Create project directory
    std::fs::create_dir_all(&project_dir)?;

    // Create project metadata
    let meta = ProjectMeta::new(name)
        .with_description(format!("Created with Immortal Engine using '{}' template", template))
        .with_version("0.1.0");

    // Create empty project graph
    let project = ProjectGraph::new(meta);

    // Save project file
    let project_file = project_dir.join(format!("{}.imortal", name));
    save_project(&project, &project_file, ProjectFormat::Json)?;

    println!("✅ Project created successfully!");
    println!("\n   To open the editor, run:");
    println!("   imortal editor --project {}", project_file.display());

    Ok(())
}

fn cmd_editor(_project: Option<&str>, port: u16) -> Result<()> {
    println!("🚀 Starting Immortal Engine Editor on port {}...", port);
    println!("\n   Note: The visual editor is not yet implemented.");
    println!("   This will launch the egui-based visual editor in a future release.");

    // TODO: Launch the UI application
    // For now, just print a message

    Ok(())
}

fn cmd_generate(project: &str, output: &str, target: &str, watch: bool) -> Result<()> {
    use imortal_ir::load_project;

    println!("⚙️  Generating code from: {}", project);
    println!("   Output: {}", output);
    println!("   Target: {}", target);

    // Load the project
    let graph = load_project(project)?;

    println!("   Loaded {} nodes and {} edges", graph.node_count(), graph.edge_count());

    // TODO: Implement actual code generation
    println!("\n   Note: Code generation is not yet implemented.");
    println!("   This will generate {} code in a future release.", target);

    if watch {
        println!("\n   Watch mode enabled - would watch for changes...");
    }

    Ok(())
}

fn cmd_validate(project: &str, format: &str) -> Result<()> {
    use imortal_ir::{load_project, validation};

    println!("🔍 Validating project: {}", project);

    // Load the project
    let graph = load_project(project)?;

    // Run validation
    let issues = validation::get_all_issues(&graph);

    let errors: Vec<_> = issues.iter().filter(|i| i.is_error()).collect();
    let warnings: Vec<_> = issues.iter().filter(|i| i.is_warning()).collect();

    match format {
        "json" => {
            // TODO: JSON output
            println!("{{\"errors\": {}, \"warnings\": {}}}", errors.len(), warnings.len());
        }
        _ => {
            println!("\n   Nodes: {}", graph.node_count());
            println!("   Edges: {}", graph.edge_count());
            println!("   Groups: {}", graph.group_count());
            println!();

            if errors.is_empty() && warnings.is_empty() {
                println!("✅ No issues found!");
            } else {
                for error in &errors {
                    println!("❌ {}", error);
                }
                for warning in &warnings {
                    println!("⚠️  {}", warning);
                }
                println!();
                println!("   {} error(s), {} warning(s)", errors.len(), warnings.len());
            }
        }
    }

    Ok(())
}

fn cmd_components(category: Option<&str>, search: Option<&str>) -> Result<()> {
    use imortal_components::{ComponentRegistry, ComponentCategory};

    let registry = ComponentRegistry::with_builtins();

    println!("📦 Available Components\n");

    let components: Vec<_> = if let Some(cat_str) = category {
        // Filter by category
        let cat = match cat_str.to_lowercase().as_str() {
            "auth" => Some(ComponentCategory::Auth),
            "data" => Some(ComponentCategory::Data),
            "api" => Some(ComponentCategory::Api),
            "storage" => Some(ComponentCategory::Storage),
            "logic" => Some(ComponentCategory::Logic),
            "ui" => Some(ComponentCategory::Ui),
            "embedded" => Some(ComponentCategory::Embedded),
            "custom" => Some(ComponentCategory::Custom),
            _ => None,
        };

        if let Some(c) = cat {
            registry.by_category(c)
        } else {
            println!("Unknown category: {}", cat_str);
            return Ok(());
        }
    } else if let Some(query) = search {
        registry.search(query)
    } else {
        registry.all().collect()
    };

    // Group by category
    let mut by_category: std::collections::HashMap<ComponentCategory, Vec<_>> =
        std::collections::HashMap::new();

    for comp in components {
        by_category
            .entry(comp.category)
            .or_insert_with(Vec::new)
            .push(comp);
    }

    for category in ComponentCategory::all() {
        if let Some(comps) = by_category.get(category) {
            if !comps.is_empty() {
                println!("{} {}", category.icon(), category.display_name());
                for comp in comps {
                    println!("   {} {} - {}", comp.icon, comp.name, comp.description);
                    println!("      ID: {}", comp.id);
                }
                println!();
            }
        }
    }

    let stats = registry.stats();
    println!("Total: {} components", stats.total);

    Ok(())
}

fn cmd_export(project: &str, output: &str, format: &str) -> Result<()> {
    use imortal_ir::{load_project, save_project, ProjectFormat};

    println!("📤 Exporting project: {}", project);
    println!("   Output: {}", output);
    println!("   Format: {}", format);

    let graph = load_project(project)?;

    let fmt = match format.to_lowercase().as_str() {
        "json" => ProjectFormat::Json,
        "json-compact" => ProjectFormat::JsonCompact,
        "toml" => ProjectFormat::Toml,
        _ => {
            println!("Unknown format: {}. Using JSON.", format);
            ProjectFormat::Json
        }
    };

    save_project(&graph, output, fmt)?;

    println!("✅ Exported successfully!");

    Ok(())
}

fn cmd_import(input: &str, output: Option<&str>, format: Option<&str>) -> Result<()> {
    println!("📥 Importing from: {}", input);
    println!("   Format: {}", format.unwrap_or("auto-detect"));

    // TODO: Implement import from various formats (OpenAPI, Prisma schema, etc.)
    println!("\n   Note: Import is not yet fully implemented.");
    println!("   Supported formats will include: JSON, TOML, OpenAPI, Prisma");

    if let Some(out) = output {
        println!("   Would output to: {}", out);
    }

    Ok(())
}

fn cmd_info() -> Result<()> {
    use imortal_components::ComponentRegistry;

    println!("🔧 Immortal Engine\n");
    println!("   Version: {}", env!("CARGO_PKG_VERSION"));
    println!("   IR Version: {}", imortal_ir::IR_VERSION);
    println!("   Component Version: {}", imortal_components::COMPONENT_VERSION);
    println!();

    let registry = ComponentRegistry::with_builtins();
    let stats = registry.stats();

    println!("📦 Built-in Components: {}", stats.total);
    for category in imortal_core::ComponentCategory::all() {
        let count = stats.category_count(*category);
        if count > 0 {
            println!("   {} {}: {}", category.icon(), category.display_name(), count);
        }
    }
    println!();

    println!("🌐 Project Home: https://github.com/70-codes/immortal_engine");
    println!("📖 Documentation: https://docs.imortal-engine.dev");

    Ok(())
}
