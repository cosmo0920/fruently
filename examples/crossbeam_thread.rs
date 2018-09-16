extern crate fruently;
#[cfg(feature = "crossbeam")]
extern crate crossbeam;

#[cfg(feature = "crossbeam")]
fn crossbeam_thread() {
    use fruently::fluent::Fluent;
    use std::collections::HashMap;
    use fruently::forwardable::JsonForwardable;

    let fruently = Fluent::new("127.0.0.1:24224", "test");
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("threaded".to_string(), "fruently".to_string());
    let _threads: Vec<_> = (0..10)
        .map(|_| {
            let obj = obj.clone();
            let fruently = fruently.clone();
            crossbeam::scope(|scope| {
                scope.spawn(|| { let _ = fruently.post(&obj); })
            });
        })
        .collect();
}
#[cfg(not(feature = "crossbeam"))]
fn crossbeam_thread() {}

fn main() {
    crossbeam_thread();
}
