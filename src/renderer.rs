use std::collections::HashMap;
use std::error::Error as StdError;
use tera::{Context, Tera};

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Failed to render")]
pub struct Error(pub String);

pub fn render(
    template: &str,
    inputs: &[(String, String)],
) -> Result<String, Error> {
    let data: HashMap<String, String> =
        inputs.iter().map(|i| (i.0.to_owned(), i.1.to_owned())).collect();

    let mut context = Context::new();
    for (key, value) in data {
        context.insert(&key, &value);
    }

    match Tera::one_off(template, &context, false) {
        Ok(r) => Ok(r),
        Err(e) => {
            let error = e
                .source()
                .map(|s| s.to_string())
                .unwrap_or("unknown error".to_owned());
            Err(Error(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_same() {
        let result = render("test", &[]);

        assert_eq!("test", result.unwrap());
    }

    #[test]
    fn render_with_input() {
        let result = render(
            "Hello {{ name }}",
            &[("name".to_owned(), "John".to_owned())],
        );

        assert_eq!("Hello John", result.unwrap());
    }

    #[test]
    fn error_when_missing_input() {
        let expected = "Variable `name` not found in context while rendering '__tera_one_off'".to_owned();
        let result = render("Hello {{ name }}", &[]);

        assert_eq!(result, Err(Error(expected)));
    }
}
