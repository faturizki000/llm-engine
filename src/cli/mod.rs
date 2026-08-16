pub mod args;
pub mod commands;
pub mod output;

use crate::cli::args::{Args, Commands};
use crate::{LocalRuntime, RuntimeConfig};
use clap::Parser;

/// Dispatches to the CLI commands supported by the single binary.
pub fn run() {
    let cli = Args::parse();
    match cli.command {
        Commands::Generate { prompt, max_tokens } => {
            let runtime = LocalRuntime::new(RuntimeConfig::default());
            let output = runtime.generate(&prompt, max_tokens);
            println!("{output}");
        }
        Commands::Chat { prompt } => {
            let runtime = LocalRuntime::new(RuntimeConfig::default());
            println!("chat: {}", runtime.generate(&prompt, 12));
        }
        Commands::Serve { port } => {
            println!("serve: port={port} mode=offline");
        }
        Commands::Model { action } => {
            println!("model: action={action} source=local");
        }
        Commands::Cache { action } => {
            println!("cache: action={action} backend=exact-local");
        }
        Commands::Benchmark { mode } => {
            let scheduler = match mode.as_str() {
                "throughput" => "throughput",
                _ => "latency",
            };
            println!("benchmark: mode={scheduler} offline=true");
        }
    }
}
