#![feature(test)]

extern crate test;
extern crate fruently;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use fruently::fluent::Fluent;
    use std::collections::HashMap;

    #[bench]
    fn benchmark_json_forwardable(b: &mut Bencher) {
        use fruently::forwardable::JsonForwardable;

        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        b.iter(|| {
            let fruently = Fluent::new("0.0.0.0:24224", "test");
            let _ = fruently.post(&obj);
        });
    }

    #[bench]
    fn benchmark_msgpack_forwardable(b: &mut Bencher) {
        use fruently::forwardable::MsgpackForwardable;

        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("name".to_string(), "fruently".to_string());
        b.iter(|| {
            let fruently = Fluent::new("0.0.0.0:24224", "test");
            let _ = fruently.post(&obj);
        });
    }
}
