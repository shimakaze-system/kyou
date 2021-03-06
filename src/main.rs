use chinese_number::{ChineseNumber, ChineseVariant};
use chrono::prelude::{Datelike, Local};
use kyou::{Tsuki, Youbi};

fn main() {
    let now = Local::today();
    let month = Tsuki::from(now.month());
    let day = now.day();
    let youbi = Youbi::from(now.weekday());
    println!(
        "{}{}日({})",
        month,
        day.to_lowercase_ten_thousand(ChineseVariant::Traditional),
        youbi
    );
}
