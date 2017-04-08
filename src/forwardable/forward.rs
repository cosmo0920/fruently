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
use retry::retry_exponentially;
use time;
use time::Timespec;
use error::FluentError;
use forwardable::{Entry, Forwardable};
use fluent::Fluent;
use store_buffer;
use serde_json;
use serde::ser::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Forward<T: Serialize> {
    tag: String,
    entries: Vec<Entry<T>>,
}

impl<T: Serialize> Forward<T> {
    pub fn new(tag: String, entries: Vec<Entry<T>>) -> Forward<T> {
        Forward {
            tag: tag,
            entries: entries,
        }
    }

    #[doc(hidden)]
    pub fn dump(self) -> String {
        let mut buf = String::new();
        for &(ref time, ref record) in &self.entries {
            let timespec = Timespec::new(time.to_owned(), 0);
            buf.push_str(&*format!("{}\t{}\t{}\n",
                                   time::strftime("%FT%T%z", &time::at(timespec)).unwrap(),
                                   self.tag,
                                   serde_json::to_string(&record).unwrap()));
        }
        buf
    }
}

impl<'a, A: ToSocketAddrs> Forwardable for Fluent<'a, A> {
    /// Post `Vec<Entry<T>>` into Fluentd.
    fn post<T>(self, entries: Vec<Entry<T>>) -> Result<(), FluentError>
        where T: Serialize + Debug
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
                store_buffer::maybe_write_records(&self.get_conf(), forward, From::from(err))
            }
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
