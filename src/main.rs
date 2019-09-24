#![allow(dead_code)]

mod advent;
mod random;
mod future;

fn main() {

use std::str;

// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// We know these bytes are valid, so just use `unwrap()`.
let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);


    future::test_main();
}
