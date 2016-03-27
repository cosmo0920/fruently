Fruently
===

[![Build Status](https://travis-ci.org/cosmo0920/fruently.svg?branch=master)](https://travis-ci.org/cosmo0920/fruently)

A yet another Fluentd logger for Rust.

### Usage

Add this to your Cargo.toml:

```toml
[dependencies]
fruently = "0.1.1"
```

and this to your crate root:

```rust
extern crate fruently;
```

#### A complete example

```rust
extern crate fruently;
use fruently::fluent::Fluent;
use std::collections::HashMap;

fn main() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "fruently".to_string());
    let fruently = Fluent::new("0.0.0.0:24224", "test");
    let _ = fruently.post(&obj);
}
```

### LICENSE

[MIT](LICENSE).
