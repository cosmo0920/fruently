//! Implement record manupulation mechanisms.

use time;
use time::Tm;
use serde_json;
use serde::ser::{Serialize, Serializer};
use serde::ser::SerializeTuple;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<T: Serialize> {
    tag: String,
    time: Tm,
    record: T,
}

impl<T: Serialize> Record<T> {
    pub fn new(tag: String, time: Tm, record: T) -> Record<T> {
        Record {
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
impl<T: Serialize> Serialize for Record<T> {
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
    use std::collections::HashMap;
    use serde_json;

    #[test]
    fn test_json_format() {
        let tag = "fruently".to_string();
        let time = time::now();
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = Record::new(tag.clone(), time, obj.clone());
        let forwardable_json = serde_json::to_string(&record).ok().unwrap();
        let json_tag = serde_json::to_string(&tag.clone()).ok().unwrap();
        let json_obj = serde_json::to_string(&obj.clone()).ok().unwrap();
        let expected = format!("[{},{},{},{}]",
                               json_tag,
                               time.to_timespec().sec,
                               json_obj,
                               serde_json::Value::Null);
        assert_eq!(expected, forwardable_json);
    }
}
