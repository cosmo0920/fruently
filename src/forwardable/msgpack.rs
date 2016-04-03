use std::net::ToSocketAddrs;
use rustc_serialize::Encodable;
use time;
use retry::retry_exponentially;
use record::Record;
use record::FluentError;
use forwardable::MsgpackForwardable;
use fluent::Fluent;

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Message<T: Encodable> {
    tag: String,
    timesec: i64,
    record: T,
}

impl<T: Encodable> Message<T> {
    pub fn new(chunk: Record<T>) -> Message<T> {
        Message {
            tag: chunk.tag,
            timesec: chunk.time.to_timespec().sec,
            record: chunk.record,
        }
    }
}

impl<A: ToSocketAddrs> MsgpackForwardable for Fluent<A> {
    /// Post record into Fluentd. Without time version.
    fn post<T>(self, record: T) -> Result<(), FluentError>
        where T: Encodable
    {
        let time = time::now();
        return self.post_with_time(record, time);
    }

    /// Post record into Fluentd. With time version.
    fn post_with_time<T>(self, record: T, time: time::Tm) -> Result<(), FluentError>
        where T: Encodable
    {
        let record = Record::new(self.get_tag(), time, record);
        let message = record.to_message();
        let addr = self.get_addr();
        let (max, multiplier) = self.get_conf().build();
        match retry_exponentially(max,
                                  multiplier,
                                  || Fluent::closure_send_as_msgpack(addr, &message),
                                  |response| response.is_ok()) {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature="fluentd")]
    use time;
    #[cfg(feature="fluentd")]
    use fluent::Fluent;

    #[test]
    #[cfg(feature="fluentd")]
    fn test_post() {
        use std::collections::HashMap;
        use forwardable::MsgpackForwardable;

        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust with msgpack!".to_string());
        let result = fruently.post(obj).is_ok();
        assert_eq!(true, result);
    }

    #[test]
    #[cfg(feature="fluentd")]
    fn test_post_with_time() {
        use std::collections::HashMap;
        use forwardable::MsgpackForwardable;

        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust with msgpack!".to_string());
        let time = time::now();
        let result = fruently.post_with_time(obj, time).is_ok();
        assert_eq!(true, result);
    }
}
