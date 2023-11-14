use crate::config::Input;
use std::collections::HashMap;

pub type UserInput = (String, String);

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("validation errors")]
pub struct Error(pub Vec<ErrorType>);

#[derive(PartialEq, Debug)]
pub enum ErrorType {
    MissingInput(String),
    InvalidOption(String, String),
}

pub fn validate_inputs(
    user_inputs: &[UserInput],
    config_inputs: &[Input],
) -> Result<Vec<UserInput>, Error> {
    let input_map = create_input_map(config_inputs);
    let mut inputs = create_inputs_with_defaults(config_inputs);
    let mut errors = Vec::new();

    fill_with_valid_inputs(user_inputs, &input_map, &mut inputs, &mut errors);
    check_for_missing_inputs(&inputs, &input_map, &mut errors);

    if errors.is_empty() {
        Ok(inputs.into_iter().collect())
    } else {
        Err(Error(errors))
    }
}

fn create_input_map(inputs: &[Input]) -> HashMap<String, Input> {
    inputs.iter().map(|i| (i.name.to_owned(), i.to_owned())).collect()
}

fn create_inputs_with_defaults(inputs: &[Input]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for input in inputs {
        if let Some(value) = &input.default {
            result.insert(input.name.to_owned(), value.to_owned());
        }
    }
    result
}

fn fill_with_valid_inputs(
    user_inputs: &[UserInput],
    input_map: &HashMap<String, Input>,
    inputs: &mut HashMap<String, String>,
    errors: &mut Vec<ErrorType>,
) {
    for ui in user_inputs {
        if !input_map.contains_key(&ui.0) {
            continue;
        }
        let options = fetch_options(input_map, &ui.0);
        if has_option(&ui.1, &options) {
            inputs.insert(ui.0.to_owned(), ui.1.to_owned());
        } else {
            errors.push(ErrorType::InvalidOption(
                ui.0.to_owned(),
                ui.1.to_owned(),
            ));
        }
    }
}

fn fetch_options(input_map: &HashMap<String, Input>, key: &str) -> Vec<String> {
    input_map.get(key).and_then(|i| i.options.to_owned()).unwrap_or_default()
}

fn has_option(option: &String, options: &Vec<String>) -> bool {
    options.is_empty() || options.contains(option)
}

fn check_for_missing_inputs(
    inputs: &HashMap<String, String>,
    input_map: &HashMap<String, Input>,
    errors: &mut Vec<ErrorType>,
) {
    for im in input_map.values() {
        if !inputs.contains_key(&im.name) {
            errors.push(ErrorType::MissingInput(im.name.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Input;

    use super::*;

    fn make_config_inputs() -> Vec<Input> {
        vec![Input {
            name: "test".to_owned(),
            default: Some("ok".to_owned()),
            options: None,
        }]
    }

    #[test]
    fn return_default_when_empty_user_inputs() {
        let input_map = make_config_inputs();

        let result = validate_inputs(&[], &input_map);

        assert_eq!(result, Ok(vec![("test".to_owned(), "ok".to_owned())]));
    }

    #[test]
    fn return_user_input_over_default() {
        let input_map = make_config_inputs();

        let result = validate_inputs(
            &[("test".to_owned(), "updated".to_owned())],
            &input_map,
        );

        assert_eq!(result, Ok(vec![("test".to_owned(), "updated".to_owned())]));
    }

    #[test]
    fn ignore_unknown_inputs() {
        let input_map = make_config_inputs();

        let result = validate_inputs(
            &[("unknown".to_owned(), "ignore".to_owned())],
            &input_map,
        );

        assert_eq!(result, Ok(vec![("test".to_owned(), "ok".to_owned())]));
    }

    #[test]
    fn ignore_empty_options_list() {
        let config_inputs = vec![Input {
            name: "test".to_owned(),
            default: None,
            options: Some(Vec::new()),
        }];

        let result = validate_inputs(
            &[("test".to_owned(), "invalid".to_owned())],
            &config_inputs,
        );

        assert_eq!(result, Ok(vec![("test".to_owned(), "invalid".to_owned())]),);
    }

    #[test]
    fn return_error_when_missing_input() {
        let config_inputs = vec![Input {
            name: "test".to_owned(),
            default: None,
            options: None,
        }];

        let got = validate_inputs(&[], &config_inputs);
        let want = Err(Error(vec![ErrorType::MissingInput("test".to_owned())]));

        assert_eq!(got, want);
    }

    #[test]
    fn return_error_when_input_is_not_a_valid_option() {
        let config_inputs = vec![Input {
            name: "test".to_owned(),
            default: None,
            options: Some(vec!["ok".to_owned(), "fail".to_owned()]),
        }];

        let result = validate_inputs(
            &[("test".to_owned(), "invalid".to_owned())],
            &config_inputs,
        );

        assert_eq!(
            result,
            Err(Error(vec![
                ErrorType::InvalidOption(
                    "test".to_owned(),
                    "invalid".to_owned()
                ),
                ErrorType::MissingInput("test".to_owned())
            ])),
        );
    }
}
