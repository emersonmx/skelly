use crate::{adapters, cli, config, usecases};

pub fn render_skeleton(
    args: &cli::Args,
    config: &config::Config,
) -> Result<(), String> {
    let cleaned_inputs = adapters::clean_inputs(&args.inputs, &config.inputs)?;

    usecases::render_skeleton::execute(
        adapters::file_finder(&config.template_directory),
        |path| {
            adapters::walk_dir_file_reader(
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

pub fn skeleton_and_stdin() -> Result<(), String> {
    let msg = "Unable to decide between skeleton and standard input.";
    eprintln!("{msg}");
    Err(msg)?
}
