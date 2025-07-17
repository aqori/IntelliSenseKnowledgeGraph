// src/main.rs
/*
 * Main executable for IntelliSenseKnowledgeGraph
 */

use clap::Parser;
use intellisenseknowledgegraph::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliSenseKnowledgeGraph - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
