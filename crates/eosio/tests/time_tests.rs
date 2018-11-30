use eosio::*;

#[test]
fn test_constructors() {
    assert_eq!(Time::zero().microseconds(), 0);
    assert_eq!(Time::from_microseconds(1).microseconds(), 1);
    assert_eq!(Time::from_milliseconds(1).microseconds(), 1_000);
    assert_eq!(Time::from_seconds(1).microseconds(), 1_000_000);
    assert_eq!(Time::from_minutes(1).microseconds(), 60_000_000);
    assert_eq!(Time::from_hours(1).microseconds(), 3_600_000_000);
    assert_eq!(Time::from_days(1).microseconds(), 86_400_000_000);
}

#[test]
fn test_converters() {
    assert_eq!(Time::from_milliseconds(1).microseconds(), 1_000);
    assert_eq!(Time::from_seconds(1).milliseconds(), 1_000);
    assert_eq!(Time::from_minutes(1).seconds(), 60);
    assert_eq!(Time::from_hours(1).minutes(), 60);
    assert_eq!(Time::from_days(1).hours(), 24);
}

#[test]
fn test_min_max() {
    let t1 = Time::from_seconds(1);
    let t2 = Time::from_seconds(2);
    let t3 = Time::from_seconds(3);
    assert_eq!(t1.max(t2), t2);
    assert_eq!(t1.min(t2), t1);
    assert_eq!(t3.max(t2), t3);
    assert_eq!(t3.min(t2), t2);
}
