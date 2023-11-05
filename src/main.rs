use std::fs;

use skelly::{
    config::{to_input_map, Config},
    renderer::render,
    validation::validate_inputs,
};
use walkdir::WalkDir;

pub const CONFIG_FILENAME: &str = "skelly.toml";
pub const SKELETON_DIRECTORY_NAME: &str = "skeleton";

mod cli;
fn main() {
    let args = cli::get_args();

    // Read skelly.toml
    let config =
        Config::from_path(&args.skeleton_path.join(CONFIG_FILENAME)).unwrap();

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
            print!("{}", r.unwrap());
        }
    }

    // Show errors or exit
}
