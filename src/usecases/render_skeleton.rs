use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::renderer;

pub fn execute(
    template_directory: &Path,
    inputs: &[(String, String)],
    output_path: &Path,
) -> Result<(), String> {
    WalkDir::new(template_directory)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_owned())
        .filter(|p| !p.is_dir())
        .for_each(|path| {
            let rendered_template = render_template(&path, inputs).unwrap();
            let relative_path =
                strip_path_prefix(&path, template_directory.to_str().unwrap())
                    .unwrap();
            let rendered_relative_path =
                render_path(&relative_path, inputs).unwrap();

            write_temnplate(
                &PathBuf::from(rendered_relative_path),
                &rendered_template,
                output_path,
            );
        });

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

fn write_temnplate(path: &Path, content: &str, output_path: &Path) {
    let output_path = output_path.join(path);
    let output_directory = output_path.parent().unwrap();
    fs::create_dir_all(output_directory).unwrap();
    fs::write(&output_path, content).unwrap();
}
