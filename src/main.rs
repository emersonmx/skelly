pub mod cli;
pub mod config;
pub mod renderer;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    eprintln!("args = {:?}", args);
}
