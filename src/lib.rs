#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! `fruently` is a yet another Fluentd logger for Rust.

extern crate time;
extern crate retry;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rmp;
extern crate rmp_serde;
extern crate byteorder;

pub mod fluent;
pub mod record;
pub mod forwardable;
pub mod retry_conf;
pub mod store_buffer;
pub mod error;
pub mod event_record;
