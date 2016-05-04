//! Send record as forwardable json.
//!
//! ## Usage
//!
//! This trait is used as follows:
//!
//! ```no_run
//! extern crate fruently;
//! use fruently::fluent::Fluent;
//! use std::collections::HashMap;
//! use fruently::forwardable::JsonForwardable;
//!
//! fn main() {
//!     let mut obj: HashMap<String, String> = HashMap::new();
//!     obj.insert("name".to_string(), "fruently".to_string());
//!     let fruently = Fluent::new("0.0.0.0:24224", "test");
//!     let _ = fruently.post(&obj);
//! }
//! ```

use std::net::ToSocketAddrs;
use rustc_serialize::Encodable;
use time;
use retry::retry_exponentially;
use record::{Record, FluentError};
use forwardable::JsonForwardable;
use fluent::Fluent;

impl<A: ToSocketAddrs> JsonForwardable for Fluent<A> {
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
        let message = try!(record.make_forwardable_json());
        let addr = self.get_addr();
        let (max, multiplier) = self.get_conf().build();
        match retry_exponentially(max,
                                  multiplier,
                                  || Fluent::closure_send_data(addr, message.clone()),
                                  |response| response.is_ok()) {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }
}

#[cfg(test)]
#[cfg(feature="fluentd")]
mod tests {
    use time;
    use fluent::Fluent;

    #[test]
    fn test_post() {
        use std::collections::HashMap;
        use forwardable::JsonForwardable;

        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust!".to_string());
        let result = fruently.post(obj).is_ok();
        assert_eq!(true, result);
    }

    #[test]
    fn test_post_with_time() {
        use std::collections::HashMap;
        use forwardable::JsonForwardable;

        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust!".to_string());
        let time = time::now();
        let result = fruently.post_with_time(obj, time).is_ok();
        assert_eq!(true, result);
    }
}
