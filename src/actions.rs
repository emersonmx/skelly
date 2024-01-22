use crate::{cli, config, usecases, validation};

pub fn render_skeleton(
    args: &cli::Args,
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

pub fn skeleton_to_stdout(
    args: &cli::Args,
    config: &config::Config,
) -> Result<(), String> {
    eprintln!("args = {:?}", args);
    eprintln!("config = {:?}", config);
    Ok(())
}

pub fn skeleton_and_stdin() -> Result<(), String> {
    let msg = "Unable to decide between skeleton and standard input.";
    eprintln!("{msg}");
    Err(msg)?
}
