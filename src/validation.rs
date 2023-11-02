use crate::config::InputMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct UserInput(pub String, pub String);

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("missing input '{0}'")]
pub struct MissingInputError(String);

pub fn validate_inputs(
    user_inputs: &Vec<UserInput>,
    input_map: &InputMap,
) -> Result<Vec<UserInput>, Vec<MissingInputError>> {
    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut errors: Vec<MissingInputError> = vec![];

    for im in input_map.values() {
        if let Some(value) = &im.default {
            inputs.insert(im.name.to_owned(), value.to_owned());
        }
    }

    for ui in user_inputs {
        if input_map.contains_key(&ui.0) {
            inputs.insert(ui.0.to_owned(), ui.1.to_owned());
        }
    }

    for im in input_map.values() {
        if !inputs.contains_key(&im.name) {
            errors.push(MissingInputError(im.name.to_owned()));
        }
    }

    if errors.is_empty() {
        Ok(inputs
            .iter()
            .map(|i| UserInput(i.0.to_owned(), i.1.to_owned()))
            .collect())
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

        assert_eq!(
            result,
            Ok(vec![UserInput("test".to_owned(), "ok".to_owned())])
        );
    }

    #[test]
    fn should_return_user_input_over_default() {
        let input_map = make_input_map();

        let result = validate_inputs(
            &vec![UserInput("test".to_owned(), "updated".to_owned())],
            &input_map,
        );

        assert_eq!(
            result,
            Ok(vec![UserInput("test".to_owned(), "updated".to_owned())])
        );
    }

    #[test]
    fn should_ignore_unknown_inputs() {
        let input_map = make_input_map();

        let result = validate_inputs(
            &vec![UserInput("unknown".to_owned(), "ignore".to_owned())],
            &input_map,
        );

        assert_eq!(
            result,
            Ok(vec![UserInput("test".to_owned(), "ok".to_owned())])
        );
    }

    #[test]
    fn should_return_error_when_missing_input() {
        let input_map = InputMap::from_iter([(
            "test".to_owned(),
            Input { name: "test".to_owned(), default: None, options: None },
        )]);

        let result = validate_inputs(&vec![], &input_map);
        println!("{:?}", result);

        assert_eq!(result, Err(vec![MissingInputError("test".to_owned())]),);
    }
}
