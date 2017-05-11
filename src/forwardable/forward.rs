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
//! #[cfg(not(feature = "time-as-integer"))]
//! use fruently::event_time::EventTime;
//!
//! // Using with Fluentd v0.14.
//! #[cfg(not(feature = "time-as-integer"))]
//! fn main() {
//!     let fruently = Fluent::new("127.0.0.1:24224", "test");
//!     let mut obj1: HashMap<String, String> = HashMap::new();
//!     obj1.insert("hey".to_string(), "Rust with forward mode!".to_string());
//!     let mut obj2: HashMap<String, String> = HashMap::new();
//!     obj2.insert("yeah".to_string(), "Also sent together!".to_string());
//!     let time = time::now();
//!     let entry = (EventTime::new(time), obj1);
//!     let entry2 = (EventTime::new(time), obj2);
//!     let _ = fruently.post(vec![(entry), (entry2)]);
//! }
//!
//! // Using with Fluentd v0.12.
//! #[cfg(feature = "time-as-integer")]
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
use dumpable::Dumpable;
#[cfg(not(feature = "time-as-integer"))]
use event_time::EventTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
}

impl<T: Serialize> Dumpable for Forward<T> {
    fn dump(self) -> String {
        #[inline]
        #[cfg(feature = "time-as-integer")]
        fn make_timespec(time: &i64) -> Timespec {
            Timespec::new(time.to_owned(), 0)
        }
        #[inline]
        #[cfg(not(feature = "time-as-integer"))]
        fn make_timespec(event_time: &EventTime) -> Timespec {
            Timespec::new(event_time.get_time().to_timespec().sec.to_owned(), 0)
        }
        let mut buf = String::new();
        for &(ref event_time_or_time, ref record) in &self.entries {
            let timespec = make_timespec(event_time_or_time);
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
                store_buffer::maybe_write_events(&self.get_conf(), forward, From::from(err))
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature="fluentd")]
mod tests {
    use time;
    use fluent::Fluent;
    #[cfg(not(feature = "time-as-integer"))]
    use event_time::EventTime;

    #[test]
    fn test_post() {
        #[inline]
        #[cfg(not(feature = "time-as-integer"))]
        fn make_time() -> EventTime {
            EventTime::new(time::now())
        }
        #[inline]
        #[cfg(feature = "time-as-integer")]
        fn make_time() -> i64 {
            time::now().to_timespec().sec
        }
        use std::collections::HashMap;
        use forwardable::Forwardable;

        // 0.0.0.0 does not work in Windows....
        let fruently = Fluent::new("127.0.0.1:24224", "test");
        let mut obj1: HashMap<String, String> = HashMap::new();
        obj1.insert("hey".to_string(), "Forward mode with EventTime!".to_string());
        let mut obj2: HashMap<String, String> = HashMap::new();
        obj2.insert("yeah".to_string(), "Yep, also sent together!".to_string());
        let time = make_time();
        let entry = (time.clone(), obj1);
        let entry2 = (time.clone(), obj2);
        let result = fruently.post(vec![(entry), (entry2)]).is_ok();
        assert_eq!(true, result);
    }
}
