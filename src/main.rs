mod cli;
mod config;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    eprintln!("args = {:?}", args);
}
