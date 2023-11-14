use app::App;
use std::{path::PathBuf, process};

mod app;
mod cli;

fn main() {
    let args = cli::get_args();
    let app = App::new(
        args.inputs,
        &args.skeleton_path,
        args.output_path.unwrap_or(PathBuf::from(".")).as_path(),
    );

    if app.run().is_err() {
        process::exit(1);
    }
}
