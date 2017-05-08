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


#[cfg(test)]
mod tests {
    use super::*;
    use time;
    use time::Timespec;
    use std::collections::HashMap;
    use rmp_serde::encode::Serializer;

    #[test]
    fn test_msgpack_format() {
        let tag = "fruently".to_string();
        let timespec = Timespec::new(1494245571, 0);
        let time = time::at(timespec);
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = EventRecord::new(tag.clone(), time, obj.clone());
        let mut buf = vec![];
        let _ = record.serialize(&mut Serializer::new(&mut buf)).unwrap();
        assert_eq!(vec![0x94, 0xa8, 0x66, 0x72, 0x75, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0xce, 0x59, 0x10,
                        0x60, 0xc3, 0x81, 0xa4, 0x6e, 0x61, 0x6d, 0x65, 0xa8, 0x66, 0x72, 0x75, 0x65,
                        0x6e, 0x74, 0x6c, 0x79, 0xc0], buf);
    }
}
