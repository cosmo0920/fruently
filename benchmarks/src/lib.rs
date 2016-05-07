#![feature(test)]

extern crate test;
extern crate time;
extern crate fruently;

#[cfg(test)]
mod tests {
    use test;
    use test::Bencher;
    use time;
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

    #[bench]
    fn benchmark_forwardable(b: &mut Bencher) {
        use fruently::forwardable::Forwardable;

        let mut obj = HashMap::new();
        let time = time::now().to_timespec().sec;
        b.iter(|| {
            let n = test::black_box(1000);
            let hmap = (0..n).fold(&mut obj, |mut acc, _| {
                {
                    acc.insert("fwd", "fruently".to_string());
                }
                acc
            });
            let entry = (time, &*hmap);
            let fruently = Fluent::new("0.0.0.0:24224", "test");
            let _ = fruently.post(vec![(entry)]);
        })
    }
}
