use clap::Parser;
use std::{fs::create_dir_all, path::PathBuf};

pub type Input = (String, String);

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

fn parse_skeleton_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if !path.is_dir() {
        return Err(format!("'{value}' is not a directory."));
    }

    path.canonicalize().or(Err(format!("unable to resolve path '{value}'.")))
}

fn parse_input(value: &str) -> Result<Input, String> {
    let (name, value) = value.split_once('=').ok_or("missing '=' in input.")?;
    Ok((name.to_owned(), value.to_owned()))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Where to output the generated skeleton into
    #[arg(
        short,
        long,
        value_name = "DIRECTORY",
        default_value = ".",
        value_parser = parse_output_path
    )]
    pub output_path: PathBuf,

    #[arg(short, long, value_name = "DIRECTORY", value_parser = parse_skeleton_path)]
    pub skeleton_path: Option<PathBuf>,

    #[arg(value_parser = parse_input)]
    pub inputs: Vec<Input>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
