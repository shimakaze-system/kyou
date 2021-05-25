use chrono::Month;
use std::fmt;
use Month::*;
use Tsuki::*;

pub enum Tsuki {
    Mtsuki,
    Kisaragi,
    Yayoi,
    Uzuki,
    Sazuki,
    Minazuki,
    Fumizuki,
    Hazuki,
    Nagatsuki,
    Kannazuki,
    Shimotsuki,
    Shiwasu,
}

impl From<Month> for Tsuki {
    fn from(month: Month) -> Self {
        match month {
            January => Mtsuki,
            February => Kisaragi,
            March => Yayoi,
            April => Uzuki,
            May => Sazuki,
            June => Minazuki,
            July => Fumizuki,
            August => Hazuki,
            September => Nagatsuki,
            October => Kannazuki,
            November => Shimotsuki,
            December => Shiwasu,
        }
    }
}

impl From<u32> for Tsuki {
    fn from(month: u32) -> Self {
        match month {
            1 => Mtsuki,
            2 => Kisaragi,
            3 => Yayoi,
            4 => Uzuki,
            5 => Sazuki,
            6 => Minazuki,
            7 => Fumizuki,
            8 => Hazuki,
            9 => Nagatsuki,
            10 => Kannazuki,
            11 => Shimotsuki,
            12 => Shiwasu,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Tsuki {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mtsuki => "睦月",
                Kisaragi => "如月",
                Yayoi => "弥生",
                Uzuki => "卯月",
                Sazuki => "皋月",
                Minazuki => "水无月",
                Fumizuki => "文月",
                Hazuki => "叶月",
                Nagatsuki => "长月",
                Kannazuki => "神无月",
                Shimotsuki => "霜月",
                Shiwasu => "师走",
            }
        )
    }
}
