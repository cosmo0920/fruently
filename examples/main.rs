extern crate fruently;
use fruently::fluent::Fluent;
use std::collections::HashMap;
use fruently::forwardable::JsonForwardable;

fn main() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "fruently".to_string());
    let fruently = Fluent::new("127.0.0.1:24224", "test");
    match fruently.post(&obj) {
        Err(e) => println!("{:?}", e),
        Ok(_) => return,
    }
}
