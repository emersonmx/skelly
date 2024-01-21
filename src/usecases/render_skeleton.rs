use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::renderer;

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Failed to render skeleton")]
pub struct Error(pub String);

pub fn execute(
    template_directory: &Path,
    inputs: &[(String, String)],
    output_path: &Path,
) -> Result<(), Error> {
    WalkDir::new(template_directory)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_owned())
        .filter(|p| !p.is_dir())
        .try_for_each(|path| -> Result<(), Error> {
            let rendered_template =
                render_template(&path, inputs).map_err(|_| {
                    Error(format!(
                        "Unable to render file '{}'.",
                        path.display()
                    ))
                })?;
            let tmp_dir = template_directory.to_str().ok_or(Error(format!(
                "Unable to convert path '{}' to string",
                template_directory.display(),
            )))?;
            let relative_path =
                strip_path_prefix(&path, tmp_dir).map_err(|_| {
                    Error(format!(
                        "Unable to strip template path '{}' from path '{}'.",
                        template_directory.display(),
                        path.display()
                    ))
                })?;
            let rendered_relative_path = render_path(&relative_path, inputs)
                .map_err(|_| {
                    Error(format!(
                        "Unable to render path '{}'.",
                        path.display()
                    ))
                })?;

            write_temnplate(
                &PathBuf::from(rendered_relative_path),
                &rendered_template,
                output_path,
            )?;

            Ok(())
        })?;

    Ok(())
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
) -> Result<String, String> {
    let raw_path = path.to_str().ok_or("Unable to convert path to string.")?;
    let rendered_path = renderer::render(raw_path, inputs)
        .map_err(|_| "Unable to render path.")?;
    Ok(rendered_path)
}

fn write_temnplate(
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
