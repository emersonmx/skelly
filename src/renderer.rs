use std::collections::HashMap;
use tera::{Context, Tera};

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("Unable to render. Error: {0}")]
    Unknown(String),
}

pub fn render(
    template: &str,
    inputs: &Vec<(String, String)>,
) -> Result<String, Error> {
    let data: HashMap<String, String> =
        inputs.iter().map(|i| (i.0.to_owned(), i.1.to_owned())).collect();

    let mut context = Context::new();
    context.insert("skelly", &data);

    match Tera::one_off(template, &context, true) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}
