extern crate chrono;
extern crate era_jp;

use chrono::prelude::*;
use era_jp::Era;

#[test]
fn it_works() {
    let dt = Utc.ymd(1868, 9, 6).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::PreMeiji);
    assert_eq!(era_jp::get_year(&dt), 1868);
    assert_eq!(era_jp::get_name(&dt), "西暦");
    assert_eq!(era_jp::get_short_name(&dt), "西暦");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "西暦");
    let dt = Utc.ymd(1868, 9, 7).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Meiji);
    assert_eq!(era_jp::get_year(&dt), 1);
    assert_eq!(era_jp::get_name(&dt), "明治");
    assert_eq!(era_jp::get_short_name(&dt), "明");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "M");
    let dt = Utc.ymd(1912, 7, 28).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Meiji);
    assert_eq!(era_jp::get_year(&dt), 45);
    assert_eq!(era_jp::get_name(&dt), "明治");
    assert_eq!(era_jp::get_short_name(&dt), "明");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "M");
    let dt = Utc.ymd(1912, 7, 29).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Taisho);
    assert_eq!(era_jp::get_year(&dt), 1);
    assert_eq!(era_jp::get_name(&dt), "大正");
    assert_eq!(era_jp::get_short_name(&dt), "大");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "T");
    let dt = Utc.ymd(1926, 12, 23).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Taisho);
    assert_eq!(era_jp::get_era(&dt), Era::Taisho);
    assert_eq!(era_jp::get_year(&dt), 15);
    assert_eq!(era_jp::get_name(&dt), "大正");
    assert_eq!(era_jp::get_short_name(&dt), "大");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "T");
    let dt = Utc.ymd(1926, 12, 24).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Showa);
    assert_eq!(era_jp::get_year(&dt), 1);
    assert_eq!(era_jp::get_name(&dt), "昭和");
    assert_eq!(era_jp::get_short_name(&dt), "昭");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "S");
    let dt = Utc.ymd(1989, 1, 5).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Showa);
    assert_eq!(era_jp::get_year(&dt), 64);
    assert_eq!(era_jp::get_name(&dt), "昭和");
    assert_eq!(era_jp::get_short_name(&dt), "昭");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "S");
    let dt = Utc.ymd(1989, 1, 6).and_hms(15, 0, 0);
    assert_eq!(era_jp::get_era(&dt), Era::Heisei);
    assert_eq!(era_jp::get_year(&dt), 1);
    assert_eq!(era_jp::get_name(&dt), "平成");
    assert_eq!(era_jp::get_short_name(&dt), "平");
    assert_eq!(era_jp::get_abbreviation_name(&dt), "H");
}