use app::App;
use clap::Parser;
use std::process;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    let app = App::new(
        args.inputs,
        &args.skeleton_path.expect("-s is required"),
        &args.output_path,
    );

    if app.run().is_err() {
        process::exit(1);
    }
}
