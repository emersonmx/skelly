use crate::config::Config;
use clap::Parser;
use std::error::Error;
use std::path::Path;
use std::{fs::create_dir_all, path::PathBuf};
use walkdir::WalkDir;

const CONFIG_NAME: &str = "skelly.toml";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Which skeleton to use
    #[arg(
        short('s'),
        long("skeleton-path"),
        value_name = "DIRECTORY",
        value_hint = clap::ValueHint::DirPath,
        value_parser = parse_skeleton_config,
        conflicts_with_all = ["library"],
    )]
    pub skeleton_config: Option<Config>,

    /// Which file to use
    #[arg(
        short,
        long("file-path"),
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        value_parser = parse_file_path,
        conflicts_with_all = ["skeleton_config", "output_path"],
    )]
    pub file_path: Option<PathBuf>,

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

    /// Templates available to the main template
    #[arg(
        short,
        long("library"),
        value_name = "PATH | NAME=PATH",
        value_parser = parse_library,
    )]
    pub library: Vec<Vec<(String, String)>>,

    /// Inputs passed to the skeleton
    #[arg(value_parser = parse_key_val::<String, String>)]
    pub inputs: Vec<(String, String)>,
}

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

fn parse_file_path(value: &str) -> Result<PathBuf, String> {
    let path = Path::new(value);
    if !path.is_file() {
        return Err(format!("'{value}' is not a file."));
    }

    let abspath = path
        .canonicalize()
        .or(Err(format!("unable to resolve path '{value}'.")))?;

    if !abspath.exists() {
        return Err(format!("file '{}' does not exist.", abspath.display()));
    }

    Ok(abspath)
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

fn parse_library(value: &str) -> Result<Vec<(String, String)>, String> {
    let path = Path::new(value);

    if path.is_file() {
        let library_file = load_library_file(path, &get_file_name)?;
        return Ok(vec![library_file]);
    }

    if path.is_dir() {
        let directory_name = get_directory_name(path)?;
        let directory_name = PathBuf::from(directory_name);
        return load_library_directory(path, &|file_path| {
            let relative_file_path = get_relative_path(path, file_path)?;
            let template_name =
                path_to_string(&directory_name.join(&relative_file_path))?;
            Ok(template_name)
        });
    }

    if let Ok((key, value)) = parse_key_val::<String, String>(value) {
        let path = Path::new(&value);
        if path.is_file() {
            let library_file = load_library_file(path, &|_| Ok(key.clone()))?;
            return Ok(vec![library_file]);
        }

        if path.is_dir() {
            let directory_name = Path::new(&key).to_owned();
            return load_library_directory(path, &|file_path| {
                let relative_file_path = get_relative_path(path, file_path)?;
                let template_name =
                    path_to_string(&directory_name.join(&relative_file_path))?;
                Ok(template_name)
            });
        }
    }

    Ok(vec![])
}

fn get_file_name(path: &Path) -> Result<String, String> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("unable to get file name from '{path:?}'."))?
        .to_string();
    Ok(file_name)
}

fn get_directory_name(path: &Path) -> Result<String, String> {
    let directory_name =
        path.file_name().and_then(|name| name.to_str()).ok_or_else(|| {
            format!("unable to get directory name from '{path:?}'.")
        })?;
    Ok(directory_name.to_string())
}

fn get_relative_path(
    base_path: &Path,
    file_path: &Path,
) -> Result<PathBuf, String> {
    let relative_path = file_path
        .strip_prefix(base_path)
        .map_err(|_| {
            format!(
                "unable to get relative path from '{file_path:?}' and '{base_path:?}'."
            )
        })?
        .to_owned();
    Ok(relative_path)
}

fn path_to_string(path: &Path) -> Result<String, String> {
    let path_str = path.to_str().ok_or_else(|| {
        format!("unable to convert path '{}' to string.", path.display())
    })?;
    Ok(path_str.to_string())
}

