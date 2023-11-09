use app::App;
use std::{path::Path, process};

mod app;
mod cli;

fn main() {
    let args = cli::get_args();
    let app = App::new(args.inputs, &args.skeleton_path, Path::new("."));

    if app.run().is_err() {
        process::exit(1);
    }
}
