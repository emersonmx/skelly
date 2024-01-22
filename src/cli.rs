use crate::config::Config;
use clap::Parser;
use std::error::Error;
use std::path::Path;
use std::{fs::create_dir_all, path::PathBuf};

const CONFIG_NAME: &str = "skelly.toml";

fn parse_skeleton_config(value: &str) -> Result<Config, String> {
    let path = Path::new(value);
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

    Config::from_file(&config_path)
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

fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
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
    #[arg(value_parser = parse_key_val::<String, String>)]
    pub inputs: Vec<(String, String)>,
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
