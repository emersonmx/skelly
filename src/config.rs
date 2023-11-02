use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to read")]
    UnableToRead,
    #[error("Unable to parse")]
    UnableToParse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    pub name: String,
    pub default: Option<String>,
    pub options: Option<Vec<String>>,
}

pub type InputMap = HashMap<String, Input>;

pub fn to_input_map(inputs: Vec<Input>) -> InputMap {
    inputs.iter().map(|i| (i.name.to_owned(), i.to_owned())).collect()
}

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path).or(Err(Error::UnableToRead))?;
        Self::from_str(&content)
    }
}
