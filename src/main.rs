use std::path::PathBuf;

use skelly::app::App;

mod cli;

fn main() {
    let args = cli::get_args();
    App::new(args.inputs, args.skeleton_path, PathBuf::from(".")).run();
}
