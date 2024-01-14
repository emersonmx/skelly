use crate::config::Config;
use clap::Parser;
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
    str::FromStr,
};

const CONFIG_NAME: &str = "skelly.toml";

pub type Input = (String, String);

fn parse_skeleton_config(value: &str) -> Result<Config, String> {
    let path = PathBuf::from(value);
    if !path.is_dir() {
        return Err(format!("'{value}' is not a directory."));
    }

    let skeleton_path = path
        .canonicalize()
        .or(Err(format!("unable to resolve path '{value}'.")))?;

    let config_path = skeleton_path.join(CONFIG_NAME);
    if !config_path.exists() {
        return Err(format!(
            "config '{}' does not exist.",
            config_path.display()
        ));
    }

    let config_content = fs::read_to_string(&config_path).or(Err(format!(
        "unable to read config '{}.",
        &config_path.display()
    )))?;

    Config::from_str(&config_content)
        .or(Err("unable to parse config.".to_string()))
}

fn parse_output_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if path.exists() {
        if !path.is_dir() {
            return Err(format!("'{value}' is not a directory."));
        }
    } else {
        create_dir_all(&path)
            .or(Err(format!("unable to create directory '{value}'.")))?;
    }

    path.canonicalize().or(Err(format!("unable to resolve path '{value}'.")))
}

fn parse_input(value: &str) -> Result<Input, String> {
    let (name, value) = value.split_once('=').ok_or("missing '=' in input.")?;
    Ok((name.to_owned(), value.to_owned()))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Which skeleton to use
    #[arg(
        short('s'),
        long("skeleton-path"),
        value_name = "DIRECTORY",
        value_hint = clap::ValueHint::DirPath,
        value_parser = parse_skeleton_config
    )]
    pub skeleton_config: Option<Config>,

    /// Where to output the generated skeleton into
    #[arg(
        short,
        long,
        value_name = "DIRECTORY",
        default_value = ".",
        value_hint = clap::ValueHint::DirPath,
        value_parser = parse_output_path,
    )]
    pub output_path: PathBuf,

    /// Inputs passed to the skeleton
    #[arg(value_parser = parse_input)]
    pub inputs: Vec<Input>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
