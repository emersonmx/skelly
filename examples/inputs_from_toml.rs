use std::io::{self, Write};
use toml::Table;

fn input_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end_matches('\n').to_string()
}

fn main() {
    let skelly: Table = r#"
    [inputs]
    tool = ["rust", "cargo"]
    target_path = "target"
    "#
    .parse()
    .unwrap();

    let inputs = skelly["inputs"].as_table().unwrap();
    let mut values = Table::new();

    println!("Inputs");
    for (key, value) in inputs.iter() {
        if value.is_array() {
            let choices: Vec<String> = value
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c.as_str().unwrap().to_string())
                .collect();
            loop {
                let line =
                    input_line(&format!("{} [{}]: ", key, choices.join(", ")));
                if line.is_empty() {
                    let first = choices.first().unwrap().as_str();
                    values.insert(key.into(), first.into());
                    break;
                }
                if choices.contains(&line) {
                    values.insert(key.into(), line.into());
                    break;
                }
            }
        } else {
            loop {
                let value = value.as_str().unwrap();
                let line = input_line(&format!("{} [{}]: ", key, value));
                if line.is_empty() {
                    values.insert(key.into(), value.into());
                } else {
                    values.insert(key.into(), line.into());
                }
                break;
            }
        }
    }
    println!();

    println!("Results");
    for (name, value) in values.iter() {
        println!("{}: {}", name.as_str(), value.as_str().unwrap().trim_end());
    }
}
