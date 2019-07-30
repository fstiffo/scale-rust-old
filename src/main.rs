#![warn(clippy::all)]
#[macro_use]
mod scale;
use scale::Scale;

fn main() {
    let scale = Scale::new();
    println!("{}", scale.cassa(), scale.tesoretto(scale::from_ymd!(2019,8,1)));
}
