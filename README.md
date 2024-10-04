# Binary

![build](https://github.com/SamuelSchlesinger/binary-rs/actions/workflows/rust.yml/badge.svg)

Dead simple binary serialization and deserialization in Rust.

```rust
use binary::{derive, Binary};

#[derive(derive::Binary)]
struct X { y: u32, z: f64 }

#[derive(derive::Binary)]
struct Y(i128);

#[derive(derive::Binary)]
struct Z;

#[derive(derive::Binary)]
enum K {
  A { a1: u32, a2: bool },
  B(char, i16),
  C,
}

#[test]
fn test_serialization() {
  let x = X { y: 10, z: 0.5 };
  assert_eq!(x, x.from_bytes(&x.to_bytes()).unwrap());
}
```
