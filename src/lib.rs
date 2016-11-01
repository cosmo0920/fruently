#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! `fruently` is a yet another Fluentd logger for Rust.

extern crate rustc_serialize;
extern crate rmp_serialize;
extern crate time;
extern crate retry;
pub mod fluent;
pub mod record;
pub mod forwardable;
pub mod retry_conf;
pub mod store_buffer;
