Fruently Benchmarks
===

Benchmarking purpose only internal crate(It will not be release to crates.io!).

**disclaimer**

Because __still__ `test` crate is marked as unstable, benchmarking tests should be splitted into other crate from original one for compiling benchmarking target crate with stable Rust.

### Benchmarking result

```log
% cargo bench
     Running target/release/fruently_benchmarks-cdbf8acbc93a4065

running 4 tests
test tests::benchmark_forwardable         ... bench:      87,793 ns/iter (+/- 12,631)
test tests::benchmark_json_forwardable    ... bench:     103,450 ns/iter (+/- 22,250,756)
test tests::benchmark_msgpack_forwardable ... bench:     174,964 ns/iter (+/- 24,993,335)
test tests::benchmark_rust_fluent_tcp     ... bench:     101,613 ns/iter (+/- 20,036,600)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```

### LICENSE

[MIT](LICENSE).
