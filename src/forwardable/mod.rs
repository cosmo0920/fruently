//! Implement concrete sending record(s) specifications.

use time;
use crate::error::FluentError;
use std::fmt::Debug;
use serde::ser::Serialize;
#[cfg(not(feature = "time-as-integer"))]
use crate::event_time::EventTime;

#[cfg(not(feature = "time-as-integer"))]
pub type Entry<T> = (EventTime, T);
#[cfg(feature = "time-as-integer")]
pub type Entry<T> = (i64, T);

pub trait JsonForwardable {
    fn post<T: Serialize + Debug + Clone>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Serialize + Debug + Clone>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub trait MsgpackForwardable {
    fn post<T: Serialize + Debug>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Serialize + Debug>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub trait Forwardable {
    fn post<T: Serialize + Debug>(self, entries: Vec<Entry<T>>) -> Result<(), FluentError>;
}

pub mod json;
pub mod msgpack;
pub mod forward;
