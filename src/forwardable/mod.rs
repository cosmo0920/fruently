//! Implement concrete sending record(s) specifications.

use rustc_serialize::Encodable;
use time;
use record::FluentError;
use std::fmt::Debug;

pub type Entry<T> where T: Encodable = (i64, T);

pub trait JsonForwardable {
    fn post<T: Encodable + Debug + Clone>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Encodable + Debug + Clone>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub trait MsgpackForwardable {
    fn post<T: Encodable + Debug>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Encodable + Debug>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub trait Forwardable {
    fn post<T: Encodable + Debug>(self, entries: Vec<Entry<T>>) -> Result<(), FluentError>;
}

pub mod json;
pub mod msgpack;
pub mod forward;
