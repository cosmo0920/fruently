extern crate fruently;
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use std::collections::HashMap;
use std::thread;

fn main() {
    let fruently = Fluent::new("127.0.0.1:24224", "test");
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("threaded".to_string(), "fruently".to_string());
    let threads: Vec<_> = (0..10)
        .map(|_| {
            let obj = obj.to_owned();
            let fruently = fruently.to_owned();
            thread::spawn(move || {
                let _ = fruently.post(&obj);
            })
        })
        .collect();
    let _: Vec<_> = threads.into_iter().map(|thread| thread.join()).collect();
}
