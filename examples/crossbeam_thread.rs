#[cfg(feature = "crossbeam")]
extern crate crossbeam_utils;
extern crate fruently;
#[cfg(feature = "crossbeam")]
use crossbeam_utils::thread;

#[cfg(feature = "crossbeam")]
fn crossbeam_thread() {
    use fruently::fluent::Fluent;
    use fruently::forwardable::JsonForwardable;
    use std::collections::HashMap;

    let fruently = Fluent::new("127.0.0.1:24224", "test");
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("threaded".to_string(), "fruently".to_string());
    let _threads: Vec<_> = (0..10)
        .map(|_| {
            let obj = obj.clone();
            let fruently = fruently.clone();
            thread::scope(|s| {
                s.spawn(|_| {
                    let _ = fruently.post(&obj);
                })
            });
        })
        .collect();
}
#[cfg(not(feature = "crossbeam"))]
fn crossbeam_thread() {}

fn main() {
    crossbeam_thread();
}
