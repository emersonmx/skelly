use crate::{
    config::{self, Config},
    renderer::render,
    validation::{self, validate_inputs},
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

    pub fn run(&self) -> Result<(), String> {
        let config = self.read_config().unwrap();

        // Validate inputs
        let inputs = self.fetch_valid_inputs(&config).unwrap();

        // Fetch a file, render its contents and copy to final path
        let walker = WalkDir::new(&self.template_path).min_depth(1).into_iter();
        for entry in walker {
            let path = entry.as_ref().map(|e| e.path());
            if let Ok(p) = path {
                if p.is_dir() {
                    continue;
                }

                let content = fs::read_to_string(p).unwrap();
                let r = render(&content, &inputs);
                let relative_path_raw = p.strip_prefix(&self.template_path);
                let relative_path = render(
                    relative_path_raw.unwrap().to_str().unwrap(),
                    &inputs,
                )
                .unwrap();

                let final_path = self.output_path.join(relative_path);
                let dir = final_path.parent().unwrap();
                create_dir_all(dir).unwrap();
                fs::write(final_path, &r.unwrap()).unwrap();
            }
        }

        // Show errors or exit
        Ok(())
    }

    fn read_config(&self) -> Result<Config, config::Error> {
        let skelly_path = self.skeleton_path.join(Self::CONFIG_FILENAME);
        let skelly_content = fs::read_to_string(skelly_path).unwrap();
        Config::from_str(&skelly_content)
    }

    fn fetch_valid_inputs(
        &self,
        config: &Config,
    ) -> Result<Vec<(String, String)>, Vec<validation::Error>> {
        validate_inputs(&self.user_inputs, &config.inputs)
    }
}
