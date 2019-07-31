#![warn(clippy::all)]

extern crate chrono;
use chrono::NaiveDate;

use cursive::views::TextView;
use cursive::Cursive;

mod scale;
#[macro_use(from_ymd)]
use scale::Scale;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', Cursive::quit);
    let scale = Scale::new();
    siv.add_layer(TextView::new(format!(
        "Tesoretto: {:04}\nPress q to quit.",
        scale.tesoretto(from_ymd!(2019, 8, 1))
    )));

    siv.run();
}
