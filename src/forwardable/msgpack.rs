//! Send record as msgpack.
//!
//! ## Usage
//!
//! This trait is used as follows:
//!
//! ```no_run
//! extern crate fruently;
//! use fruently::fluent::Fluent;
//! use std::collections::HashMap;
//! use fruently::forwardable::MsgpackForwardable;
//!
//! fn main() {
//!     let mut obj: HashMap<String, String> = HashMap::new();
//!     obj.insert("name".to_string(), "fruently".to_string());
//!     let fruently = Fluent::new("127.0.0.1:24224", "test");
//!     let _ = fruently.post(&obj);
//! }
//! ```

use std::fmt::Debug;
use std::net::ToSocketAddrs;
use time;
use retry::retry_exponentially;
#[cfg(not(feature = "time-as-integer"))]
use crate::event_record::EventRecord;
#[cfg(feature = "time-as-integer")]
use crate::record::Record;
use crate::error::FluentError;
use crate::forwardable::MsgpackForwardable;
use crate::fluent::Fluent;
use crate::store_buffer;
use serde::ser::Serialize;

impl<'a, A: ToSocketAddrs> MsgpackForwardable for Fluent<'a, A> {
    /// Post record into Fluentd. Without time version.
    fn post<T>(self, record: T) -> Result<(), FluentError>
    where
        T: Serialize + Debug,
    {
        let time = time::now();
        self.post_with_time(record, time)
    }

    /// Post record into Fluentd. With time version.
    fn post_with_time<T>(self, record: T, time: time::Tm) -> Result<(), FluentError>
    where
        T: Serialize + Debug,
    {
        #[inline]
        #[cfg(feature = "time-as-integer")]
        fn make_record<T>(tag: String, time: time::Tm, record: T) -> Record<T>
        where
            T: Serialize + Debug,
        {
            Record::new(tag, time, record)
        }
        #[inline]
        #[cfg(not(feature = "time-as-integer"))]
        fn make_record<T>(tag: String, time: time::Tm, record: T) -> EventRecord<T>
        where
            T: Serialize + Debug,
        {
            EventRecord::new(tag, time, record)
        }
        let record = make_record(self.get_tag().into_owned(), time, record);
        let addr = self.get_addr();
        let (max, multiplier) = self.get_conf().into_owned().clone().build();
        match retry_exponentially(max, multiplier, || Fluent::closure_send_as_msgpack(addr, &record), |response| {
            response.is_ok()
        }) {
            Ok(_) => Ok(()),
            Err(err) => store_buffer::maybe_write_events(&self.get_conf(), record, From::from(err)),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "fluentd")]
mod tests {
    use time;
    use crate::fluent::Fluent;

    #[test]
    fn test_post() {
        use std::collections::HashMap;
        use crate::forwardable::MsgpackForwardable;

        // 0.0.0.0 does not work in Windows.
        let fruently = Fluent::new("127.0.0.1:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust with msgpack!".to_string());
        let result = fruently.post(obj).is_ok();
        assert_eq!(true, result);
    }

    #[test]
    fn test_post_with_time() {
        use std::collections::HashMap;
        use crate::forwardable::MsgpackForwardable;

        // 0.0.0.0 does not work in Windows.
        let fruently = Fluent::new("127.0.0.1:24224", "test");
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("hey".to_string(), "Rust with msgpack!".to_string());
        let time = time::now();
        let result = fruently.post_with_time(obj, time).is_ok();
        assert_eq!(true, result);
    }
}
