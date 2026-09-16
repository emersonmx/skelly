use std::collections::HashMap;
use std::error::Error as StdError;
use tera::{Context, Tera};

const INPUT_TEMPLATE_NAME: &str = "__input_template";

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Failed to render")]
pub struct Error(pub String);

pub fn render(
    library: &Vec<(String, String)>,
    template: &str,
    inputs: &[(String, String)],
) -> Result<String, Error> {
    let tera = new_tera(library, template)?;

    let data: HashMap<String, String> =
        inputs.iter().map(|i| (i.0.to_owned(), i.1.to_owned())).collect();

    let mut context = Context::new();
    for (key, value) in data {
        context.insert(key, &value);
    }

    match tera.render(INPUT_TEMPLATE_NAME, &context) {
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

fn new_tera(
    library: &Vec<(String, String)>,
    template: &str,
) -> Result<Tera, Error> {
    let mut tera = Tera::default();

    tera.autoescape_on(Vec::<&str>::new());

    register_base64(&mut tera);
    register_dates(&mut tera);
    register_filesize_format(&mut tera);
    register_format(&mut tera);
    register_json(&mut tera);
    register_rand(&mut tera);
    register_regex(&mut tera);
    register_slug(&mut tera);
    register_urlencode(&mut tera);

    for (library_name, library_content) in library {
        tera.add_raw_template(library_name, library_content).map_err(|e| {
            let message =
                e.source().map(|s| s.to_string()).unwrap_or(e.to_string());
            Error(message)
        })?;
    }

    tera.add_raw_template(INPUT_TEMPLATE_NAME, template).map_err(|e| {
        let message =
            e.source().map(|s| s.to_string()).unwrap_or(e.to_string());
        Error(message)
    })?;

    Ok(tera)
}

fn register_base64(tera: &mut Tera) {
    tera.register_filter("b64_encode", tera_contrib::base64::b64_encode);
    tera.register_filter("b64_decode", tera_contrib::base64::b64_decode);
}

fn register_dates(tera: &mut Tera) {
    tera.register_filter("date", tera_contrib::dates::date);
    tera.register_test("before", tera_contrib::dates::is_before);
    tera.register_test("after", tera_contrib::dates::is_after);
    tera.register_function("now", tera_contrib::dates::now);
}

fn register_filesize_format(tera: &mut Tera) {
    tera.register_filter(
        "filesize_format",
        tera_contrib::filesize_format::filesize_format,
    );
}

fn register_format(tera: &mut Tera) {
    tera.register_filter("format", tera_contrib::format::format);
}

fn register_json(tera: &mut Tera) {
    tera.register_filter("json_encode", tera_contrib::json::json_encode);
}

fn register_rand(tera: &mut Tera) {
    tera.register_function("get_random", tera_contrib::rand::get_random);
    tera.register_filter("shuffle", tera_contrib::rand::shuffle);
}

fn register_regex(tera: &mut Tera) {
    tera.register_filter("striptags", tera_contrib::regex::striptags);
    tera.register_filter("spaceless", tera_contrib::regex::spaceless);
    tera.register_filter(
        "regex_replace",
        tera_contrib::regex::RegexReplace::default(),
    );
    tera.register_test("matching", tera_contrib::regex::Matching::default());
}

fn register_slug(tera: &mut Tera) {
    tera.register_filter("slug", tera_contrib::slug::slug);
}

fn register_urlencode(tera: &mut Tera) {
    tera.register_filter("urlencode", tera_contrib::urlencode::urlencode);
    tera.register_filter(
        "urlencode_strict",
        tera_contrib::urlencode::urlencode_strict,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use rstest::rstest;

    #[rstest]
    fn return_same() {
        let result = render(&vec![], "test", &[]);

        assert_eq!("test", result.unwrap());
    }

    #[rstest]
    fn render_with_input() {
        let result = render(
            &vec![],
            "Hello {{ name }}",
            &[("name".to_owned(), "John".to_owned())],
        );

        assert_eq!("Hello John", result.unwrap());
    }

    #[rstest]
    fn error_when_missing_input() {
        let result = render(&vec![], "Hello {{ name }}", &[]);

        assert_yaml_snapshot!(format!("{:?}", result.unwrap_err().0));
    }
}
