mod cli;
fn main() {
    let args = cli::get_args();
    println!("{:?}", args);
}
