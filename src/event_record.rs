//! Implement EventTime record manupulation mechanisms.

use time;
use time::Tm;
use serde_json;
use serde::ser::{Serialize, Serializer};
use serde::ser::SerializeTuple;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRecord<T: Serialize> {
    tag: String,
    time: Tm,
    record: T,
}

impl<T: Serialize> EventRecord<T> {
    pub fn new(tag: String, time: Tm, record: T) -> EventRecord<T> {
        EventRecord {
            tag: tag,
            time: time,
            record: record,
        }
    }

    #[doc(hidden)]
    pub fn dump(self) -> String {
        format!("{}\t{}\t{}\n",
                time::strftime("%FT%T%z", &self.time).unwrap(),
                self.tag,
                serde_json::to_string(&self.record).unwrap())
    }
}

/// Construct custom encoding json/msgpack style.
///
/// Because `Record` struct should map following style json/msgpack:
///
/// `[tag, unixtime/eventtime, record]`
///
/// ref: https://github.com/fluent/fluentd/wiki/Forward-Protocol-Specification-v0#message-mode
impl<T: Serialize> Serialize for EventRecord<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_tuple(4)?;
        seq.serialize_element(&self.tag)?;
        seq.serialize_element(&self.time.to_timespec().sec)?;
        seq.serialize_element(&self.record)?;
        seq.serialize_element(&None::<T>)?;
        seq.end()
    }
}
