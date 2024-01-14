pub mod cli;
pub mod config;
pub mod renderer;
pub mod usecases;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    match &args {
        Args { skeleton_config: Some(skeleton_config), .. } => {
            eprintln!("skeleton_config = {:?}", skeleton_config);
            eprintln!("output_path = {:?}", args.output_path);
            eprintln!("inputs = {:?}", args.inputs);
        }
        Args { skeleton_config: None, .. } => {
            println!("NO SKELETON CONFIG!");
            eprintln!("output_path = {:?}", args.output_path);
            eprintln!("inputs = {:?}", args.inputs);
        }
    }
}
