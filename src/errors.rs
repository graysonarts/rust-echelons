use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

pub type UserFacingResult<T> = Result<T, UserError>;

pub enum LibraryError {}

#[derive(Debug)]
pub enum UserError {
    ConfigNotFound,
    DirectoryDoesNotExist(PathBuf),
    IOError(std::io::Error),
}

impl Error for UserError {}
impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UserError::ConfigNotFound => "No config file found, specify it with -c".to_owned(),
                UserError::DirectoryDoesNotExist(dir) => format!("{:?} not found", dir),
                UserError::IOError(e) => format!("Unable to complete Operation: {}", e),
            }
        )
    }
}

impl From<std::io::Error> for UserError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
