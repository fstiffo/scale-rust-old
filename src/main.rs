#![warn(clippy::all)]

extern crate chrono;
use chrono::NaiveDate;

mod scale;
#[macro_use(from_ymd)]
use scale::Scale;

fn main() {
    let scale = Scale::new();
    println!("{}", scale.tesoretto(from_ymd!(2019, 8, 1)));
}
