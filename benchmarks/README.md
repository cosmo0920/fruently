Fruently Benchmarks
===

Benchmarking purpose only internal crate(It will not be release to crates.io!).

**disclaimer**

Because __still__ `test` crate is marked as unstable, benchmarking tests should be splitted into other crate from original one for compiling benchmarking target crate with stable Rust.

### Benchmarking result

```log
% cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/fruently_benchmarks-e456f25ba1641bf0

running 4 tests
test tests::benchmark_forwardable         ... bench:      68,021 ns/iter (+/- 9,985)
test tests::benchmark_json_forwardable    ... bench:     116,517 ns/iter (+/- 28,595,299)
test tests::benchmark_msgpack_forwardable ... bench:     234,518 ns/iter (+/- 33,364,971)
test tests::benchmark_rust_fluent_tcp     ... bench:     133,353 ns/iter (+/- 39,989,065)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```

### LICENSE

[MIT](LICENSE).
