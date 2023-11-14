use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(thiserror::Error, PartialEq, Debug)]
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
    pub fn new(inputs: &[Input]) -> Self {
        Self { inputs: inputs.to_owned() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_string() {
        let config = Config::from_str(
            r#"
            [[inputs]]
            name = "example"
            "#,
        )
        .unwrap();

        assert_eq!(
            config,
            Config::new(&[Input {
                name: "example".to_owned(),
                default: None,
                options: None,
            }]),
        );
    }
}
