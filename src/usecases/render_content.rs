use crate::renderer::render;

pub fn execute(
    text: &str,
    context: &[(String, String)],
) -> Result<String, String> {
    render(text, context).map_err(|e| e.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_a_valid_content() {
        let result = execute(
            "Hello {{ name }}!",
            &[("name".to_owned(), "John".to_owned())],
        );
        assert_eq!(result, Ok("Hello John!".to_owned()));
    }

    #[test]
    fn return_error_when_invalid_content() {
        let result = execute("{{ name }} <{{ email }}>!", &[]);
        assert_eq!(
            result.unwrap_err(),
            "Variable `name` not found in context while rendering '__tera_one_off'"
        )
    }
}
