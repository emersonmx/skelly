mod actions;
mod adapters;
mod cli;
mod config;
mod renderer;
mod usecases;
mod validation;

use std::io::IsTerminal;

use clap::Parser;

fn main() {
    let response = actions::handle(
        cli::Args::parse(),
        std::io::stdin().is_terminal(),
        std::io::stdout().is_terminal(),
    );

    if response.is_err() {
        std::process::exit(1);
    }
}
