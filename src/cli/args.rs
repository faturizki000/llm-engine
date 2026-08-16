use clap::{Parser, Subcommand, ValueEnum};

/// llm-engine CLI arguments.
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Generate {
        #[arg(long)]
        prompt: String,
        #[arg(long, default_value_t = 16)]
        max_tokens: usize,
    },
    Chat {
        #[arg(long)]
        prompt: String,
    },
    Serve {
        #[arg(long, default_value_t = 3000)]
        port: u16,
    },
    Model {
        #[arg(long)]
        action: String,
    },
    Cache {
        #[arg(long)]
        action: String,
    },
    Benchmark {
        #[arg(long, default_value = "latency")]
        mode: String,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum BenchmarkMode {
    Latency,
    Throughput,
    Memory,
}
