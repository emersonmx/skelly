use skelly::{
    config::{self, Config},
    renderer::render,
    validation::{self, validate_inputs},
};
use std::{
    fs::{self, create_dir_all},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};
use walkdir::WalkDir;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    FailedToReadFile(#[from] io::Error),
    #[error(transparent)]
    FailedToParseConfig(#[from] config::Error),
    #[error("{0:?}")]
    InvalidInputs(Vec<validation::Error>),
    #[error("{0}")]
    FailedToRender(String),
}

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

    pub fn run(&self) -> Result<(), Error> {
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

    fn read_config(&self) -> Result<Config, Error> {
        let skelly_path = self.skeleton_path.join(Self::CONFIG_FILENAME);
        let skelly_content = fs::read_to_string(skelly_path)?;
        Config::from_str(&skelly_content).map_err(Error::FailedToParseConfig)
    }

    fn fetch_valid_inputs(&self) -> Result<Vec<(String, String)>, Error> {
        let config = self.read_config()?;
        validate_inputs(&self.user_inputs, &config.inputs)
            .map_err(Error::InvalidInputs)
    }

    fn iter_template_path(&self) -> walkdir::IntoIter {
        WalkDir::new(&self.template_path).min_depth(1).into_iter()
    }

    fn process_template(
        &self,
        path: &Path,
        inputs: &[(String, String)],
    ) -> Result<(), Error> {
        let rendered_template = self.render_template(path, inputs)?;
        let relative_path = self.strip_template_path(path)?;
        let rendered_relative_path =
            self.render_path(&relative_path, inputs)?;
        self.write_temnplate(&rendered_relative_path, &rendered_template)?;
        Ok(())
    }

    fn strip_template_path(&self, path: &Path) -> Result<PathBuf, Error> {
        let relative_path =
            path.strip_prefix(&self.template_path).map_err(|_| {
                Error::FailedToRender(
                    "Unable to strip template path".to_owned(),
                )
            })?;
        Ok(PathBuf::from(relative_path))
    }

    fn render_template(
        &self,
        path: &Path,
        inputs: &[(String, String)],
    ) -> Result<String, Error> {
        let content = fs::read_to_string(path)?;
        render(&content, inputs).map_err(|_| {
            Error::FailedToRender("Unable to render template".to_owned())
        })
    }

    fn render_path(
        &self,
        path: &Path,
        inputs: &[(String, String)],
    ) -> Result<PathBuf, Error> {
        let raw_path = path.to_str().ok_or(Error::FailedToRender(
            "Unable to convert relative path to str".to_owned(),
        ))?;
        let rendered_path = render(raw_path, inputs).map_err(|_| {
            Error::FailedToRender("Unable to render relative path".to_owned())
        })?;
        Ok(PathBuf::from(rendered_path))
    }

    fn write_temnplate(&self, path: &Path, content: &str) -> Result<(), Error> {
        let output_path = self.output_path.join(path);
        let output_directory =
            output_path.parent().ok_or(Error::FailedToRender(
                "Unable to fetch parent directory".to_owned(),
            ))?;
        create_dir_all(output_directory)?;
        fs::write(output_path, content)?;

        Ok(())
    }
}
