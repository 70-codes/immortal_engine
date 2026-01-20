//! Immortal Engine - Visual Prototyping System
//!
//! This is the main entry point for the Immortal Engine desktop application.
//! It launches the visual editor where users can create applications by
//! dragging components and drawing connections between them.

use std::env;
use std::path::PathBuf;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "imortal=info,eframe=warn".into()),
        )
        .init();

    tracing::info!("Starting Immortal Engine...");

    // Check for command line arguments
    let args: Vec<String> = env::args().collect();

    // If a project file is provided, open it
    if args.len() > 1 {
        let project_path = &args[1];
        tracing::info!("Opening project: {}", project_path);

        match imortal_ir::load_project(project_path) {
            Ok(project) => {
                tracing::info!(
                    "Loaded project '{}' with {} nodes",
                    project.meta.name,
                    project.node_count()
                );

                let path = PathBuf::from(project_path);
                if let Err(e) = imortal_ui::run_with_project_path(project, path) {
                    tracing::error!("Failed to run UI: {}", e);
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                tracing::error!("Failed to load project: {}", e);
                eprintln!("Error loading project '{}': {}", project_path, e);
                std::process::exit(1);
            }
        }
    } else {
        // Start with a new/empty project
        tracing::info!("Starting with new project");

        if let Err(e) = imortal_ui::run() {
            tracing::error!("Failed to run UI: {}", e);
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    tracing::info!("Immortal Engine closed");
}
