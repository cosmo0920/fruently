Fruently
===

[![Build Status](https://travis-ci.org/cosmo0920/fruently.svg?branch=master)](https://travis-ci.org/cosmo0920/fruently)
[![](http://meritbadge.herokuapp.com/fruently)](https://crates.io/crates/fruently)

[Documentation](http://cosmo0920.github.io/fruently/fruently/index.html)

A yet another Fluentd logger for Rust.

### Usage

Add this to your Cargo.toml:

```toml
[dependencies]
fruently = "0.5.0"
```

and this to your crate root:

```rust,ignore
extern crate fruently;
```

#### Complete examples

#### Forwarding with JSON

```rust
extern crate fruently;
use fruently::fluent::Fluent;
use std::collections::HashMap;
use fruently::forwardable::JsonForwardable;

fn main() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "fruently".to_string());
    let fruently = Fluent::new("0.0.0.0:24224", "test");
    let _ = fruently.post(&obj);
}
```

#### Forwarding with msgpack

```rust
extern crate fruently;
use fruently::fluent::Fluent;
use std::collections::HashMap;
use fruently::forwardable::MsgpackForwardable;

fn main() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "fruently".to_string());
    let fruently = Fluent::new("0.0.0.0:24224", "test");
    let _ = fruently.post(&obj);
}
```

#### Forwarding asynchronously

Fruently does not have asynchronous API because Rust has `std::thread::spawn` function to make asynchronous API from synchronous one.

If you want to send records into Fluentd asynchronously, please consider using `std::thread::spawn` like: [asynchronous example](examples/thread.rs).

### LICENSE

[MIT](LICENSE).
