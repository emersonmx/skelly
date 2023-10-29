use tera::{Context, Tera};

fn main() {
    let template = r#"{{ name }} <{{ email }}>"#;

    let mut context = Context::new();
    context.insert("name", "John");
    context.insert("email", "john@example.com");

    let result = Tera::one_off(template, &context, true).unwrap();
    println!("{}", result);
}
