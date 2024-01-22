use std::fs;
use std::path::{Path, PathBuf};

use crate::usecases::render_skeleton::Error as RenderSkeletonError;
use crate::{config, renderer, validation};

pub fn clean_inputs(
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

pub fn file_finder(path: &Path) -> impl IntoIterator<Item = PathBuf> {
    walkdir::WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_owned())
        .filter(|p| !p.is_dir())
}

pub fn walk_dir_file_reader(
    path: &Path,
    inputs: &[(String, String)],
    template_directory: &Path,
) -> Result<(PathBuf, String), RenderSkeletonError> {
    let rendered_template = render_template(path, inputs).map_err(|_| {
        RenderSkeletonError(format!(
            "Unable to render file '{}'.",
            path.display()
        ))
    })?;
    let tmp_dir =
        template_directory.to_str().ok_or(RenderSkeletonError(format!(
            "Unable to convert path '{}' to string",
            template_directory.display(),
        )))?;
    let relative_path = strip_path_prefix(path, tmp_dir).map_err(|_| {
        RenderSkeletonError(format!(
            "Unable to strip template path '{}' from path '{}'.",
            template_directory.display(),
            path.display()
        ))
    })?;
    let rendered_relative_path =
        render_path(&relative_path, inputs).map_err(|_| {
            RenderSkeletonError(format!(
                "Unable to render path '{}'.",
                path.display()
            ))
        })?;

    Ok((PathBuf::from(rendered_relative_path), rendered_template))
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

fn strip_path_prefix(path: &Path, prefix: &str) -> Result<PathBuf, String> {
    let relative_path = path.strip_prefix(prefix).map_err(|_| {
        format!(
            "Unable to strip template path '{}' from path '{}'",
            prefix,
            path.display()
        )
    })?;
    Ok(PathBuf::from(relative_path))
}

fn render_path(
    path: &Path,
    inputs: &[(String, String)],
) -> Result<String, String> {
    let raw_path = path.to_str().ok_or("Unable to convert path to string.")?;
    let rendered_path = renderer::render(raw_path, inputs)
        .map_err(|_| "Unable to render path.")?;
    Ok(rendered_path)
}

pub fn file_writer(
    path: &Path,
    content: &str,
    output_path: &Path,
) -> Result<(), RenderSkeletonError> {
    let output_path = output_path.join(path);
    let output_directory = output_path.parent().ok_or(RenderSkeletonError(
        format!("Unable to fetch parent directory of '{}'.", path.display()),
    ))?;
    fs::create_dir_all(output_directory).map_err(|_| {
        RenderSkeletonError(format!(
            "Unable to create path '{}'.",
            output_directory.display()
        ))
    })?;
    fs::write(&output_path, content).map_err(|_| {
        RenderSkeletonError(format!(
            "Unable to write content to path '{}'.",
            &output_path.display()
        ))
    })?;
    Ok(())
}
