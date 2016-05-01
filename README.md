Fruently
===

[![Build Status](https://travis-ci.org/cosmo0920/fruently.svg?branch=master)](https://travis-ci.org/cosmo0920/fruently)

[Documentation](http://cosmo0920.github.io/fruently/fruently/index.html)

A yet another Fluentd logger for Rust.

### Usage

Add this to your Cargo.toml:

```toml
[dependencies]
fruently = "0.4.0"
```

and this to your crate root:

```rust,ignore
extern crate fruently;
```

#### A complete example

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

### LICENSE

[MIT](LICENSE).
