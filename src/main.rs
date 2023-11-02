use skelly::{
    config::{to_input_map, Config},
    validation::{validate_inputs, UserInput},
};

pub const CONFIG_FILENAME: &str = "skelly.toml";
pub const SKELETON_DIRECTORY_NAME: &str = "skeleton";

mod cli;
fn main() {
    let args = cli::get_args();

    // Read skelly.toml
    let config =
        Config::from_path(&args.skeleton_path.join(CONFIG_FILENAME)).unwrap();

    // Validate inputs
    let user_inputs: Vec<UserInput> = args
        .inputs
        .iter()
        .map(|i| UserInput(i.0.to_owned(), i.1.to_owned()))
        .collect();
    let input_map = to_input_map(config.inputs);
    let inputs = validate_inputs(&user_inputs, &input_map);
    println!("{:?}", inputs);

    // Fetch list of files to render
    // Render files
    // Show errors or exit
}
