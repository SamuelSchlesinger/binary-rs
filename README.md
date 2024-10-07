# Binary

![build](https://github.com/SamuelSchlesinger/binary-rs/actions/workflows/rust.yml/badge.svg)

Dead simple binary serialization and deserialization in Rust.

```rust
use binary::{derive::Binary, Binary};

#[derive(Binary)]
struct X { y: u32, z: f64 }

#[derive(Binary)]
struct Y(i128);

#[derive(Binary)]
struct Z;

#[derive(Binary)]
enum K {
  A { a1: u32, a2: bool },
  B(char, i16),
  C,
}

struct M {
  a: i8,
  b: f32,
}

impl Binary for M {
  fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
    let (a, bs) = i8::parse(bs)?;
    let (b, bs) = f32::parse(bs)?;
    Some((M { a, b }, bs))
  }

  fn unparse(&self, bs: &mut Vec<u8>) {
    self.a.unparse(bs);
    self.b.unparse(bs);
  }
}

#[test]
fn test_serialization() {
  let x = X { y: 10, z: 0.5 };
  assert_eq!(x, x.from_bytes(&x.to_bytes()).unwrap());
}
```
