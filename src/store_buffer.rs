//! Store buffer when failing to send events.

use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::Write;
use std::fmt::Debug;
use retry_conf::RetryConf;
use error::FluentError;
use std::fs::OpenOptions;
use serde::ser::Serialize;
use dumpable::Dumpable;

/// Create file with write, create, append, and open option.
fn ensure_file_with_wca(path: PathBuf) -> Result<File, io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;
    Ok(file)
}

/// Write events buffer into file with TSV format.
pub fn maybe_write_events<T>(conf: &RetryConf, events: T, err: FluentError) -> Result<(), FluentError>
where
    T: Serialize + Dumpable + Debug,
{
    let store_needed = conf.clone().need_to_store();
    let store_path = conf.clone().store_path();
    if store_needed {
        match ensure_file_with_wca(store_path.clone().unwrap()) {
            Ok(mut f) => {
                let mut w = Vec::new();
                write!(&mut w, "{}", events.dump()).unwrap();
                f.write_all(&w)?;
                f.sync_data()?;
                Err(FluentError::FileStored(format!(
                    "stored buffer in specified file: \
                                                     {:?}",
                    store_path.unwrap()
                )))
            },
            Err(e) => Err(From::from(e)),
        }
    } else {
        Err(err)
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use super::*;
    use time;
    use std::collections::HashMap;
    use self::tempdir::TempDir;
    use record::Record;
    use retry_conf::RetryConf;
    use error::FluentError;
    use forwardable::forward::Forward;
    #[cfg(not(feature = "time-as-integer"))]
    use event_time::EventTime;
    #[cfg(not(feature = "time-as-integer"))]
    use event_record::EventRecord;

    #[test]
    fn test_write_record() {
        let tag = "fruently".to_string();
        let time = time::now();
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = Record::new(tag.clone(), time, obj.clone());
        let tmp = TempDir::new("fruently").unwrap().into_path().join("buffer");
        let conf = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_events(&conf, record, FluentError::Dummy("dummy".to_string())).is_err());
        assert!(tmp.exists())
    }

    #[cfg(not(feature = "time-as-integer"))]
    #[test]
    fn test_write_event_record() {
        let tag = "fruently".to_string();
        let time = time::now();
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = EventRecord::new(tag.clone(), time, obj.clone());
        let tmp = TempDir::new("fruently").unwrap().into_path().join("buffer");
        let conf = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_events(&conf, record, FluentError::Dummy("dummy".to_string())).is_err());
        assert!(tmp.exists())
    }

    #[test]
    fn test_write_record_2_times() {
        let tag = "fruently".to_string();
        let time = time::now();
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = Record::new(tag.clone(), time, obj.clone());
        let tmp = TempDir::new("fruently").unwrap().into_path().join("buffer");
        let conf = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_events(&conf, record, FluentError::Dummy("dummy".to_string())).is_err());
        assert!(tmp.exists());
        let mut obj2: HashMap<String, String> = HashMap::new();
        obj2.insert("name2".to_string(), "fruently2".to_string());
        let record2 = Record::new(tag.clone(), time, obj2.clone());
        let conf2 = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_events(&conf2, record2, FluentError::Dummy("dummy".to_string())).is_err());
        assert!(tmp.exists())
    }

    #[test]
    fn test_write_forward_records() {
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
        let tag = "fruently".to_string();
        let mut obj1: HashMap<String, String> = HashMap::new();
        obj1.insert("hey".to_string(), "Rust with forward mode!".to_string());
        let mut obj2: HashMap<String, String> = HashMap::new();
        obj2.insert("yeah".to_string(), "Also sent together!".to_string());
        let time = make_time();
        let entry = (time.clone(), obj1);
        let entry2 = (time.clone(), obj2);
        let entries = vec![(entry), (entry2)];
        let forward = Forward::new(tag, entries);
        let tmp = TempDir::new("fruently").unwrap().into_path().join("buffer");
        let conf = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_events(&conf, forward, FluentError::Dummy("dummy".to_string())).is_err());
        assert!(tmp.exists())
    }
}
