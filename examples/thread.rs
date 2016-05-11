extern crate fruently;
use fruently::fluent::Fluent;
use std::thread;
use std::collections::HashMap;
use fruently::forwardable::JsonForwardable;

fn main() {
    let fruently = Fluent::new("0.0.0.0:24224", "test");
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "fruently".to_string());
    let child = thread::spawn(move || {
            let _ = fruently.post(&obj);
    });
    child.join().unwrap();
}
