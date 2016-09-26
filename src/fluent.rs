//! Send record(s) into Fluentd.

use std::borrow::{Borrow, Cow};
use std::net::ToSocketAddrs;
use std::net;
use std::io::Write;
use record::{FluentError, Record};
use retry_conf::RetryConf;
use forwardable::forward::Forward;
use rustc_serialize::Encodable;
use rmp_serialize::Encoder;

#[derive(Debug, Clone, PartialEq)]
pub struct Fluent<'a, A>
    where A: ToSocketAddrs
{
    addr: A,
    tag: Cow<'a, str>,
    conf: RetryConf,
}

impl<'a, A: ToSocketAddrs> Fluent<'a, A> {
    /// Create Fluent type.
    ///
    /// ### Usage
    ///
    /// ```
    /// use fruently::fluent::Fluent;
    /// let fruently_with_str_tag = Fluent::new("127.0.0.1:24224", "test");
    /// let fruently_with_string_tag = Fluent::new("127.0.0.1:24224", "test".to_string());
    /// ```
    pub fn new<T>(addr: A, tag: T) -> Fluent<'a, A>
        where T: Into<Cow<'a, str>>
    {
        Fluent {
            addr: addr,
            tag: tag.into(),
            conf: RetryConf::new(),
        }
    }

    pub fn new_with_conf<T>(addr: A, tag: T, conf: RetryConf) -> Fluent<'a, A>
        where T: Into<Cow<'a, str>>
    {
        Fluent {
            addr: addr,
            tag: tag.into(),
            conf: conf,
        }
    }

    #[doc(hidden)]
    pub fn get_addr(&self) -> &A {
        self.addr.borrow()
    }

    #[doc(hidden)]
    pub fn get_tag(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(&self.tag)
    }

    #[doc(hidden)]
    pub fn get_conf(&self) -> Cow<RetryConf> {
        Cow::Borrowed(&self.conf)
    }

    #[doc(hidden)]
    /// For internal usage.
    pub fn closure_send_as_json(addr: &A, message: String) -> Result<(), FluentError> {
        let mut stream = try!(net::TcpStream::connect(addr));
        let result = stream.write(&message.into_bytes());
        drop(stream);

        match result {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }

    #[doc(hidden)]
    /// For internal usage.
    pub fn closure_send_as_msgpack<T: Encodable>(addr: &A,
                                                 record: &Record<T>)
                                                 -> Result<(), FluentError> {
        let mut stream = try!(net::TcpStream::connect(addr));
        let mut encoder = Encoder::new(&mut stream);
        let result = record.encode(&mut encoder);

        match result {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }

    #[doc(hidden)]
    /// For internal usage.
    pub fn closure_send_as_forward<T: Encodable>(addr: &A,
                                                 forward: &Forward<T>)
                                                 -> Result<(), FluentError> {
        let mut stream = try!(net::TcpStream::connect(addr));
        let mut encoder = Encoder::new(&mut stream);
        let result = forward.encode(&mut encoder);

        match result {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use retry_conf::RetryConf;
    use std::borrow::Cow;

    #[test]
    fn create_fruently() {
        let fruently = Fluent::new("127.0.0.1:24224", "test");
        let expected = Fluent {
            addr: "127.0.0.1:24224",
            tag: Cow::Borrowed("test"),
            conf: RetryConf::new(),
        };
        assert_eq!(expected, fruently);
    }
}
