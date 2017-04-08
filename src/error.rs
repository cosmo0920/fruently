//! Implement error types for fruently crate.

use std::io;
use retry;
use serde_json;
use rmp_serde::encode;

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
