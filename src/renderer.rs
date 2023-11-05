use tera::{Context, Tera};

pub struct Input(pub String, pub String);

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Unable to render. Error: {0}")]
pub struct Error(pub String);

pub fn render(s: &str, inputs: &Vec<Input>) -> Result<String, Error> {
    let mut context = Context::new();
    for i in inputs {
        context.insert(&i.0, &i.1);
    }
    match Tera::one_off(&s, &context, true) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error(e.to_string())),
    }
}