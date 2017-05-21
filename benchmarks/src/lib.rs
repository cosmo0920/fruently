#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(test)]

extern crate test;
extern crate time;
extern crate fruently;
extern crate rust_fluent;

#[cfg(test)]
mod tests {
    use test;
    use test::Bencher;
    use time;
    use fruently::fluent::Fluent;
    use std::collections::HashMap;

    #[bench]
    fn benchmark_rust_fluent_tcp(b: &mut Bencher) {
        use rust_fluent::tcp;
        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("json".to_string(), "value".to_string());

        b.iter(|| {
            let fluentd = tcp::Fluentd::new("0.0.0.0:24224", "test");
            let _ = fluentd.write(&obj);
        });
    }

    #[bench]
    fn benchmark_json_forwardable(b: &mut Bencher) {
        use fruently::forwardable::JsonForwardable;

        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("json".to_string(), "fruently".to_string());
        b.iter(|| {
            let fruently = Fluent::new("0.0.0.0:24224", "test");
            let _ = fruently.post(&obj);
        });
    }

    #[bench]
    fn benchmark_msgpack_forwardable(b: &mut Bencher) {
        use fruently::forwardable::MsgpackForwardable;

        let mut obj: HashMap<String, String> = HashMap::new();
        obj.insert("msgp".to_string(), "fruently".to_string());
        b.iter(|| {
            let fruently = Fluent::new("0.0.0.0:24224", "test");
            let _ = fruently.post(&obj);
        });
    }

    #[bench]
    fn benchmark_forwardable(b: &mut Bencher) {
        use fruently::forwardable::Forwardable;
        use fruently::event_time::EventTime;

        let mut obj = HashMap::new();
        let mut hmap = HashMap::new();
        let time = time::now();
        b.iter(|| {
            let n = test::black_box(1000);
            let thmap = (0..n).fold(&mut obj, |mut acc, _| {
                {
                    acc.insert("fwd", "fruently".to_string());
                }
                acc
            });
            hmap = thmap.clone();
        });
        let entry = (EventTime::new(time), hmap);
        let fruently = Fluent::new("0.0.0.0:24224", "test");
        let _ = fruently.post(vec![(entry)]);
    }
}
