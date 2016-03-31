use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::Encodable;
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
    EncodeError(json::EncoderError),
    IOError(io::Error),
    RetryError(retry::RetryError),
}

impl From<io::Error> for FluentError {
    fn from(err: io::Error) -> FluentError {
        FluentError::IOError(err)
    }
}

impl From<json::EncoderError> for FluentError {
    fn from(err: json::EncoderError) -> FluentError {
        FluentError::EncodeError(err)
    }
}

impl From<retry::RetryError> for FluentError {
    fn from(err: retry::RetryError) -> FluentError {
        FluentError::RetryError(err)
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
        let tag = try!(json::encode(&self.tag));
        let record = try!(json::encode(&self.record));
        let option = Json::Null;
        let message = format!("[{},{},{},{}]",
                              tag,
                              self.time.to_timespec().sec,
                              record,
                              option);
        Ok(message)
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
