use crate::config::InputMap;
use std::collections::HashMap;

pub type UserInput = (String, String);

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("missing input '{0}'")]
    MissingInput(String),
    #[error("invalid option {1} to input '{0}'")]
    InvalidOption(String, String),
}

pub fn validate_inputs(
    user_inputs: &Vec<UserInput>,
    input_map: &InputMap,
) -> Result<Vec<UserInput>, Vec<Error>> {
    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut errors: Vec<Error> = vec![];

    for im in input_map.values() {
        if let Some(value) = &im.default {
            inputs.insert(im.name.to_owned(), value.to_owned());
        }
    }

    for ui in user_inputs {
        if !input_map.contains_key(&ui.0) {
            continue;
        }
        let options = input_map
            .get(&ui.0)
            .and_then(|i| i.options.to_owned())
            .unwrap_or(vec![]);
        if options.is_empty() || options.contains(&ui.1) {
            inputs.insert(ui.0.to_owned(), ui.1.to_owned());
        } else {
            errors.push(Error::InvalidOption(ui.0.to_owned(), ui.1.to_owned()));
        }
    }

    for im in input_map.values() {
        if !inputs.contains_key(&im.name) {
            errors.push(Error::MissingInput(im.name.to_owned()));
        }
    }

    if errors.is_empty() {
        Ok(inputs.into_iter().collect())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Input;

    use super::*;

    fn make_input_map() -> InputMap {
        InputMap::from_iter([(
            "test".to_owned(),
            Input {
                name: "test".to_owned(),
                default: Some("ok".to_owned()),
                options: None,
            },
        )])
    }

    #[test]
    fn should_return_default_when_empty_user_inputs() {
        let input_map = make_input_map();

        let result = validate_inputs(&vec![], &input_map);

        assert_eq!(result, Ok(vec![("test".to_owned(), "ok".to_owned())]));
    }

    #[test]
    fn should_return_user_input_over_default() {
        let input_map = make_input_map();

        let result = validate_inputs(
            &vec![("test".to_owned(), "updated".to_owned())],
            &input_map,
        );

        assert_eq!(result, Ok(vec![("test".to_owned(), "updated".to_owned())]));
    }

    #[test]
    fn should_ignore_unknown_inputs() {
        let input_map = make_input_map();

        let result = validate_inputs(
            &vec![("unknown".to_owned(), "ignore".to_owned())],
            &input_map,
        );

        assert_eq!(result, Ok(vec![("test".to_owned(), "ok".to_owned())]));
    }

    #[test]
    fn should_ignore_empty_options_list() {
        let input_map = InputMap::from_iter([(
            "test".to_owned(),
            Input {
                name: "test".to_owned(),
                default: None,
                options: Some(vec![]),
            },
        )]);

        let result = validate_inputs(
            &vec![("test".to_owned(), "invalid".to_owned())],
            &input_map,
        );
        println!("{:?}", result);

        assert_eq!(result, Ok(vec![("test".to_owned(), "invalid".to_owned())]),);
    }

    #[test]
    fn should_return_error_when_missing_input() {
        let input_map = InputMap::from_iter([(
            "test".to_owned(),
            Input { name: "test".to_owned(), default: None, options: None },
        )]);

        let result = validate_inputs(&vec![], &input_map);
        println!("{:?}", result);

        assert_eq!(result, Err(vec![Error::MissingInput("test".to_owned())]),);
    }

    #[test]
    fn should_return_error_when_input_is_not_a_valid_option() {
        let input_map = InputMap::from_iter([(
            "test".to_owned(),
            Input {
                name: "test".to_owned(),
                default: None,
                options: Some(vec!["ok".to_owned(), "fail".to_owned()]),
            },
        )]);

        let result = validate_inputs(
            &vec![("test".to_owned(), "invalid".to_owned())],
            &input_map,
        );
        println!("{:?}", result);

        assert_eq!(
            result,
            Err(vec![
                Error::InvalidOption("test".to_owned(), "invalid".to_owned()),
                Error::MissingInput("test".to_owned())
            ]),
        );
    }
}
