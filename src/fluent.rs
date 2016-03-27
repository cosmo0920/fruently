use std::net::ToSocketAddrs;
use std::convert::AsRef;
use std::io;
use std::net;
use time;
use rustc_serialize::json;
use rustc_serialize::Encodable;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fluent<A>
    where A: ToSocketAddrs
{
    addr: A,
    tag: String,
}

pub enum FluentError {
    DecodeError(json::EncoderError),
    IOError(io::Error),
}

impl From<io::Error> for FluentError {
    fn from(err: io::Error) -> FluentError {
        FluentError::IOError(err)
    }
}

impl From<json::EncoderError> for FluentError {
    fn from(err: json::EncoderError) -> FluentError {
        FluentError::DecodeError(err)
    }
}

impl<A: ToSocketAddrs> Fluent<A> {
    /// Create Fluent type.
    ///
    /// ### Usage
    ///
    /// ```
    /// use fruently::fluent::Fluent;
    /// let fruently_with_str_tag = Fluent::new("0.0.0.0:24224", "test");
    /// let fruently_with_string_tag = Fluent::new("0.0.0.0:24224", "test".to_string());
    /// ```
    pub fn new<T>(addr: A, tag: T) -> Fluent<A>
        where T: AsRef<str>
    {
        Fluent {
            addr: addr,
            tag: format!("{}", tag.as_ref()),
        }
    }

    /// Post record into Fluentd. Without time version.
    pub fn post<T>(self, record: T) -> Result<(), FluentError>
        where T: Encodable
    {
        let time = time::now();
        return self.post_with_time(record, time);
    }

    /// Post record into Fluentd. With time version.
    pub fn post_with_time<T>(self, record: T, time: time::Tm) -> Result<(), FluentError>
        where T: Encodable
    {
        let tag = try!(json::encode(&self.tag));
        let record = try!(json::encode(&record));

        let message = format!("[{},{},{},null]", tag, time.to_timespec().sec, record);
        let mut stream = try!(net::TcpStream::connect(self.addr));
        let _ = stream.write(&message.into_bytes());
        drop(stream);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fruently() {
        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let expected = Fluent {
            addr: "0.0.0.0:24224",
            tag: "test".to_string(),
        };
        assert_eq!(expected, fruently);
    }
}
