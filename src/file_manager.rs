use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::read_dir;
use std::io::Error as IoError;
use std::path::Path;
use thiserror::Error;

pub(crate) fn get_maze_names(dir: &Path) -> Result<Vec<String>, FileError> {
  let entries = match read_dir(dir) {
    Ok(dir) => dir,
    Err(err) => return Err(FileError::from(err)),
  };

  let mut file_names = Vec::new();

  for entry in entries {
    let entry = entry?;
    if let Some(file_name) = entry.file_name().to_str() {
      file_names.push(file_name.to_string());
    }
  }

  Ok(file_names)
}

#[derive(Debug, Error)]
pub(crate) enum FileError {
  Io(#[from] IoError),
}

impl Display for FileError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.source().unwrap())
  }
}

#[cfg(test)]
mod tests {
  // TODO: tests
}
