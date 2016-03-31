use std::net::ToSocketAddrs;
use std::convert::AsRef;
use std::net;
use std::io::Write;
use rustc_serialize::Encodable;
use time;
use retry::retry;
use record::Record;
use record::FluentError;
use retry_conf::RetryConf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fluent<A>
    where A: ToSocketAddrs
{
    addr: A,
    tag: String,
    conf: RetryConf,
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
            conf: RetryConf::new(),
        }
    }

    pub fn new_with_conf<T>(addr: A, tag: T, conf: RetryConf) -> Fluent<A>
        where T: AsRef<str>
    {
        Fluent {
            addr: addr,
            tag: format!("{}", tag.as_ref()),
            conf: conf,
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
        let record = Record::new(self.tag.clone(), time, record);
        let message = try!(record.make_forwardable_json());
        let addr = self.addr;
        let (max, timeout) = self.conf.build();
        match retry(max,
                    timeout,
                    || Fluent::closure_send_data(&addr, message.clone()),
                    |response| response.is_ok()) {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }

    fn closure_send_data(addr: &A, message: String) -> Result<(), FluentError> {
        let mut stream = try!(net::TcpStream::connect(addr));
        let result = stream.write(&message.into_bytes());
        drop(stream);

        match result {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature="fluentd")]
    use time;
    use retry_conf::RetryConf;

    #[test]
    fn create_fruently() {
        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let expected = Fluent {
            addr: "0.0.0.0:24224",
            tag: "test".to_string(),
            conf: RetryConf::new(),
        };
        assert_eq!(expected, fruently);
    }

    #[test]
    #[cfg(feature="fluentd")]
    fn test_post() {
        use std::collections::HashMap;

        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust!".to_string());
        let result = fruently.post(obj).is_ok();
        assert_eq!(true, result);
    }

    #[test]
    #[cfg(feature="fluentd")]
    fn test_post_with_time() {
        use std::collections::HashMap;

        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust!".to_string());
        let time = time::now();
        let result = fruently.post_with_time(obj, time).is_ok();
        assert_eq!(true, result);
    }
}
