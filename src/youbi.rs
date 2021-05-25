use chrono::Weekday;
use std::fmt;
use Weekday::*;
use Youbi::*;

pub enum Youbi {
    Getsuyoubi,
    Kayoubi,
    Suiyoubi,
    Mokuyoubi,
    Kinyoubi,
    Doyoubi,
    Nichiyoubi,
}

impl From<Weekday> for Youbi {
    fn from(weekday: Weekday) -> Self {
        match weekday {
            Mon => Getsuyoubi,
            Tue => Kayoubi,
            Wed => Suiyoubi,
            Thu => Mokuyoubi,
            Fri => Kinyoubi,
            Sat => Doyoubi,
            Sun => Nichiyoubi,
        }
    }
}

impl fmt::Display for Youbi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f, "{}", match self {
            Getsuyoubi => "月",
            Kayoubi => "火",
            Suiyoubi => "水",
            Mokuyoubi => "木",
            Kinyoubi => "金",
            Doyoubi => "土",
            Nichiyoubi => "日",
        }
        }
    }
}
