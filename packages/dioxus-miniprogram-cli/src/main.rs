//! Dioxus Mini Program CLI
//! 
//! A command-line tool for building Dioxus applications targeting WeChat Mini Program.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod config;
mod new_project;

#[derive(Parser)]
#[command(name = "dx-miniprogram")]
#[command(about = "Build Dioxus apps for WeChat Mini Program", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Set the project directory
    #[arg(short, long, default_value = ".")]
    project_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Target directory for build output
        #[arg(short, long)]
        target_dir: Option<PathBuf>,

        /// Use Worker mode (recommended)
        #[arg(short, long, default_value = "true")]
        worker: bool,
    },
    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Template to use
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    /// Initialize a new project in an existing directory
    Init,
    /// Serve the project for development
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
    },
    /// Generate Mini Program page files
    GeneratePage {
        /// Page name
        name: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter("info")
            .init();
    }

    match cli.command {
        Commands::Build { release, target_dir, worker } => {
            build::build_project(cli.project_dir, release, target_dir, worker)?;
        }
        Commands::New { name, template } => {
            new_project::create_project(&name, &template)?;
        }
        Commands::Init => {
            new_project::init_project(cli.project_dir)?;
        }
        Commands::Serve { port, open } => {
            build::serve_project(cli.project_dir, port, open)?;
        }
        Commands::GeneratePage { name, output } => {
            let output_dir = output.unwrap_or_else(|| cli.project_dir.join("pages"));
            build::generate_page(&name, output_dir)?;
        }
    }

    Ok(())
}
