extern crate fruently;
extern crate time;
use fruently::fluent::Fluent;
use std::collections::HashMap;
use fruently::forwardable::Forwardable;

fn main() {
    let fruently = Fluent::new("0.0.0.0:24224", "test");
    let mut obj = HashMap::new();
    let time = time::now().to_timespec().sec;
    let thmap = (0..10).fold(&mut obj, |mut acc, i| {
        {
            acc.insert(format!("fwd{}", i), "fruently".to_string());
        }
        acc
    });
    let hmap = thmap.clone();
    let entry = (time, hmap);
    let _ = fruently.post(vec![(entry.clone()),(entry.clone())]);
}
