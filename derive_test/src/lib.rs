use binary::{derive, Binary};

#[derive(derive::Binary)]
struct Example {
    a: u128,
    b: i64,
    c: f32,
}

#[derive(derive::Binary)]
struct Other(u128, i64);

#[derive(derive::Binary)]
enum WhatsIt {
    GoesEr(u128, u64),
    Pozer { x: f32, y: f64, z: i32 },
    Whaner,
}
