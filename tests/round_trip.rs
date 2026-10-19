//! The types the macros declare, written out and read back.

use chrono_macros::chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_macros::{impl_duration_trait, impl_utc_trait, typed_datetime, typed_duration, IsDuration, IsUtcDatetime};

typed_datetime!(Stamp, "stamp");
typed_datetime!(Other, "other");
typed_duration!(Wait, "wait");

#[test]
fn typed_datetime_display_fromstr_from() {
    let t = Stamp::from(Utc.with_ymd_and_hms(2024, 12, 25, 10, 0, 0).unwrap() + Duration::nanoseconds(123_456_789));
    let text = t.to_string();
    assert_eq!(text, "2024-12-25T10:00:00.123456789+00:00");
    assert_eq!(text.parse::<Stamp>().unwrap(), t); // FromStr
    assert_eq!(Stamp::from_str(&text), t); // the inherent, panicking parse
    assert_eq!(Stamp::from(DateTime::<Utc>::from(t)), t);
    assert!("yesterday".parse::<Stamp>().is_err());
    let now = Stamp::now();
    assert_eq!(now.to_string().parse::<Stamp>().unwrap(), now);
    assert_eq!(Stamp::KIND, "stamp");
    assert_eq!(Other::from_str("2024-12-25T12:00:00+02:00").0, Utc.with_ymd_and_hms(2024, 12, 25, 10, 0, 0).unwrap());
}

#[test]
#[should_panic(expected = "invalid datetime")]
fn typed_datetime_inherent_from_str_panics_on_bad_text() {
    let _ = Stamp::from_str("yesterday");
}

#[test]
fn typed_duration_display_from_and_constructors() {
    let d = Wait::from_millis(90_500);
    assert_eq!(d.to_string(), "90s");
    assert_eq!(Wait::from(Duration::from(d)), d);
    assert_eq!(Wait::from_secs(90).0, Duration::seconds(90));
    assert!(Wait::zero().is_zero());
    assert!(!d.is_zero());
    assert!(Wait::from_secs(1) < Wait::from_secs(2));
    assert_eq!(Wait::KIND, "wait");
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trips() {
    let t = Stamp::from_str("2024-12-25T10:00:00Z");
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json, "\"2024-12-25T10:00:00Z\"");
    assert_eq!(json, serde_json::to_string(&t.0).unwrap()); // what the derive on a newtype struct writes
    assert_eq!(serde_json::from_str::<Stamp>(&json).unwrap(), t);

    let d = Wait::from_millis(1_500);
    let json = serde_json::to_string(&d).unwrap();
    assert_eq!(json, "[1,500000000]"); // chrono writes (seconds, nanoseconds)
    assert_eq!(serde_json::from_str::<Wait>(&json).unwrap(), d);
}

struct Mine(DateTime<Utc>);
impl_utc_trait!(Mine);
struct MyWait(Duration);
impl_duration_trait!(MyWait);

#[test]
fn impl_traits_for_own_newtypes() {
    let t = Utc.with_ymd_and_hms(2024, 12, 25, 10, 0, 0).unwrap();
    assert_eq!(Mine(t).as_utc(), &t);
    assert_eq!(MyWait(Duration::seconds(4)).as_duration(), &Duration::seconds(4));
    assert_eq!(chrono_macros::helpers::extend_by(Mine(t), MyWait(Duration::seconds(4))), t + Duration::seconds(4));
}
