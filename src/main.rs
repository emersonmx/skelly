pub mod actions;
pub mod adapters;
pub mod cli;
pub mod config;
pub mod renderer;
pub mod usecases;
pub mod validation;

use std::io::IsTerminal;

use clap::Parser;
use cli::Args;

fn main() {
    let response = handle_actions(
        Args::parse(),
        std::io::stdin().is_terminal(),
        std::io::stdout().is_terminal(),
    );

    if response.is_err() {
        std::process::exit(1);
    }
}

fn handle_actions(
    args: Args,
    use_input_terminal: bool,
    use_output_terminal: bool,
) -> Result<(), String> {
    match (&args, use_input_terminal, use_output_terminal) {
        (Args { skeleton_config: Some(skeleton_config), .. }, true, true) => {
            actions::render_skeleton(&args, skeleton_config)?
        }
        (Args { skeleton_config: Some(skeleton_config), .. }, true, false) => {
            actions::skeleton_to_stdout(&args, skeleton_config)?
        }
        (Args { skeleton_config: Some(_), .. }, false, true) => {
            actions::skeleton_and_stdin_error()?
        }
        (Args { skeleton_config: None, .. }, _, _) => {
            actions::stdin_to_stdout()?
        }
        _ => println!("WAT?!"),
    }

    Ok(())
}
