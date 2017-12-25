//! Japanese Era Library
extern crate chrono;
use chrono::prelude::*;

/// Start Meiji Era
pub fn meiji() -> DateTime<Utc> {
    Utc.ymd(1868, 9, 7).and_hms(15, 0, 0)
}

/// Start Tasho Era
pub fn taisyou() -> DateTime<Utc> {
    Utc.ymd(1912, 7, 29).and_hms(15, 0, 0)
}

/// Start Showa Era
pub fn shouwa() -> DateTime<Utc> {
    Utc.ymd(1926, 12, 24).and_hms(15, 0, 0)
}

/// Start Heisei Era
pub fn heisei() -> DateTime<Utc> {
    Utc.ymd(1989, 1, 6).and_hms(15, 0, 0)
}

/// get era from datetime
pub fn get_era<T: TimeZone>(dt: &DateTime<T>) -> Era {
    let dt = dt.with_timezone(&Utc);
    if dt < meiji() {
        Era::PreMeiji
    } else if dt < taisyou() {
        Era::Meiji
    } else if dt < shouwa() {
        Era::Taisho
    } else if dt < heisei() {
        Era::Showa
    } else {
        Era::Heisei
    }
}

/// get era year
pub fn get_year<T: TimeZone>(dt: &DateTime<T>) -> i32 {
    let year = dt.year();
    match get_era(dt) {
        Era::PreMeiji => year,
        Era::Meiji => year - meiji().year() + 1,
        Era::Taisho => year - taisyou().year() + 1,
        Era::Showa => year - shouwa().year() + 1,
        Era::Heisei => year - heisei().year() + 1,
    }
}

/// get full name
pub fn get_name<T: TimeZone>(dt: &DateTime<T>) -> &str {
    match get_era(dt) {
        Era::PreMeiji => "西暦",
        Era::Meiji => "明治",
        Era::Taisho => "大正",
        Era::Showa => "昭和",
        Era::Heisei => "平成",
    }
}

/// get short name
pub fn get_short_name<T: TimeZone>(dt: &DateTime<T>) -> &str {
    match get_era(dt) {
        Era::PreMeiji => "西暦",
        Era::Meiji => "明",
        Era::Taisho => "大",
        Era::Showa => "昭",
        Era::Heisei => "平",
    }
}

/// get abbrevivation name
pub fn get_abbreviation_name<T: TimeZone>(dt: &DateTime<T>) -> &str {
    match get_era(dt) {
        Era::PreMeiji => "西暦",
        Era::Meiji => "M",
        Era::Taisho => "T",
        Era::Showa => "S",
        Era::Heisei => "H",
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Era {
    PreMeiji,
    Meiji,
    Taisho,
    Showa,
    Heisei
}