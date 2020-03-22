//! Implement error types for fruently crate.

use retry;
use rmp_serde::encode;
use serde_json;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FluentError {
    JsonEncode(serde_json::Error),
    MsgpackEncode(encode::Error),
    IO(io::Error),
    Retry(retry::RetryError),
    FileStored(String),
    #[doc(hidden)]
    Dummy(String),
}

impl fmt::Display for FluentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FluentError::JsonEncode(ref e) => write!(f, "Fluent JSON encode error: {}", e),
            FluentError::MsgpackEncode(ref e) => write!(f, "Fluent msgpack encode error: {}", e),
            FluentError::IO(ref e) => write!(f, "Fluent IO error: {}", e),
            FluentError::Retry(ref e) => write!(f, "Fluent retry error: {}", e),
            FluentError::FileStored(ref e) => write!(f, "Fluent file stored error: {}", e),
            FluentError::Dummy(ref e) => write!(f, "Fluent dummy error: {}", e),
        }
    }
}

impl error::Error for FluentError {
    fn description(&self) -> &str {
        "FluentError"
    }
}

impl From<io::Error> for FluentError {
    fn from(err: io::Error) -> FluentError {
        FluentError::IO(err)
    }
}

impl From<encode::Error> for FluentError {
    fn from(err: encode::Error) -> FluentError {
        FluentError::MsgpackEncode(err)
    }
}

impl From<retry::RetryError> for FluentError {
    fn from(err: retry::RetryError) -> FluentError {
        FluentError::Retry(err)
    }
}

impl From<serde_json::Error> for FluentError {
    fn from(err: serde_json::Error) -> FluentError {
        FluentError::JsonEncode(err)
    }
}

#[cfg(test)]
mod tests {
    extern crate failure;
    use self::failure::Error;
    use super::*;
    use std;

    type Result<T> = std::result::Result<T, Error>;

    #[test]
    fn test_failure_err() {
        let f = || -> Result<()> {
            Err(FluentError::Dummy("".to_owned()))?;
            Ok(())
        };

        assert!(f().is_err());
    }
}
