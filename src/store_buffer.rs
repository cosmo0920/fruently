use std::fs::File;
use std::io::Write;
use std::fmt::Debug;
use retry_conf::RetryConf;
use retry::RetryError;
use rustc_serialize::Encodable;
use record::FluentError;

pub fn maybe_write_record<T>(conf: &RetryConf, record: T, err: RetryError) -> Result<(), FluentError>
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
        Err(From::from(err))
    }
}
