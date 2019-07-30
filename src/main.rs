#![warn(clippy::all)]
#[macro_use]
mod scale;
use scale::Scale;

fn main() {
    let scale = Scale::new();
    println!("Hello, world!");
}
