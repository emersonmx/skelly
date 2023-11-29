use app::App;
use clap::Parser;
use std::process;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    let app = App::new(
        args.inputs,
        &args.skeleton_path,
        &args.output_path,
        &args.prefix,
    );

    if app.run().is_err() {
        process::exit(1);
    }
}
