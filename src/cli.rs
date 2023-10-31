use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Input(String, String);

fn check_skeleton_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if !path.is_dir() {
        return Err(format!("'{value}' is not a directory."));
    }

    if !path.join("skelly.toml").exists() {
        return Err("skelly.toml not exists.".to_string());
    }

    path.canonicalize().or(Err(format!("unable to resolve path '{value}'.")))
}

fn parse_input(value: &str) -> Result<Input, String> {
    let (name, value) = value.split_once("=").ok_or("missing '=' in input.")?;
    Ok(Input(name.to_owned(), value.to_owned()))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(value_parser = check_skeleton_path)]
    pub skeleton_path: PathBuf,

    #[arg(value_parser = parse_input)]
    pub inputs: Vec<Input>,
}

pub fn get_args() -> Args {
    Args::parse()
}
