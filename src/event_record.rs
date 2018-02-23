//! Implement `EventTime` record manupulation mechanisms.

use time;
use time::Tm;
use serde_json;
use serde::ser::{Serialize, Serializer};
use serde::ser::SerializeTuple;
use event_time::EventTime;
use dumpable::Dumpable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRecord<T: Serialize> {
    tag: String,
    event: Vec<Event<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Event<T: Serialize> {
    event_time: EventTime,
    record: T,
}

impl<T: Serialize> Event<T> {
    pub fn new(event_time: EventTime, record: T) -> Event<T> {
        Event {
            event_time: event_time,
            record: record,
        }
    }

    pub fn get_record(&self) -> &T {
        &self.record
    }

    pub fn get_event_time(&self) -> &EventTime {
        &self.event_time
    }
}

impl<T: Serialize> EventRecord<T> {
    pub fn new(tag: String, time: Tm, record: T) -> EventRecord<T> {
        EventRecord {
            tag: tag,
            event: vec![Event::new(EventTime::new(time), record)],
        }
    }
}

impl<T: Serialize> Dumpable for EventRecord<T> {
    fn dump(self) -> String {
        format!(
            "{}\t{}\t{}\n",
            time::strftime("%FT%T%z", self.event[0].get_event_time().get_time()).unwrap(),
            self.tag,
            serde_json::to_string(&self.event[0].get_record()).unwrap()
        )
    }
}

/// Construct custom encoding json/msgpack style.
///
/// Because `Record` struct should map following style msgpack with `ExtType`:
///
/// `[tag, [eventtime, record]]`
///
/// ref: <https://github.com/fluent/fluentd/wiki/Forward-Protocol-Specification-v1#message-modes>
impl<T: Serialize> Serialize for EventRecord<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_tuple(3)?;
        seq.serialize_element(&self.tag)?;
        seq.serialize_element(&self.event)?;
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
        assert_eq!(
            vec![
                0x93,
                0xa8,
                0x66,
                0x72,
                0x75,
                0x65,
                0x6e,
                0x74,
                0x6c,
                0x79,
                0x91,
                0x92,
                0xc4,
                0x0a,
                0xd7,
                0x00,
                0x59,
                0x10,
                0x60,
                0xc3,
                0x00,
                0x00,
                0x00,
                0x00,
                0x81,
                0xa4,
                0x6e,
                0x61,
                0x6d,
                0x65,
                0xa8,
                0x66,
                0x72,
                0x75,
                0x65,
                0x6e,
                0x74,
                0x6c,
                0x79,
                0xc0,
            ],
            buf
        );
    }
}
