use chrono::prelude::*;

const DATE_FORMAT: &str = "%Y-%m-%d";

fn weeks_between(a: &str, b: &str) -> i32 {
    let t1 = NaiveDate::parse_from_str(a, DATE_FORMAT).unwrap();
    let t2 = NaiveDate::parse_from_str(b, DATE_FORMAT).unwrap();
    (t2 - t1).num_days() as i32 / 7
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
