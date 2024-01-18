pub mod cli;
pub mod config;
pub mod renderer;
pub mod usecases;
pub mod validation;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    match &args {
        Args { skeleton_config: Some(skeleton_config), .. } => {
            let cleaned_inputs = validation::validate_inputs(
                &args.inputs,
                &skeleton_config.inputs,
            );
            eprintln!("skeleton_config = {:?}", skeleton_config);
            eprintln!("output_path = {:?}", args.output_path);
            eprintln!("inputs = {:?}", args.inputs);
            eprintln!("cleaned_inputs = {:?}", cleaned_inputs);
        }
        Args { skeleton_config: None, .. } => {
            println!("NO SKELETON CONFIG!");
            eprintln!("output_path = {:?}", args.output_path);
            eprintln!("inputs = {:?}", args.inputs);
        }
    }
}
