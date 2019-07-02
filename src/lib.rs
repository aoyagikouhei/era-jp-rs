//! Japanese Era Library

use chrono::prelude::*;

/// Start Meiji Era
pub fn meiji() -> DateTime<Utc> {
    Utc.ymd(1868, 9, 7).and_hms(15, 0, 0)
}

/// Start Tasho Era
pub fn taisyo() -> DateTime<Utc> {
    Utc.ymd(1912, 7, 29).and_hms(15, 0, 0)
}

/// Start Showa Era
pub fn showa() -> DateTime<Utc> {
    Utc.ymd(1926, 12, 24).and_hms(15, 0, 0)
}

/// Start Heisei Era
pub fn heisei() -> DateTime<Utc> {
    Utc.ymd(1989, 1, 6).and_hms(15, 0, 0)
}

/// Start Reiwa Era
pub fn reiwa() -> DateTime<Utc> {
    Utc.ymd(2019, 4, 30).and_hms(15, 0, 0)
}

/// get era from datetime
pub fn get_era<T: TimeZone>(dt: &DateTime<T>) -> Era {
    let dt = dt.with_timezone(&Utc);
    if dt < meiji() {
        Era::PreMeiji
    } else if dt < taisyo() {
        Era::Meiji
    } else if dt < showa() {
        Era::Taisho
    } else if dt < heisei() {
        Era::Showa
    } else if dt < reiwa() {
        Era::Heisei
    } else {
        Era::Reiwa
    }
}

/// get era year
pub fn get_year<T: TimeZone>(dt: &DateTime<T>) -> i64 {
    let year = dt.year();
    let res = match get_era(dt) {
        Era::PreMeiji => year,
        Era::Meiji => year - meiji().year() + 1,
        Era::Taisho => year - taisyo().year() + 1,
        Era::Showa => year - showa().year() + 1,
        Era::Heisei => year - heisei().year() + 1,
        Era::Reiwa => year - reiwa().year() + 1,
    };
    res as i64
}

/// get full name
pub fn get_name<T: TimeZone>(dt: &DateTime<T>) -> &str {
    match get_era(dt) {
        Era::PreMeiji => "西暦",
        Era::Meiji => "明治",
        Era::Taisho => "大正",
        Era::Showa => "昭和",
        Era::Heisei => "平成",
        Era::Reiwa => "令和",
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
        Era::Reiwa => "令",
    }
}

/// get ligature
pub fn get_ligature<T: TimeZone>(dt: &DateTime<T>) -> &str {
    match get_era(dt) {
        Era::PreMeiji => "西暦",
        Era::Meiji => "\u{337E}",
        Era::Taisho => "\u{337D}",
        Era::Showa => "\u{337C}",
        Era::Heisei => "\u{337B}",
        Era::Reiwa => "\u{32FF}",
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
        Era::Reiwa => "R",
    }
}

/// get year from era
pub fn get_year_from_era(year: i64, era: Era) -> i64 {
    let base = match era {
        Era::PreMeiji => 1,
        Era::Meiji => meiji().year(),
        Era::Taisho => taisyo().year(),
        Era::Showa => showa().year(),
        Era::Heisei => heisei().year(),
        Era::Reiwa => reiwa().year(),
    };
    ((base - 1) as i64) + year
}

#[derive(Debug, Clone, PartialEq)]
pub enum Era {
    PreMeiji,
    Meiji,
    Taisho,
    Showa,
    Heisei,
    Reiwa,
}
