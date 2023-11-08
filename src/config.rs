use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to parse")]
    UnableToParse,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Input {
    pub name: String,
    pub default: Option<String>,
    pub options: Option<Vec<String>>,
}

pub type InputMap = HashMap<String, Input>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub inputs: Vec<Input>,
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).or(Err(Error::UnableToParse))
    }
}

impl Config {
    pub fn new(inputs: Vec<Input>) -> Self {
        Self { inputs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_from_string() {
        let config = Config::from_str(
            r#"
            [[inputs]]
            name = "example"
            "#,
        )
        .unwrap();

        assert_eq!(
            config,
            Config::new(vec![Input {
                name: "example".to_owned(),
                default: None,
                options: None,
            }]),
        );
    }
}
