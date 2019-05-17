use eosio::*;

#[test]
fn test_constructors() {
    assert_eq!(Time::UNIX_EPOCH.as_micros(), 0);
    assert_eq!(Time::from_micros(1).as_micros(), 1);
    assert_eq!(Time::from_millis(1).as_micros(), 1_000);
    assert_eq!(Time::from_secs(1).as_micros(), 1_000_000);
    assert_eq!(Time::from_mins(1).as_micros(), 60_000_000);
    assert_eq!(Time::from_hours(1).as_micros(), 3_600_000_000);
    assert_eq!(Time::from_days(1).as_micros(), 86_400_000_000);
}

#[test]
fn test_converters() {
    assert_eq!(Time::from_millis(1).as_micros(), 1_000);
    assert_eq!(Time::from_secs(1).as_millis(), 1_000);
    assert_eq!(Time::from_mins(1).as_secs(), 60);
    assert_eq!(Time::from_hours(1).as_mins(), 60);
    assert_eq!(Time::from_days(1).as_hours(), 24);
}

#[test]
fn test_min_max() {
    let t1 = Time::from_secs(1);
    let t2 = Time::from_secs(2);
    let t3 = Time::from_secs(3);
    assert_eq!(t1.max(t2), t2);
    assert_eq!(t1.min(t2), t1);
    assert_eq!(t3.max(t2), t3);
    assert_eq!(t3.min(t2), t2);
}
