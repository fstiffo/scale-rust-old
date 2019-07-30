#![warn(clippy::all)]
#[macro_use]
extern crate decimal;
extern crate chrono;
extern crate kairos;
mod scale;
use scale::setup_attuale;

fn main() {
    println!("Hello, world!");
    let attuale = setup_attuale();
}
