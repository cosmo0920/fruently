use time::Tm;
use serde::ser::{Serialize, Serializer};
use byteorder::{BigEndian, WriteBytesExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTime {
    time: Tm,
}

impl EventTime {
    pub fn new(time: Tm) -> EventTime {
        EventTime { time: time }
    }

    pub fn get_time(&self) -> &Tm {
        &self.time
    }
}

impl Serialize for EventTime {
    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
    //
    // although it may also be generic over the input types T.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let mut buf = vec![];
        buf.write_u8(0xd7).map_err(Error::custom)?;
        buf.write_u8(0x00).map_err(Error::custom)?;
        buf.write_i32::<BigEndian>(self.clone().time.clone().to_timespec().sec as i32)
            .map_err(Error::custom)?;
        buf.write_i32::<BigEndian>(self.clone().time.clone().to_timespec().nsec as i32)
            .map_err(Error::custom)?;
        serializer.serialize_bytes(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time;
    use time::Timespec;
    use rmp_serde::encode::Serializer;

    #[test]
    fn test_event_time_format() {
        let timespec = Timespec::new(1494245571, 0);
        let time = time::at(timespec);
        let event_time = EventTime::new(time);
        let mut buf = vec![];
        let _ = event_time
            .serialize(&mut Serializer::new(&mut buf))
            .unwrap();
        assert_eq!(
            vec![0xc4, 0x0a, 0xd7, 0x00, 0x59, 0x10, 0x60, 0xc3, 0x00, 0x00, 0x00, 0x00],
            buf
        );
    }
}
