use std::path::Path;

use skelly::app::App;

mod cli;

fn main() -> Result<(), String> {
    let args = cli::get_args();
    App::new(args.inputs, &args.skeleton_path, Path::new(".")).run()
}
