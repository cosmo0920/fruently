use std::fs::File;
use std::io::Write;
use std::fmt::Debug;
use retry_conf::RetryConf;
use rustc_serialize::Encodable;
use record::FluentError;
use forwardable::forward::Forward;

pub fn maybe_write_record<T>(conf: &RetryConf, record: T, err: FluentError) -> Result<(), FluentError>
    where T: Encodable + Debug
{
    let store_needed = conf.clone().need_to_store();
    let store_path = conf.clone().store_path();
    if store_needed && store_path.is_some() {
        match File::create(store_path.unwrap()) {
            Ok(mut f) => {
                let mut w = Vec::new();
                write!(&mut w, "{:?}", record).unwrap();
                try!(f.write(&w));
            },
            Err(e) => return Err(From::from(e)),
        }
        Ok(())
    }
    else {
        Err(err)
    }
}

pub fn maybe_write_records<T>(conf: &RetryConf, forward: Forward<T>, err: FluentError) -> Result<(), FluentError>
    where T: Encodable + Debug
{
    let store_needed = conf.clone().need_to_store();
    let store_path = conf.clone().store_path();
    if store_needed && store_path.is_some() {
        match File::create(store_path.unwrap()) {
            Ok(mut f) => {
                let mut w = Vec::new();
                write!(&mut w, "{:?}", forward).unwrap();
                try!(f.write(&w));
            },
            Err(e) => return Err(From::from(e)),
        }
        Ok(())
    }
    else {
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
    use record::FluentError;
    use forwardable::forward::Forward;

    #[test]
    fn test_write_record() {
        let tag = "fruently".to_string();
        let time = time::now();
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        let record = Record::new(tag.clone(), time, obj.clone());
        let tmp = TempDir::new("fruently").unwrap().into_path().join("buffer");
        let conf = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_record(&conf, record, FluentError::Dummy("dummy".to_string())).is_ok());
        assert!(tmp.exists())
    }

    #[test]
    fn test_write_records() {
        let tag = "fruently".to_string();
        let mut obj1: HashMap<String, String> = HashMap::new();
        obj1.insert("hey".to_string(), "Rust with forward mode!".to_string());
        let mut obj2: HashMap<String, String> = HashMap::new();
        obj2.insert("yeah".to_string(), "Also sent together!".to_string());
        let time = time::now().to_timespec().sec;
        let entry = (time, obj1);
        let entry2 = (time, obj2);
        let entries = vec![(entry), (entry2)];
        let forward = Forward::new(tag, entries);
        let tmp = TempDir::new("fruently").unwrap().into_path().join("buffer");
        let conf = RetryConf::new().store_file(tmp.clone());
        assert!(maybe_write_records(&conf, forward, FluentError::Dummy("dummy".to_string())).is_ok());
        assert!(tmp.exists())
    }
}
