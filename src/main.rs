pub mod cli;
pub mod config;
pub mod renderer;
pub mod usecases;
pub mod validation;

use std::io::IsTerminal;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    let use_input_terminal = std::io::stdin().is_terminal();
    let use_output_terminal = std::io::stdout().is_terminal();

    match (&args, use_input_terminal, use_output_terminal) {
        (Args { skeleton_config: Some(skeleton_config), .. }, true, true) => {
            usecases::render_skeleton::execute(
                &args.inputs,
                &args.output_path,
                skeleton_config,
            )
            .unwrap();
        }
        (Args { skeleton_config: None, .. }, _, _) => {
            println!("NO SKELETON CONFIG!");
            eprintln!("output_path = {:?}", args.output_path);
            eprintln!("inputs = {:?}", args.inputs);
        }
        _ => {
            println!("WAT?!");
        }
    }
}
