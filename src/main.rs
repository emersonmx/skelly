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
            render_skeleton_action(&args, skeleton_config)?
        }
        (Args { skeleton_config: Some(_), .. }, false, true) => {
            skeleton_and_stdin_action()?
        }
        _ => println!("WAT?!"),
    }

    Ok(())
}

fn render_skeleton_action(
    args: &Args,
    config: &config::Config,
) -> Result<(), String> {
    let cleaned_inputs = validation::validate_inputs(
        &args.inputs,
        &config.inputs,
    )
    .map_err(|error| {
        let errors = error.0.iter().fold(String::new(), |acc, e| {
            let msg = match e {
                validation::ErrorType::MissingInput(name) => {
                    format!("Missing input '{}'.", name)
                }
                validation::ErrorType::InvalidOption(key, value) => {
                    format!("Invalid option '{}' to input '{}'.", value, key)
                }
            };
            format!("{}{}\n", acc, msg)
        });

        eprint!("{errors}");

        errors
    })?;
    usecases::render_skeleton::execute(
        &config.template_directory,
        &cleaned_inputs,
        &args.output_path,
    )
    .map_err(|error| {
        eprintln!("{}", error.0);
        error.to_string()
    })?;

    Ok(())
}

fn skeleton_and_stdin_action() -> Result<(), String> {
    let msg = "Unable to decide between skeleton and standard input.";
    eprintln!("{msg}");
    Err(msg)?
}
