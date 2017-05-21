Fruently Benchmarks
===

Benchmarking purpose only internal crate(It will not be release to crates.io!).

**disclaimer**

Because __still__ `test` crate is marked as unstable, benchmarking tests should be splitted into other crate from original one for compiling benchmarking target crate with stable Rust.

### Benchmarking result

```log
% rustup run nightly rustc --version
rustc 1.19.0-nightly (01951a61a 2017-05-20)
% rustup run nightly cargo bench
    Finished release [optimized] target(s) in 97.19 secs
     Running target/release/deps/fruently_benchmarks-2fd8041e19ce8fe3

running 4 tests
test tests::benchmark_forwardable         ... bench:      61,855 ns/iter (+/- 517)
test tests::benchmark_json_forwardable    ... bench:     124,372 ns/iter (+/- 20,038,743)
test tests::benchmark_msgpack_forwardable ... bench:     230,228 ns/iter (+/- 93,671)
test tests::benchmark_rust_fluent_tcp     ... bench:     125,479 ns/iter (+/- 22,255,023)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```

### LICENSE

[MIT](LICENSE).
