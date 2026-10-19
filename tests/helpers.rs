//! Every helper in `chrono_macros::helpers`, the canonical ones and the deprecated ones, on a timestamp an hour
//! in the past and one an hour in the future.
#![allow(deprecated)]

use chrono_macros::chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_macros::helpers::*;
use chrono_macros::{IsDuration, IsUtcDatetime};

fn hour_ago() -> DateTime<Utc> {
    Utc::now() - Duration::hours(1)
}
fn hour_ahead() -> DateTime<Utc> {
    Utc::now() + Duration::hours(1)
}
fn fixed() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2024, 12, 25, 10, 0, 0).unwrap()
}

#[test]
fn is_past_canonical() {
    assert!(is_past(ExpiresAt::from(hour_ago())));
    assert!(!is_past(ExpiresAt::from(hour_ahead())));
    assert!(is_past(hour_ago())); // chrono's own DateTime<Utc>
    let t = hour_ago();
    #[allow(clippy::needless_borrows_for_generic_args)]
    let by_ref = is_past(&t); // and a reference
    assert!(by_ref);
}

#[test]
fn is_expired_family_agrees_with_is_past() {
    for (t, past) in [(hour_ago(), true), (hour_ahead(), false)] {
        let e = TokenExpiry::from(t);
        assert_eq!(is_past(e), past);
        assert_eq!(is_expired_a(e), past);
        assert_eq!(is_expired_2(e), past);
        assert_eq!(is_expired_3(e), past);
        assert_eq!(is_expired_4(e), past);
        assert_eq!(is_expired_5(e), past);
        assert_eq!(is_past_2(e), past);
        assert_eq!(is_expired_a(&t), past); // the Deref-bound ones take &DateTime<Utc>
    }
}

#[test]
fn is_future_and_is_future_2() {
    assert!(is_future(ExpiresAt::from(hour_ahead())));
    assert!(!is_future(ExpiresAt::from(hour_ago())));
    assert!(is_future_2(ExpiresAt::from(hour_ahead())));
    assert!(!is_future_2(ExpiresAt::from(hour_ago())));
}

#[test]
fn time_remaining_and_time_remaining_2() {
    let left = time_remaining(ExpiresAt::from(hour_ahead()));
    assert!(left > Duration::minutes(59) && left <= Duration::hours(1));
    assert_eq!(time_remaining(ExpiresAt::from(hour_ago())), Duration::zero());
    let left2 = time_remaining_2(ExpiresAt::from(hour_ahead()));
    assert!(left2 > Duration::minutes(59) && left2 <= Duration::hours(1));
    assert_eq!(time_remaining_2(ExpiresAt::from(hour_ago())), Duration::zero());
}

#[test]
fn clamp_to_now_and_clamp_to_now_2() {
    let past = hour_ago();
    assert_eq!(clamp_to_now(CreatedAt::from(past)), past);
    assert_eq!(clamp_to_now_2(CreatedAt::from(past)), past);
    let before = Utc::now();
    let c = clamp_to_now(CreatedAt::from(hour_ahead()));
    let c2 = clamp_to_now_2(CreatedAt::from(hour_ahead()));
    let after = Utc::now();
    assert!(before <= c && c <= after);
    assert!(before <= c2 && c2 <= after);
}

#[test]
fn extend_by_add_duration_add_duration_2() {
    let want = Utc.with_ymd_and_hms(2024, 12, 25, 10, 0, 30).unwrap();
    assert_eq!(extend_by(CreatedAt::from(fixed()), RetryDelay::from_secs(30)), want);
    assert_eq!(extend_by(fixed(), Duration::seconds(30)), want); // chrono's own values
    assert_eq!(add_duration(CreatedAt::from(fixed()), RetryDelay::from_secs(30)), want);
    assert_eq!(add_duration(&fixed(), &Duration::seconds(30)), want);
    assert_eq!(add_duration_2(CreatedAt::from(fixed()), RetryDelay::from_secs(30)), want);
    assert_eq!(extend_by(fixed(), Duration::seconds(-30)), fixed() - Duration::seconds(30));
}

#[test]
fn into_seconds_seconds_left_seconds_left_simple() {
    let d = SessionTimeout::from_millis(90_999);
    assert_eq!(into_seconds(d), 90);
    assert_eq!(seconds_left(d), 90);
    assert_eq!(seconds_left_simple(d), 90);
    assert_eq!(into_seconds(Duration::seconds(-5)), -5);
    assert_eq!(seconds_left(&Duration::seconds(7)), 7);
    assert_eq!(seconds_left(Box::new(Duration::seconds(8))), 8); // AsDuration: anything that derefs
}

#[test]
fn traits_on_typed_and_chrono_values() {
    let t = fixed();
    assert_eq!(CreatedAt::from(t).as_utc(), &t);
    assert_eq!(IsUtcDatetime::as_utc(&t), &t);
    assert_eq!(IsUtcDatetime::as_utc(&&t), &t);
    let d = Duration::seconds(3);
    assert_eq!(IsDuration::as_duration(&RetryBackoff::from(d)), &d);
    assert_eq!(IsDuration::as_duration(&d), &d);
    assert_eq!(AsDuration::as_duration(&RetryBackoff::from(d)), &d);
}

#[test]
fn ready_made_types_and_their_kinds() {
    assert_eq!(AuditTimestamp::KIND, "audit");
    assert_eq!(CreatedAt::KIND, "created_at");
    assert_eq!(UpdatedAt::KIND, "updated_at");
    assert_eq!(DeletedAt::KIND, "deleted_at");
    assert_eq!(ExpiresAt::KIND, "expires_at");
    assert_eq!(TokenExpiry::KIND, "expiry");
    assert_eq!(LastLogin::KIND, "last_login");
    assert_eq!(RetryBackoff::KIND, "retry_backoff");
    assert_eq!(RetryDelay::KIND, "retry_delay");
    assert_eq!(SessionTimeout::KIND, "session_timeout");
    use chrono_macros::helpers::prelude as p;
    let _: (p::CreatedAt, p::UpdatedAt, p::DeletedAt, p::ExpiresAt) =
        (CreatedAt::now(), UpdatedAt::now(), DeletedAt::now(), ExpiresAt::now());
    let _: (p::SessionTimeout, p::RetryDelay) = (SessionTimeout::zero(), RetryDelay::zero());
    // helpers::chrono is chrono
    let _: chrono_macros::helpers::chrono::DateTime<Utc> = fixed();
}

#[test]
fn audit_stamped() {
    struct Note {
        created: CreatedAt,
        updated: UpdatedAt,
    }
    impl AuditStamped for Note {
        fn created_at(&self) -> CreatedAt {
            self.created
        }
        fn updated_at(&self) -> UpdatedAt {
            self.updated
        }
    }
    let n = Note { created: CreatedAt::from(fixed()), updated: UpdatedAt::from(fixed() + Duration::days(1)) };
    assert_eq!(*n.created_at(), fixed());
    assert_eq!(*n.updated_at() - *n.created_at(), Duration::days(1));
}

#[cfg(feature = "uuid")]
#[test]
fn user_with_typed_fields() {
    let u = User {
        id: UserId::named("alice"),
        created: CreatedAt::from(fixed()),
        last_login: LastLogin::now(),
        expires_at: ExpiresAt::from(hour_ahead()),
    };
    assert_eq!(UserId::KIND, "user");
    assert_eq!(u.id, UserId::named("alice"));
    assert!(is_future(u.expires_at));
    assert!(format!("{u:?}").starts_with("User {"));
}
