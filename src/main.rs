use app::App;
use clap::Parser;
use std::process;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    let app = App::new(
        args.inputs,
        args.skeleton_path.as_deref(),
        args.output_path.as_path(),
    );

    if app.run().is_err() {
        process::exit(1);
    }
}
