#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Failed to render text")]
pub struct Error(pub String);

pub fn execute<R, W>(reader: R, writer: W) -> Result<(), Error>
where
    R: Fn() -> Result<String, Error>,
    W: Fn(String) -> Result<(), Error>,
{
    let content = reader()?;
    writer(content)?;

    Ok(())
}
