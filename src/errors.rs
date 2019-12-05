use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    StrError(std::str::Utf8Error),
    JsonError(json5::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::StrError(ref e) => e.fmt(f),
            Error::JsonError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::IoError(ref e) => Some(e),
            Error::StrError(ref e) => Some(e),
            Error::JsonError(ref e) => Some(e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IoError(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Error {
        Error::StrError(e)
    }
}

impl From<json5::Error> for Error {
    fn from(e: json5::Error) -> Error {
        Error::JsonError(e)
    }
}
