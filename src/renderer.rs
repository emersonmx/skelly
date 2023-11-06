use tera::{Context, Tera};

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("Unable to render. Error: {0}")]
    Unknown(String),
}

pub fn render(
    s: &str,
    inputs: &Vec<(String, String)>,
) -> Result<String, Error> {
    let mut context = Context::new();
    for i in inputs {
        context.insert(&i.0, &i.1);
    }
    match Tera::one_off(s, &context, true) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}
