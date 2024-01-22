use crate::{adapters, cli, config, usecases};

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
    let cleaned_inputs = adapters::clean_inputs(&args.inputs, &config.inputs)?;

    usecases::render_skeleton::execute(
        adapters::file_finder(&config.template_directory),
        |path| {
            adapters::file_reader(
                path,
                &cleaned_inputs,
                &config.template_directory,
            )
        },
        |path, content| {
            adapters::file_writer(path, &content, &args.output_path)
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
    eprintln!("args = {:?}", args);
    eprintln!("config = {:?}", config);
    Ok(())
}

pub fn skeleton_and_stdin_error() -> Result<(), String> {
    let msg = "Unable to decide between skeleton and standard input.";
    eprintln!("{msg}");
    Err(msg)?
}

pub fn stdin_to_stdout(args: &cli::Args) -> Result<(), String> {
    eprintln!("args = {:?}", args);
    Ok(())
}
