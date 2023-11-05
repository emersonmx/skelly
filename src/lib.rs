pub mod config;
pub mod renderer;
pub mod validation;

use std::path::PathBuf;

pub const TEMPLATE_DIRECTORY_NAME: &str = "skeleton";

#[derive(Debug, Clone, PartialEq)]
pub struct UserInput {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Template {
    pub path: PathBuf,
    pub inputs: Vec<UserInput>,
}

#[derive(Debug)]
pub struct Context {
    pub template: Template,
    pub destination_path: PathBuf,
}

// fn filter_user_inputs(
//     user_inputs: &Vec<UserInput>,
//     config_inputs: &Vec<ConfigInput>,
// ) -> Vec<UserInput> {
//     let valid_inputs: HashSet<&String> =
//         config_inputs.iter().map(|i| &i.name).collect();
//     user_inputs
//         .iter()
//         .filter(|i| valid_inputs.contains(&i.name))
//         .map(|i| i.clone())
//         .collect()
// }
