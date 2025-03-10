use crate::utils::get_workspace_root;
use lazy_static::lazy_static;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::create_dir;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use thiserror::Error;

const LOGGING_CONFIG: &str = "log4rs.yaml";
const MAZES_DIR: &str = "mazes";

lazy_static! {
  pub static ref MAZES_PATH: PathBuf = get_workspace_root().unwrap().join(MAZES_DIR);
}

#[derive(Debug, Error)]
pub(crate) enum InitError {
  Io(#[from] IoError),
  Logger(#[from] anyhow::Error),
}

impl Display for InitError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.source().unwrap())
  }
}

pub(crate) fn init() -> Result<(), InitError> {
  init_logger()?;
  check_mazes_dir(&MAZES_PATH)
}

fn init_logger() -> Result<(), InitError> {
  log4rs::init_file(LOGGING_CONFIG, Default::default()).map_err(InitError::from)
}

fn check_mazes_dir(path: &Path) -> Result<(), InitError> {
  log::trace!("checking for mazes directory");

  match path.exists() {
    true => Ok(()),
    false => create_dir(MAZES_PATH.as_path()).map_err(InitError::from),
  }
}
