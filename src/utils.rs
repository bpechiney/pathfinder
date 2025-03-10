use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum UtilsError {
  Io(#[from] IoError),
  Serde(#[from] serde_json::Error),
}

impl Display for UtilsError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.source().unwrap())
  }
}

pub(crate) fn get_workspace_root() -> Result<PathBuf, UtilsError> {
  let json = match Command::new("cargo")
    .args(["metadata", "--no-deps", "--format-version", "1"])
    .output()
  {
    Ok(o) => String::from_utf8(o.stdout).unwrap(),
    Err(err) => return Err(UtilsError::from(err)),
  };

  parse_metadata(json)
}

fn parse_metadata(json: String) -> Result<PathBuf, UtilsError> {
  // Parse JSON output
  let manifest: Manifest = serde_json::from_str(json.as_str()).map_err(UtilsError::from)?;

  Ok(PathBuf::from(manifest.workspace_root))
}

#[derive(Debug, Deserialize)]
struct Manifest {
  workspace_root: String,
}

#[cfg(test)]
mod tests {
  // TODO: tests
}
