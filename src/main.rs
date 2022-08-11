use chinese_number::{ChineseNumber, ChineseVariant};
use color_eyre::eyre::Result;
use kyou::{Tsuki, Youbi};
use time::OffsetDateTime;

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = OffsetDateTime::now_local()?;
    let month = Tsuki::from(now.month());
    let day = now.day();
    let youbi = Youbi::from(now.weekday());
    println!(
        "{}{}日({})",
        month,
        day.to_lowercase_ten_thousand(ChineseVariant::Traditional),
        youbi
    );
    Ok(())
}
