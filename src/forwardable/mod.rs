use rustc_serialize::Encodable;
use time;
use record::FluentError;

pub type Entry<T> where T: Encodable = (i64, T);

pub trait JsonForwardable {
    fn post<T: Encodable>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Encodable>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub trait MsgpackForwardable {
    fn post<T: Encodable>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Encodable>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub trait Forwardable {
    fn post<T: Encodable>(self, entries: Vec<Entry<T>>) -> Result<(), FluentError>;
}

pub mod json;
pub mod msgpack;
pub mod forward;
