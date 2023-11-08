use crate::{config::Config, renderer::render, validation::validate_inputs};
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
    str::FromStr,
};
use walkdir::WalkDir;

pub struct App {
    user_inputs: Vec<(String, String)>,
    skeleton_path: PathBuf,
    output_path: PathBuf,
}

impl App {
    const CONFIG_FILENAME: &str = "skelly.toml";
    const SKELETON_DIRECTORY_NAME: &str = "skeleton";

    pub fn new(
        user_inputs: Vec<(String, String)>,
        skeleton_path: PathBuf,
        output_path: PathBuf,
    ) -> Self {
        Self { user_inputs, skeleton_path, output_path }
    }

    pub fn run(&self) {
        let config = self.read_config();

        // Validate inputs
        let inputs = validate_inputs(&self.user_inputs, &config.inputs);

        // Fetch a file, render its contents and copy to final path
        let template_path =
            self.skeleton_path.join(Self::SKELETON_DIRECTORY_NAME);
        let walker = WalkDir::new(&template_path).min_depth(1).into_iter();
        for entry in walker {
            let path = entry.as_ref().map(|e| e.path());
            if let Ok(p) = path {
                if p.is_dir() {
                    continue;
                }

                let content = fs::read_to_string(p).unwrap();
                let r = render(&content, inputs.as_ref().unwrap());
                let relative_path_raw = p.strip_prefix(&template_path);
                let relative_path = render(
                    relative_path_raw.unwrap().to_str().unwrap(),
                    inputs.as_ref().unwrap(),
                )
                .unwrap();

                let final_path = self.output_path.join(relative_path);
                let dir = final_path.parent().unwrap();
                create_dir_all(dir).unwrap();
                fs::write(final_path, &r.unwrap()).unwrap();
            }
        }

        // Show errors or exit
    }

    fn read_config(&self) -> Config {
        let skelly_path = self.skeleton_path.join(Self::CONFIG_FILENAME);
        let skelly_content = fs::read_to_string(skelly_path).unwrap();
        Config::from_str(&skelly_content).unwrap()
    }
}
