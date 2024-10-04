use binary::{derive, Binary};

#[derive(derive::Binary)]
struct Example {
    a: u128,
    b: i64,
    c: f32,
}

#[derive(derive::Binary)]
struct Other(u128, i64);
