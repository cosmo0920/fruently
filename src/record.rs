use rustc_serialize::json;
use rustc_serialize::Encodable;
use time::Tm;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<T: Encodable> {
    tag: String,
    time: Tm,
    record: T,
}

pub enum FluentError {
    EncodeError(json::EncoderError),
    IOError(io::Error),
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
        let message = format!("[{},{},{},null]", tag, self.time.to_timespec().sec, record);
        Ok(message)
    }
}
