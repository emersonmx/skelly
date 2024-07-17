use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};

#[derive(thiserror::Error, PartialEq, Debug)]
#[error("Failed to render skeleton")]
pub struct Error(pub String);

pub fn execute<F, R, W>(
    file_finder: F,
    reader: R,
    writer: W,
) -> Result<(), Error>
where
    F: IntoIterator<Item = PathBuf>,
    R: Fn(&Path) -> Result<(PathBuf, String), Error>,
    W: Fn(&Path, String) -> Result<(), Error>,
{
    file_finder.into_iter().try_for_each(|path| -> Result<(), Error> {
        let (relative_path, content) = reader(&path)?;
        writer(&relative_path, content)?;
        let mode = match path.metadata() {
            Ok(meta) => meta.mode(),
            Err(_) => 0o644,
        };
        fs::set_permissions(relative_path, fs::Permissions::from_mode(mode))
            .map_err(|e| Error(e.to_string()))?;
        Ok(())
    })?;

    Ok(())
}
