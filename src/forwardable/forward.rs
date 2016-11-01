//! Send records as forward mode.
//!
//! ## Usage
//!
//! This trait is used as follows:
//!
//! ```no_run
//! extern crate fruently;
//! extern crate time;
//! use fruently::fluent::Fluent;
//! use std::collections::HashMap;
//! use fruently::forwardable::Forwardable;
//!
//! fn main() {
//!     let fruently = Fluent::new("127.0.0.1:24224", "test");
//!     let mut obj1: HashMap<String, String> = HashMap::new();
//!     obj1.insert("hey".to_string(), "Rust with forward mode!".to_string());
//!     let mut obj2: HashMap<String, String> = HashMap::new();
//!     obj2.insert("yeah".to_string(), "Also sent together!".to_string());
//!     let time = time::now().to_timespec().sec;
//!     let entry = (time, obj1);
//!     let entry2 = (time, obj2);
//!     let _ = fruently.post(vec![(entry), (entry2)]);
//! }
//! ```

use std::fmt::Debug;
use std::net::ToSocketAddrs;
use rustc_serialize::Encodable;
use retry::retry_exponentially;
use record::FluentError;
use forwardable::{Entry, Forwardable};
use fluent::Fluent;
use store_buffer;

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Forward<T: Encodable> {
    tag: String,
    entries: Vec<Entry<T>>,
}

impl<T: Encodable> Forward<T> {
    pub fn new(tag: String, entries: Vec<Entry<T>>) -> Forward<T> {
        Forward {
            tag: tag,
            entries: entries,
        }
    }
}

impl<'a, A: ToSocketAddrs> Forwardable for Fluent<'a, A> {
    /// Post `Vec<Entry<T>>` into Fluentd.
    fn post<T>(self, entries: Vec<Entry<T>>) -> Result<(), FluentError>
        where T: Encodable + Debug
    {
        let forward = Forward::new(self.get_tag().into_owned(), entries);
        let addr = self.get_addr();
        let (max, multiplier) = self.get_conf().into_owned().build();
        match retry_exponentially(max,
                                  multiplier,
                                  || Fluent::closure_send_as_forward(addr, &forward),
                                  |response| response.is_ok()) {
            Ok(_) => Ok(()),
            Err(err) => {
                store_buffer::maybe_write_records(&self.get_conf(), forward, err)
            },
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
        use forwardable::Forwardable;

        // 0.0.0.0 does not work in Windows....
        let fruently = Fluent::new("127.0.0.1:24224", "test");
        let mut obj1: HashMap<String, String> = HashMap::new();
        obj1.insert("hey".to_string(), "Rust with forward mode!".to_string());
        let mut obj2: HashMap<String, String> = HashMap::new();
        obj2.insert("yeah".to_string(), "Also sent together!".to_string());
        let time = time::now().to_timespec().sec;
        let entry = (time, obj1);
        let entry2 = (time, obj2);
        let result = fruently.post(vec![(entry), (entry2)]).is_ok();
        assert_eq!(true, result);
    }
}
