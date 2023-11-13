use anyhow::{anyhow, Result};
use skelly::{
    config::Config,
    renderer::render,
    validation::{validate_inputs, ErrorType},
};
use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
    str::FromStr,
};
use walkdir::WalkDir;

pub struct App {
    user_inputs: Vec<(String, String)>,
    skeleton_path: PathBuf,
    output_path: PathBuf,
    template_path: PathBuf,
}

impl App {
    const CONFIG_FILENAME: &str = "skelly.toml";
    const SKELETON_DIRECTORY_NAME: &str = "skeleton";

    pub fn new(
        user_inputs: Vec<(String, String)>,
        skeleton_path: &Path,
        output_path: &Path,
    ) -> Self {
        Self {
            user_inputs,
            skeleton_path: skeleton_path.to_owned(),
            output_path: output_path.to_owned(),
            template_path: skeleton_path.join(Self::SKELETON_DIRECTORY_NAME),
        }
    }

    pub fn run(&self) -> Result<()> {
        let inputs = self.fetch_valid_inputs()?;
        for entry in self.iter_template_path() {
            let path = entry.as_ref().map(|e| e.path());
            if let Ok(path) = path {
                if path.is_dir() {
                    continue;
                }

                self.process_template(path, &inputs)?;
            }
        }

        Ok(())
    }

    fn read_config(&self) -> Result<Config> {
        let skelly_path = self.skeleton_path.join(Self::CONFIG_FILENAME);
        let skelly_content = fs::read_to_string(skelly_path)?;
        let config = Config::from_str(&skelly_content)?;
        Ok(config)
    }

    fn fetch_valid_inputs(&self) -> Result<Vec<(String, String)>> {
        let config = self.read_config()?;
        match validate_inputs(&self.user_inputs, &config.inputs) {
            Ok(inputs) => Ok(inputs),
            Err(errors) => {
                for error in &errors.0 {
                    match error {
                        ErrorType::MissingInput(name) => {
                            println!("Missing input '{}'.", name);
                        }
                        ErrorType::InvalidOption(name, value) => {
                            println!(
                                "Invalid option '{}' to input '{}'.",
                                value, name
                            );
                        }
                    }
                }
                Err(errors.into())
            }
        }
    }

    fn iter_template_path(&self) -> walkdir::IntoIter {
        WalkDir::new(&self.template_path).min_depth(1).into_iter()
    }

    fn process_template(
        &self,
        path: &Path,
        inputs: &[(String, String)],
    ) -> Result<()> {
        let rendered_template = self.render_template(path, inputs)?;
        let relative_path = self.strip_template_path(path)?;
        let rendered_relative_path =
            self.render_path(&relative_path, inputs)?;
        self.write_temnplate(&rendered_relative_path, &rendered_template)?;
        Ok(())
    }

    fn strip_template_path(&self, path: &Path) -> Result<PathBuf> {
        let relative_path = path.strip_prefix(&self.template_path)?;
        Ok(PathBuf::from(relative_path))
    }

    fn render_template(
        &self,
        path: &Path,
        inputs: &[(String, String)],
    ) -> Result<String> {
        let content = fs::read_to_string(path)?;
        let rendered_content = render(&content, inputs)?;
        Ok(rendered_content)
    }

    fn render_path(
        &self,
        path: &Path,
        inputs: &[(String, String)],
    ) -> Result<PathBuf> {
        let raw_path = path
            .to_str()
            .ok_or(anyhow!("Unable to convert relative path to str"))?;
        let rendered_path = render(raw_path, inputs)?;
        Ok(PathBuf::from(rendered_path))
    }

    fn write_temnplate(&self, path: &Path, content: &str) -> Result<()> {
        let output_path = self.output_path.join(path);
        let output_directory = output_path
            .parent()
            .ok_or(anyhow!("Unable to fetch parent directory",))?;
        create_dir_all(output_directory)?;
        fs::write(output_path, content)?;

        Ok(())
    }
}
