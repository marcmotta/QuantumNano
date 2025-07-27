// src/main.rs
/*
 * Main executable for QuantumNano
 */

use clap::Parser;
use quantumnano::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumNano - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
