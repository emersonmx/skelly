use serde::{Deserialize, Deserializer, Serialize, de};
use std::{
    fs,
    path::{Path, PathBuf},
};

const DEFAULT_TEMPLATE_DIRECTORY: &str = "skeleton";

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("Unable to read file")]
    UnableToReadFile,
    #[error("Unable to parse")]
    UnableToParse,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip, default = "default_template_directory")]
    pub template_directory: PathBuf,

    pub inputs: Vec<Input>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let skeleton_directory =
            path.parent().unwrap_or(Path::new(".")).to_owned();
        let template_directory =
            skeleton_directory.join(default_template_directory()).to_owned();
        let content =
            fs::read_to_string(path).or(Err(Error::UnableToReadFile))?;
        let result: Self =
            toml::from_str(&content).or(Err(Error::UnableToParse))?;
        Ok(Self { template_directory, ..result })
    }
}

fn default_template_directory() -> PathBuf {
    Path::new(DEFAULT_TEMPLATE_DIRECTORY).to_owned()
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Input {
    pub name: String,

    #[serde(default, deserialize_with = "deserialize_default")]
    pub default: Option<String>,

    #[serde(default, deserialize_with = "deserialize_options")]
    pub options: Option<Vec<String>>,
}

fn value_to_string(value: &toml::Value) -> String {
    match value {
        toml::Value::String(v) => v.to_owned(),
        v => v.to_string(),
    }
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

    fn default_config() -> Config {
        Config {
            template_directory: default_template_directory(),
            inputs: Vec::new(),
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
                inputs: vec![Input {
                    name: "example".to_owned(),
                    default: None,
                    options: None,
                }],
                ..default_config()
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
                inputs: vec![Input {
                    name: "example".to_owned(),
                    default: Some("42".to_owned()),
                    options: None,
                }],
                ..default_config()
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
                inputs: vec![Input {
                    name: "example".to_owned(),
                    default: None,
                    options: Some(vec![
                        "1".to_owned(),
                        "2".to_owned(),
                        "3".to_owned()
                    ]),
                }],
                ..default_config()
            },
        );
    }
}
