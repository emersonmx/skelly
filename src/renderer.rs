use std::collections::HashMap;
use std::error::Error as StdError;
use tera::{Context, Tera};

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Failed to render")]
pub struct Error(pub String);

pub fn render(
    library_dir: Option<&str>,
    template: &str,
    inputs: &[(String, String)],
) -> Result<String, Error> {
    let mut tera = match library_dir {
        Some(dir) => Tera::new(&format!("{dir}/**/*")).map_err(|e| {
            let message =
                e.source().map(|s| s.to_string()).unwrap_or(e.to_string());
            Error(message)
        })?,
        None => Tera::default(),
    };
    tera.autoescape_on(vec![]);

    let data: HashMap<String, String> =
        inputs.iter().map(|i| (i.0.to_owned(), i.1.to_owned())).collect();

    let mut context = Context::new();
    for (key, value) in data {
        context.insert(&key, &value);
    }

    match tera.render_str(template, &context) {
        Ok(r) => Ok(r),
        Err(e) => {
            let message =
                e.source().map(|s| s.to_string()).unwrap_or(e.to_string());
            let error_message = if message.starts_with("Variable `")
                && message.contains("` not found in context")
            {
                let variable_name = message
                    .split("Variable `")
                    .nth(1)
                    .and_then(|s| s.split('`').next())
                    .unwrap_or("unknown");
                format!("Variable `{variable_name}` not found",)
            } else {
                message
            };

            Err(Error(error_message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_same() {
        let result = render(None, "test", &[]);

        assert_eq!("test", result.unwrap());
    }

    #[test]
    fn render_with_input() {
        let result = render(
            None,
            "Hello {{ name }}",
            &[("name".to_owned(), "John".to_owned())],
        );

        assert_eq!("Hello John", result.unwrap());
    }

    #[test]
    fn error_when_missing_input() {
        let expected = "Variable `name` not found".to_string();
        let result = render(None, "Hello {{ name }}", &[]);

        assert_eq!(result, Err(Error(expected)));
    }
}
