extern crate fruently;
extern crate time;
use fruently::fluent::Fluent;
use std::collections::HashMap;
use fruently::forwardable::Forwardable;
use fruently::event_time::EventTime;

fn main() {
    let fruently = Fluent::new("127.0.0.1:24224", "test");
    let mut obj = HashMap::new();
    let time = time::now();
    let thmap = (0..10).fold(&mut obj, |mut acc, i| {
        {
            acc.insert(format!("fwd{}", i), "fruently".to_string());
        }
        acc
    });
    let hmap = thmap.clone();
    let entry = (EventTime::new(time), hmap);
    let _ = fruently.post(vec![(entry.clone()), (entry.clone())]);
}
