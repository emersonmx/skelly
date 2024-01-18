use serde::{de, Deserialize, Deserializer, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const DEFAULT_SKELETON_PATH: &str = "skeleton";

fn value_to_string(value: &toml::Value) -> String {
    match value {
        toml::Value::String(v) => v.to_owned(),
        v => v.to_string(),
    }
}

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("Unable to read file")]
    UnableToReadFile,
    #[error("Unable to parse")]
    UnableToParse,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Input {
    pub name: String,

    #[serde(default, deserialize_with = "deserialize_default")]
    pub default: Option<String>,

    #[serde(default, deserialize_with = "deserialize_options")]
    pub options: Option<Vec<String>>,
}

fn deserialize_default<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: toml::Value = Deserialize::deserialize(deserializer)?;
    Ok(Some(value_to_string(&value)))
}

fn deserialize_options<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: toml::Value = Deserialize::deserialize(deserializer)?;
    let values: Vec<String> = value
        .as_array()
        .ok_or(de::Error::custom("unable to deserialize options"))?
        .iter()
        .map(value_to_string)
        .collect();

    Ok(Some(values))
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip, default = "default_skeleton_path")]
    pub skeleton_path: PathBuf,
    pub inputs: Vec<Input>,
}

fn default_skeleton_path() -> PathBuf {
    Path::new(DEFAULT_SKELETON_PATH).to_owned()
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let skeleton_path =
            path.parent().ok_or(Error::UnableToReadFile)?.to_owned();
        let content =
            fs::read_to_string(path).or(Err(Error::UnableToReadFile))?;
        let result: Self =
            toml::from_str(&content).or(Err(Error::UnableToParse))?;
        Ok(Self { skeleton_path, ..result })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    impl FromStr for Config {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            toml::from_str(s).or(Err(Error::UnableToParse))
        }
    }

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
            Config {
                skeleton_path: Path::new(DEFAULT_SKELETON_PATH).to_owned(),
                inputs: vec![Input {
                    name: "example".to_owned(),
                    default: None,
                    options: None,
                }],
            },
        );
    }

    #[test]
    fn convert_default_field_to_string() {
        let config = Config::from_str(
            r#"
            [[inputs]]
            name = "example"
            default = 42
            "#,
        )
        .unwrap();

        assert_eq!(
            config,
            Config {
                skeleton_path: Path::new(DEFAULT_SKELETON_PATH).to_owned(),
                inputs: vec![Input {
                    name: "example".to_owned(),
                    default: Some("42".to_owned()),
                    options: None,
                }],
            },
        );
    }

    #[test]
    fn convert_options_field_to_a_string_vector() {
        let config = Config::from_str(
            r#"
            [[inputs]]
            name = "example"
            options = [1, 2, 3]
            "#,
        )
        .unwrap();

        assert_eq!(
            config,
            Config {
                skeleton_path: Path::new(DEFAULT_SKELETON_PATH).to_owned(),
                inputs: vec![Input {
                    name: "example".to_owned(),
                    default: None,
                    options: Some(vec![
                        "1".to_owned(),
                        "2".to_owned(),
                        "3".to_owned()
                    ]),
                }],
            },
        );
    }
}
