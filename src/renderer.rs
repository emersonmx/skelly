use std::collections::HashMap;
use tera::{Context, Tera};

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("Failed to render")]
    FailedToRender,
}

pub fn render(
    template: &str,
    inputs: &[(String, String)],
    prefix: &str,
) -> Result<String, Error> {
    let data: HashMap<String, String> =
        inputs.iter().map(|i| (i.0.to_owned(), i.1.to_owned())).collect();

    let mut context = Context::new();
    match prefix.trim() {
        "" => {
            for (key, value) in data {
                context.insert(&key, &value);
            }
        }
        p => context.insert(p, &data),
    }

    match Tera::one_off(template, &context, false) {
        Ok(r) => Ok(r),
        Err(_) => Err(Error::FailedToRender),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_same() {
        let result = render("test", &[], "");

        assert_eq!("test", result.unwrap());
    }

    #[test]
    fn render_with_input() {
        let result = render(
            "Hello {{ name }}",
            &[("name".to_owned(), "John".to_owned())],
            "",
        );

        assert_eq!("Hello John", result.unwrap());
    }

    #[test]
    fn render_with_input_and_prefix() {
        let result = render(
            "Hello {{ skelly.name }}",
            &[("name".to_owned(), "John".to_owned())],
            "skelly",
        );

        assert_eq!("Hello John", result.unwrap());
    }

    #[test]
    fn error_when_missing_input() {
        let result = render("Hello {{ skelly.name }}", &[], "skelly");

        assert_eq!(result, Err(Error::FailedToRender));
    }
}
