use clap::Parser;
use std::io::IsTerminal;

mod actions;
mod adapters;
mod cli;
mod config;
mod renderer;
mod usecases;
mod validation;

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
