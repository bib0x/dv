use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FlakePathError {
  Empty,
  NotFound(PathBuf),
}

impl std::error::Error for FlakePathError {}

impl Display for FlakePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlakePathError::Empty =>
                write!(f, "Empty Flake path. Use --path or DV_FLAKE_DIR environment variable"),
            FlakePathError::NotFound(path) =>
                write!(f, "Flake path {:?} not found", path),
        }
    }
}
