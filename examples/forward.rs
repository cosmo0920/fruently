extern crate fruently;
extern crate time;
use fruently::fluent::Fluent;
use std::collections::HashMap;
use fruently::forwardable::Forwardable;
#[cfg(not(feature = "time-as-integer"))]
use fruently::event_time::EventTime;

#[cfg(not(feature = "time-as-integer"))]
fn event_time_main() {
    let fruently = Fluent::new("127.0.0.1:24224", "test");
    let mut obj = HashMap::new();
    let time = time::now();
    let thmap = (0..10).fold(&mut obj, |acc, i| {
        {
            acc.insert(format!("fwd{}", i), "fruently".to_string());
        }
        acc
    });
    let hmap = thmap.clone();
    let entry = (EventTime::new(time), hmap);
    let _ = fruently.post(vec![(entry.clone()), (entry.clone())]);
}

#[cfg(feature = "time-as-integer")]
fn integer_time_main() {
    let fruently = Fluent::new("127.0.0.1:24224", "test");
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
    let _ = fruently.post(vec![(entry.clone()), (entry.clone())]);
}

fn main() {
    #[cfg(not(feature = "time-as-integer"))] event_time_main();
    #[cfg(feature = "time-as-integer")] integer_time_main();
}
