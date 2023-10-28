use toml::Table;

fn main() {
    let skelly = r#"
    [inputs]
    tool = ["rust", "cargo"]
    target_path = "target"
    "#;
    let value = skelly.parse::<Table>().unwrap();
    println!("{:?}", value);
}
