//! Traits for dump record(s).

pub trait Dumpable {
    fn dump(self) -> String;
}
