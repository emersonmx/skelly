use std::fs;
use std::path::{Path, PathBuf};

use crate::renderer;
use crate::usecases::render_skeleton::Error;

pub fn file_finder(path: &Path) -> impl IntoIterator<Item = PathBuf> {
    walkdir::WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_owned())
        .filter(|p| !p.is_dir())
}

pub fn file_reader(
    path: &Path,
    inputs: &[(String, String)],
    template_directory: &Path,
) -> Result<(PathBuf, String), Error> {
    let rendered_template = render_template(path, inputs).map_err(|_| {
        Error(format!("Unable to render file '{}'.", path.display()))
    })?;
    let relative_path =
        path.strip_prefix(template_directory).map_err(|_| {
            Error(format!(
                "Unable to strip template path '{}' from path '{}'.",
                template_directory.display(),
                path.display()
            ))
        })?;
    let rendered_relative_path =
        render_path(relative_path, inputs).map_err(|_| {
            Error(format!("Unable to render path '{}'.", path.display()))
        })?;

    Ok((rendered_relative_path, rendered_template))
}

fn render_template(
    path: &Path,
    inputs: &[(String, String)],
) -> Result<String, String> {
    let content =
        fs::read_to_string(path).map_err(|_| "Unable to read template")?;
    let rendered_content = renderer::render(&content, inputs)
        .map_err(|_| "Unable to render template")?;
    Ok(rendered_content)
}

fn render_path(
    path: &Path,
    inputs: &[(String, String)],
) -> Result<PathBuf, String> {
    let raw_path = path.to_str().ok_or("Unable to convert path to string.")?;
    let rendered_path = renderer::render(raw_path, inputs)
        .map_err(|_| "Unable to render path.")?;
    Ok(PathBuf::from(rendered_path))
}

pub fn file_writer(
    path: &Path,
    content: &str,
    output_path: &Path,
) -> Result<(), Error> {
    let output_path = output_path.join(path);
    let output_directory = output_path.parent().ok_or(Error(format!(
        "Unable to fetch parent directory of '{}'.",
        path.display()
    )))?;
    fs::create_dir_all(output_directory).map_err(|_| {
        Error(format!(
            "Unable to create path '{}'.",
            output_directory.display()
        ))
    })?;
    fs::write(&output_path, content).map_err(|_| {
        Error(format!(
            "Unable to write content to path '{}'.",
            &output_path.display()
        ))
    })?;
    Ok(())
}
