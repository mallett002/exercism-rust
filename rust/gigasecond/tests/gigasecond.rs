use time::{Date, PlainDateTime, Time};

#[test]
fn date_only_specification_of_time() {
    // given
    let start = datetime(2011, 4, 25, 0, 0, 0);

    // when
    let actual = gigasecond::after(start);

    // then
    let expected = datetime(2043, 1, 1, 1, 46, 40);

    assert_eq!(actual, expected);
}

#[test]
fn second_test_for_date_only_specification_of_time() {
    // given
    let start = datetime(1977, 6, 13, 0, 0, 0);

    // when
    let actual = gigasecond::after(start);

    // then
    let expected = datetime(2009, 2, 19, 1, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
fn third_test_for_date_only_specification_of_time() {
    // given
    let start = datetime(1959, 7, 19, 0, 0, 0);

    // when
    let actual = gigasecond::after(start);

    // then
    let expected = datetime(1991, 3, 27, 1, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
fn full_time_specified() {
    // given
    let start = datetime(2015, 1, 24, 22, 0, 0);

    // when
    let actual = gigasecond::after(start);

    // then
    let expected = datetime(2046, 10, 2, 23, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
fn full_time_with_day_roll_over() {
    // given
    let start = datetime(2015, 1, 24, 23, 59, 59);

    // when
    let actual = gigasecond::after(start);

    // then
    let expected = datetime(2046, 10, 3, 1, 46, 39);
    assert_eq!(actual, expected);
}

fn datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> time::PlainDateTime {
    PlainDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
