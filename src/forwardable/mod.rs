use rustc_serialize::Encodable;
use time;
use record::FluentError;

pub trait JsonForwardable {
    fn post<T: Encodable>(self, record: T) -> Result<(), FluentError>;
    fn post_with_time<T: Encodable>(self, record: T, time: time::Tm) -> Result<(), FluentError>;
}

pub mod json;
