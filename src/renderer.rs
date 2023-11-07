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
) -> Result<String, Error> {
    let data: HashMap<String, String> =
        inputs.iter().map(|i| (i.0.to_owned(), i.1.to_owned())).collect();

    let mut context = Context::new();
    context.insert("skelly", &data);

    match Tera::one_off(template, &context, true) {
        Ok(r) => Ok(r),
        Err(_) => Err(Error::FailedToRender),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_same() {
        let result = render("test", &vec![]);

        assert_eq!("test", result.unwrap());
    }

    #[test]
    fn should_render_with_input() {
        let result = render(
            "Hello {{ skelly.name }}",
            &vec![("name".to_owned(), "John".to_owned())],
        );

        assert_eq!("Hello John", result.unwrap());
    }

    #[test]
    fn should_error_when_missing_input() {
        let result = render("Hello {{ skelly.name }}", &vec![]);

        assert_eq!(result, Err(Error::FailedToRender));
    }
}
