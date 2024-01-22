use crate::{adapters, cli, config, usecases, validation};

pub fn handle(
    args: cli::Args,
    use_input_terminal: bool,
    use_output_terminal: bool,
) -> Result<(), String> {
    match (&args, use_input_terminal, use_output_terminal) {
        (
            cli::Args { skeleton_config: Some(skeleton_config), .. },
            true,
            true,
        ) => render_skeleton(&args, skeleton_config)?,
        (
            cli::Args { skeleton_config: Some(skeleton_config), .. },
            true,
            false,
        ) => skeleton_to_stdout(&args, skeleton_config)?,
        (cli::Args { skeleton_config: Some(_), .. }, false, _) => {
            skeleton_and_stdin_error()?
        }
        (cli::Args { skeleton_config: None, .. }, _, _) => {
            stdin_to_stdout(&args)?
        }
    }

    Ok(())
}

pub fn render_skeleton(
    args: &cli::Args,
    config: &config::Config,
) -> Result<(), String> {
    let cleaned_inputs = clean_inputs(&args.inputs, &config.inputs)?;

    usecases::render_skeleton::execute(
        adapters::file_finder(&config.template_directory),
        |path| {
            adapters::file_reader(
                path,
                &cleaned_inputs,
                &config.template_directory,
            )
            .map_err(|e| usecases::render_skeleton::Error(e.to_string()))
        },
        |path, content| {
            adapters::file_writer(path, &content, &args.output_path)
                .map_err(|e| usecases::render_skeleton::Error(e.to_string()))
        },
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
    let cleaned_inputs = clean_inputs(&args.inputs, &config.inputs)?;

    usecases::render_skeleton::execute(
        adapters::file_finder(&config.template_directory),
        |path| {
            adapters::file_reader(
                path,
                &cleaned_inputs,
                &config.template_directory,
            )
            .map_err(|e| usecases::render_skeleton::Error(e.to_string()))
        },
        |_, content| {
            adapters::text_writer(content);
            Ok(())
        },
    )
    .map_err(|error| {
        eprintln!("{}", error.0);
        error.to_string()
    })?;

    Ok(())
}

pub fn skeleton_and_stdin_error() -> Result<(), String> {
    let msg = "Unable to decide between skeleton and standard input.";
    eprintln!("{msg}");
    Err(msg)?
}

pub fn stdin_to_stdout(args: &cli::Args) -> Result<(), String> {
    usecases::render_text::execute(
        || {
            let text = adapters::text_reader(&args.inputs)
                .map_err(|e| usecases::render_text::Error(e.to_string()))?;
            Ok(text)
        },
        |content| {
            adapters::text_writer(content);
            Ok(())
        },
    )
    .map_err(|error| {
        eprintln!("{}", error.0);
        error.to_string()
    })?;
    Ok(())
}

fn clean_inputs(
    user_inputs: &[(String, String)],
    config_inputs: &[config::Input],
) -> Result<Vec<(String, String)>, String> {
    let inputs = validation::validate_inputs(user_inputs, config_inputs)
        .map_err(|error| {
            let errors = error.0.iter().fold(String::new(), |acc, e| {
                let msg = match e {
                    validation::ErrorType::MissingInput(name) => {
                        format!("Missing input '{}'.", name)
                    }
                    validation::ErrorType::InvalidOption(key, value) => {
                        format!(
                            "Invalid option '{}' to input '{}'.",
                            value, key
                        )
                    }
                };
                format!("{}{}\n", acc, msg)
            });

            eprint!("{errors}");

            errors
        })?;

    Ok(inputs)
}
