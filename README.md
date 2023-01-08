# Chaining

[![crates.io](https://img.shields.io/crates/v/chaining.svg)](https://crates.io/crates/chaining)
[![Released API docs](https://docs.rs/chaining/badge.svg)](https://docs.rs/chaining)
[![Continuous Integration](https://github.com/plippe/chaining/actions/workflows/ci.yml/badge.svg)](https://github.com/plippe/chaining/actions/workflows/ci.yml)

Adds chaining methods `tap` and `pipe` to every type. Inspired by Scala's [ChainingOps](https://www.scala-lang.org/api/current/scala/util/ChainingOps.html).

## Getting Started
Add `chaining` to your dependencies in your Cargo.toml file:

```toml
[dependencies]
...
chaining = "x.y.z"
...
```

## Examples
```rust
use chaining::*;

let times6 = |i: i8| i * 6;
let i = (1 - 2 - 3).pipe(times6).pipe(i8::abs);
assert_eq!(24, i);

let xs = &[1, 2, 3].tap(|xs| println!("debug {}", xs.len()));
assert_eq!(&[1, 2, 3], xs);
```
