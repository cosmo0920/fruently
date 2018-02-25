Fruently
===

[![Build Status](https://travis-ci.org/cosmo0920/fruently.svg?branch=master)](https://travis-ci.org/cosmo0920/fruently)
[![](http://meritbadge.herokuapp.com/fruently)](https://crates.io/crates/fruently)
[![Build status](https://ci.appveyor.com/api/projects/status/4qybnaegbiqs0738/branch/master?svg=true)](https://ci.appveyor.com/project/cosmo0920/fruently/branch/master)

[Documentation](http://cosmo0920.github.io/fruently/fruently/index.html)

A yet another Fluentd logger for Rust.

##### Note

If you use this library in Windows, please install Visual Studio 2015 or 2017 and rust compiler which is targeted for MSVC API and its package manager, which is called cargo via [rustup.rs](https://www.rustup.rs/).

And then, follow the below usage instructions.

### Usage

Add this to your Cargo.toml:

```toml
[dependencies]
fruently = "~0.10.0"
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
    let fruently = Fluent::new("127.0.0.1:24224", "test");
    match fruently.post(&obj) {
        Err(e) => println!("{:?}", e),
        Ok(_) => return,
    }
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
    let fruently = Fluent::new("127.0.0.1:24224", "test");
    match fruently.post(&obj) {
        Err(e) => println!("{:?}", e),
        Ok(_) => return,
    }
}
```

#### Forwarding asynchronously

Fruently does not have asynchronous API because Rust has `std::thread::spawn` function to make asynchronous API from synchronous one.

If you want to send records into Fluentd asynchronously, please consider using `std::thread::spawn` like: [asynchronous example](examples/thread.rs).

### Backward compatibility

Using with Fluentd v0.12, you must specify `time-as-integer` feature flag:

```toml
[build-dependencies.fruently]
version = "~0.10.0"
features = ["time-as-integer"]
```

### Related Articles

* EventTime: http://www.clear-code.com/blog/2017/5/24.html (ja)
* Fluent Logger Reliability: http://www.clear-code.com/blog/2017/4/28.html (ja)
* Fundamentals: http://www.clear-code.com/blog/2016/4/22.html (ja)

### LICENSE

[MIT](LICENSE).
