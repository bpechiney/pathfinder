extern crate core;

use std::error::Error;

mod file_manager;
mod init;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
  init::init()?;
  let mazes = file_manager::get_maze_names(init::MAZES_PATH.as_path())?;
  // TODO: replace with interactive prompt
  for maze in mazes {
    log::info!("{}", maze);
  }

  Ok(())
}
