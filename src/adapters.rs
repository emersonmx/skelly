use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::renderer;

fn make_message(message: &str, error: &str, verbose: bool) -> String {
    if verbose {
        format!("{}\n    {}", message, error)
    } else {
        message.to_owned()
    }
}

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
    verbose: bool,
) -> Result<(PathBuf, String), String> {
    let rendered_template =
        render_template(path, inputs, verbose).map_err(|e| {
            make_message(
                &format!("Unable to render file '{}'.", path.display()),
                &e,
                verbose,
            )
        })?;
    let relative_path = path.strip_prefix(template_directory).map_err(|e| {
        make_message(
            &format!(
                "Unable to strip template path '{}' from path '{}'.",
                template_directory.display(),
                path.display()
            ),
            &e.to_string(),
            verbose,
        )
    })?;
    let rendered_relative_path = render_path(relative_path, inputs, verbose)
        .map_err(|e| {
            make_message(
                &format!("Unable to render path '{}'.", path.display()),
                &e,
                verbose,
            )
        })?;

    Ok((rendered_relative_path, rendered_template))
}

fn render_template(
    path: &Path,
    inputs: &[(String, String)],
    verbose: bool,
) -> Result<String, String> {
    let content = fs::read_to_string(path).map_err(|e| {
        make_message("Unable to read template.", &e.to_string(), verbose)
    })?;
    let rendered_content = renderer::render(&content, inputs).map_err(|e| {
        make_message("Unable to render template.", &e.0, verbose)
    })?;
    Ok(rendered_content)
}

fn render_path(
    path: &Path,
    inputs: &[(String, String)],
    verbose: bool,
) -> Result<PathBuf, String> {
    let raw_path = path.to_str().ok_or("Unable to convert path to string.")?;
    let rendered_path = renderer::render(raw_path, inputs)
        .map_err(|e| make_message("Unable to render path.", &e.0, verbose))?;
    Ok(PathBuf::from(rendered_path))
}

pub fn text_reader(
    inputs: &[(String, String)],
    verbose: bool,
) -> Result<String, String> {
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).map_err(|e| {
        make_message("Unable to read from stdin.", &e.to_string(), verbose)
    })?;
    let rendered_content = renderer::render(&content, inputs).map_err(|e| {
        make_message("Unable to render template.", &e.0, verbose)
    })?;
    Ok(rendered_content)
}

pub fn file_writer(
    path: &Path,
    content: &str,
    output_path: &Path,
    verbose: bool,
) -> Result<(), String> {
    let output_path = output_path.join(path);
    let output_directory = output_path.parent().ok_or(format!(
        "Unable to fetch parent directory of '{}'.",
        path.display()
    ))?;
    fs::create_dir_all(output_directory).map_err(|e| {
        make_message(
            &format!("Unable to create path '{}'.", output_directory.display()),
            &e.to_string(),
            verbose,
        )
    })?;
    fs::write(&output_path, content).map_err(|e| {
        make_message(
            &format!(
                "Unable to write content to path '{}'.",
                &output_path.display()
            ),
            &e.to_string(),
            verbose,
        )
    })?;
    Ok(())
}

pub fn text_writer(content: String) {
    print!("{content}");
}
