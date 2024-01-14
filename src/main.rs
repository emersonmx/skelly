pub mod cli;
pub mod config;
pub mod renderer;
pub mod usecases;

use clap::Parser;
use cli::Args;
use usecases::render_content;

fn main() {
    let args = Args::parse();
    eprintln!("args = {:?}", args);

    let output = render_content::execute(
        "{{ name }} <{{ email }}>",
        &[
            ("name".to_owned(), "John".to_owned()),
            ("email".to_owned(), "johndoe@example.com".to_owned()),
        ],
    );

    eprintln!("output = {:?}", output);
}
