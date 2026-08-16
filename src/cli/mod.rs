pub mod args;
pub mod commands;
pub mod output;

use crate::cli::args::{Args, Commands};
use clap::Parser;

/// Dispatches to the CLI commands supported by the single binary.
pub fn run() {
    let cli = Args::parse();
    match cli.command {
        Commands::Generate { prompt, max_tokens } => {
            println!("generate: prompt={prompt} max_tokens={max_tokens}");
        }
        Commands::Chat { prompt } => {
            println!("chat: {prompt}");
        }
        Commands::Serve { port } => {
            println!("serve: port={port}");
        }
        Commands::Model { action } => {
            println!("model: {action}");
        }
        Commands::Cache { action } => {
            println!("cache: {action}");
        }
        Commands::Benchmark { mode } => {
            println!("benchmark: {mode}");
        }
    }
}
