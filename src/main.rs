use app::App;
use clap::Parser;
use std::process;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    let skeleton_path = args.skeleton_path.expect("-s is required");
    let app = App::new(
        args.inputs,
        Some(skeleton_path.as_path()),
        args.output_path.as_path(),
    );

    if app.run().is_err() {
        process::exit(1);
    }
}