fn load_file_content(path: &Path) -> Result<String, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|_| format!("unable to read file '{}'.", path.display()))?;
    Ok(content)
}

fn load_library_file(
    path: &Path,
    template_name_fn: &dyn Fn(&Path) -> Result<String, String>,
) -> Result<(String, String), String> {
    let template_name = template_name_fn(path)?;
    let content = load_file_content(path)?;

    Ok((template_name, content))
}

fn load_library_directory(
    path: &Path,
    template_name_fn: &dyn Fn(&Path) -> Result<String, String>,
) -> Result<Vec<(String, String)>, String> {
    let mut library_files = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();

        let template_name = template_name_fn(file_path)?;
        let content = load_file_content(file_path)?;

        library_files.push((template_name, content));
    }

    Ok(library_files)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tempfile::tempdir;

    #[rstest]
    fn verify_args() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }

    #[rstest]
    fn parse_library_file() {
        let temp_test_dir =
            tempdir().expect("Failed to create temporary directory");
        let tmp_dir = temp_test_dir.path().to_str().unwrap();

        let file_path = Path::new(tmp_dir).join("test_file.txt");
        let file_content = "test content";
        std::fs::write(&file_path, file_content)
            .expect("Failed to write to test file");

        let file_path_str = file_path.to_str().unwrap();
        let result = parse_library(file_path_str).unwrap();

        assert_eq!(
            result,
            vec![("test_file.txt".to_string(), file_content.to_string())]
        );
    }

    #[rstest]
    fn parse_library_directory() {
        let temp_test_dir =
            tempdir().expect("Failed to create temporary directory");
        let basename =
            temp_test_dir.path().file_name().unwrap().to_str().unwrap();
        let tmp_dir = temp_test_dir.path().to_str().unwrap();
        let mut files = [
            ("file1.txt", "content1"),
            ("file2.txt", "content2"),
            ("file3.txt", "content3"),
        ];
        files.sort_by(|a, b| a.0.cmp(b.0));

        for (file_name, content) in &files {
            let file_path = Path::new(tmp_dir).join(file_name);
            std::fs::write(&file_path, content)
                .expect("Failed to write to test file");
        }

        let mut result = parse_library(tmp_dir).unwrap();
        result.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            result,
            files
                .iter()
                .map(|(name, content)| (
                    format!("{basename}/{name}"),
                    content.to_string()
                ))
                .collect::<Vec<_>>()
        );
    }

    #[rstest]
    fn parse_library_key_value_file_path() {
        let temp_test_dir =
            tempdir().expect("Failed to create temporary directory");
        let tmp_dir = temp_test_dir.path().to_str().unwrap();

        let file_path = Path::new(tmp_dir).join("test_file.txt");
        let file_content = "test content";
        std::fs::write(&file_path, file_content)
            .expect("Failed to write to test file");

        let file_path_str = file_path.to_str().unwrap();
        let result = parse_library(&format!("kvtest={file_path_str}")).unwrap();

        assert_eq!(
            result,
            vec![("kvtest".to_string(), file_content.to_string())]
        );
    }

    #[rstest]
    fn parse_library_key_value_directory_path() {
        let temp_test_dir =
            tempdir().expect("Failed to create temporary directory");
        let tmp_dir = temp_test_dir.path().to_str().unwrap();
        let mut files = [
            ("file1.txt", "content1"),
            ("file2.txt", "content2"),
            ("file3.txt", "content3"),
        ];
        files.sort_by(|a, b| a.0.cmp(b.0));

        for (file_name, content) in &files {
            let file_path = Path::new(tmp_dir).join(file_name);
            std::fs::write(&file_path, content)
                .expect("Failed to write to test file");
        }

        let mut result = parse_library(&format!("kvtest={tmp_dir}")).unwrap();
        result.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            result,
            files
                .iter()
                .map(|(name, content)| (
                    format!("kvtest/{name}"),
                    content.to_string()
                ))
                .collect::<Vec<_>>()
        );
    }
}
