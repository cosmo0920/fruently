//! Implement record manupulation mechanisms.

use rustc_serialize::json;
use rustc_serialize::{Encodable, Encoder};
use rmp_serialize::encode;
use time::Tm;
use std::io;
use retry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<T: Encodable> {
    tag: String,
    time: Tm,
    record: T,
}

pub enum FluentError {
    JsonEncode(json::EncoderError),
    MsgpackEncode(encode::Error),
    IO(io::Error),
    Retry(retry::RetryError),
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

impl From<json::EncoderError> for FluentError {
    fn from(err: json::EncoderError) -> FluentError {
        FluentError::JsonEncode(err)
    }
}

impl<T: Encodable> Record<T> {
    pub fn new(tag: String, time: Tm, record: T) -> Record<T> {
        Record {
            tag: tag,
            time: time,
            record: record,
        }
    }

    pub fn make_forwardable_json(self) -> Result<String, FluentError> {
        let message = try!(json::encode(&self));
        Ok(message)
    }
}

/// Construct custom encoding json/msgpack style.
///
/// Because `Record` struct should map following style json/msgpack:
///
/// `[tag, unixtime/eventtime, record]`
///
/// ref: https://github.com/fluent/fluentd/wiki/Forward-Protocol-Specification-v0#message-mode
impl<T: Encodable> Encodable for Record<T> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match *self {
            Record { tag: ref p_tag, time: ref p_time, record: ref p_record } => {
                encoder.emit_tuple(4, |encoder| {
                    try!(encoder.emit_tuple_arg(0, |encoder| p_tag.encode(encoder)));
                    try!(encoder.emit_tuple_arg(1, |encoder| {
                        p_time.to_timespec().sec.encode(encoder)
                    }));
                    try!(encoder.emit_tuple_arg(2, |encoder| p_record.encode(encoder)));
                    // Put `None::<T>` as-is for now.
                    try!(encoder.emit_tuple_arg(3, |encoder| None::<T>.encode(encoder)));
                    Ok(())
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time;
    use std::collections::HashMap;
    use rustc_serialize::json;
    use rustc_serialize::json::Json;

    #[test]
    fn test_json_format() {
        let tag = "fruently".to_string();
        let time = time::now();
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = Record::new(tag.clone(), time, obj.clone());
        let forwardable_json = record.make_forwardable_json().ok().unwrap();
        let json_tag = json::encode(&tag.clone()).ok().unwrap();
        let json_obj = json::encode(&obj.clone()).ok().unwrap();
        let expected = format!("[{},{},{},{}]",
                               json_tag,
                               time.to_timespec().sec,
                               json_obj,
                               Json::Null);
        assert_eq!(expected, forwardable_json);
    }
}
