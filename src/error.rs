use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error{
    MissingParams,
    InvalidApiKey,
    Reqwest(reqwest::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            _ => None,
        }
    }
}

// 为自定义错误实现fmt::Display trait。
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingParams => write!(f, "MissingParams"),
            Error::InvalidApiKey => write!(f, "InvalidApiKey"),
            Error::Reqwest(e) => write!(f, "Reqwest: {}", e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;