use std::error::Error;
use std::fmt::Display;
use std::io::Error as IoError;
use std::path::PathBuf;

pub type UserFacingResult<T> = Result<T, UserError>;
pub type LibraryResult<T> = Result<T, LibraryError>;

#[derive(Debug)]
pub enum LibraryError {
    IOError(IoError),
    InvalidConfiguration(String),
}
impl Error for LibraryError {}
impl Display for LibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LibraryError::IOError(e) => format!("IO Error: {}", e),
                LibraryError::InvalidConfiguration(e) => format!("Configuration Invalid: {}", e),
            }
        )
    }
}

impl From<IoError> for LibraryError {
    fn from(e: IoError) -> Self {
        Self::IOError(e)
    }
}
impl From<toml::de::Error> for LibraryError {
    fn from(e: toml::de::Error) -> Self {
        Self::InvalidConfiguration(format!("{}", e))
    }
}

#[derive(Debug)]
pub enum UserError {
    ConfigNotFound,
    DirectoryDoesNotExist(PathBuf),
    IOError(IoError),
    ConfigurationError(String),
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
                UserError::ConfigurationError(e) =>
                    format!("Cannot Process Configuration File: {}", e),
            }
        )
    }
}

impl From<IoError> for UserError {
    fn from(e: IoError) -> Self {
        Self::IOError(e)
    }
}

impl From<LibraryError> for UserError {
    fn from(e: LibraryError) -> Self {
        match e {
            LibraryError::IOError(e) => UserError::IOError(e),
            LibraryError::InvalidConfiguration(e) => UserError::ConfigurationError(e),
        }
    }
}
