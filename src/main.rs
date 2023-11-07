use skelly::{
    config::{to_input_map, Config},
    renderer::render,
    validation::validate_inputs,
};
use std::{
    fs::{self, create_dir_all},
    path::Path,
    str::FromStr,
};
use walkdir::WalkDir;

mod cli;

pub const CONFIG_FILENAME: &str = "skelly.toml";
pub const SKELETON_DIRECTORY_NAME: &str = "skeleton";

fn main() {
    let args = cli::get_args();
    let output_path = Path::new(".");

    // Read skelly.toml
    let skelly_path = args.skeleton_path.join(CONFIG_FILENAME);
    let skelly_content = fs::read_to_string(skelly_path).unwrap();
    let config = Config::from_str(&skelly_content).unwrap();

    // Validate inputs
    let input_map = to_input_map(config.inputs);
    let inputs = validate_inputs(&args.inputs, &input_map);

    // Fetch a file, render its contents and copy to final path
    let template_path = args.skeleton_path.join(SKELETON_DIRECTORY_NAME);
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

            let final_path = output_path.join(relative_path);
            let dir = final_path.parent().unwrap();
            create_dir_all(dir).unwrap();
            fs::write(final_path, &r.unwrap()).unwrap();
        }
    }

    // Show errors or exit
}
