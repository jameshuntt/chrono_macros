//! Helpers for timestamps and durations, and ready-made typed timestamps and durations.
//!
//! This module was the `chrono_helpers` crate. Every function it had is still here under its name. Where it
//! had several functions doing one job, one is the canonical name and the others are deprecated in favour of it
//! (they still compile and return the same answers):
//!
//! | job | canonical | deprecated in favour of it |
//! |---|---|---|
//! | is it before now | [`is_past`] | `is_expired_a`, `is_expired_2`, `is_expired_3`, `is_expired_4`, `is_expired_5`, `is_past_2` |
//! | is it after now | [`is_future`] | `is_future_2` |
//! | how long until it | [`time_remaining`] | `time_remaining_2` |
//! | the earlier of it and now | [`clamp_to_now`] | `clamp_to_now_2` |
//! | a timestamp plus a duration | [`extend_by`] (trait bounds), [`add_duration`] (`Deref` bounds) | `add_duration_2` |
//! | a duration in whole seconds | [`into_seconds`] (trait bound), [`seconds_left`] (`Deref` bound) | `seconds_left_simple` |
//!
//! The canonical functions take [`IsUtcDatetime`] / [`IsDuration`]: the types
//! [`typed_datetime!`](crate::typed_datetime!) and [`typed_duration!`](crate::typed_duration!) declare, chrono's
//! own `DateTime<Utc>` and `Duration`, and references to them. `add_duration` and `seconds_left` take anything
//! that derefs to the chrono value.
//!
//! The typed values deref to the chrono value, which is what the `Deref`-bound helpers rely on:
//!
//! ```
//! use std::ops::Deref;
//! use chrono_macros::chrono::{DateTime, Utc};
//!
//! pub struct TokenExpiry(pub DateTime<Utc>);
//!
//! impl Deref for TokenExpiry {
//!     type Target = DateTime<Utc>;
//!     fn deref(&self) -> &Self::Target { &self.0 }
//! }
//!
//! let expiry = TokenExpiry(Utc::now());
//! let dt: &DateTime<Utc> = &expiry; // this works!
//! let is_past = *expiry <= Utc::now(); // this works too!
//! assert!(is_past);
//! # let _ = dt;
//!
//! // the same shape for any inner type
//! struct SomeInnerType;
//! struct MyType(SomeInnerType);
//! impl Deref for MyType {
//!     type Target = SomeInnerType;
//!     fn deref(&self) -> &Self::Target { &self.0 }
//! }
//! let _inner: &SomeInnerType = &MyType(SomeInnerType);
//! ```
//!
//! With the ready-made types:
//!
//! ```
//! use chrono_macros::helpers::{add_duration, is_past, seconds_left, CreatedAt, RetryDelay, SessionTimeout, TokenExpiry};
//!
//! let expired = is_past(TokenExpiry::from_str("2024-12-25T10:00:00Z"));
//! let secs = seconds_left(SessionTimeout::from_secs(90));
//! let t = add_duration(CreatedAt::now(), RetryDelay::from_secs(30));
//! assert!(expired);
//! assert_eq!(secs, 90);
//! assert!(t > *CreatedAt::now());
//! ```

/// Whether `timestamp` is before now.
#[deprecated(since = "0.1.0", note = "use `is_past`")]
pub fn is_expired_a<T>(timestamp: T) -> bool
where
    T: std::ops::Deref<Target = chrono::DateTime<chrono::Utc>>,
{
    *timestamp < chrono::Utc::now()
}
/// Whether `val` is before now.
#[deprecated(since = "0.1.0", note = "use `is_past`")]
pub fn is_expired_2<T: IsUtcDatetime>(val: T) -> bool {
    val.as_utc() < &chrono::Utc::now()
}
/// Whether `val` is before now. (Private in `chrono_helpers`.)
#[deprecated(since = "0.1.0", note = "use `is_past`")]
pub fn is_expired_3<T: Deref<Target = DateTime<Utc>>>(val: T) -> bool {
    *val < Utc::now()
}
/// Whether `t` is before now.
#[deprecated(since = "0.1.0", note = "use `is_past`")]
pub fn is_expired_4<T: Deref<Target = DateTime<Utc>>>(t: T) -> bool {
    *t < Utc::now()
}



/// A value that derefs to a `chrono::Duration`: the bound [`seconds_left`] takes.
///
/// Implemented for every `T: Deref<Target = Duration>` (the [`typed_duration!`](crate::typed_duration!) types,
/// `&Duration`, `Box<Duration>`, ...).
pub trait AsDuration {
    /// The duration.
    fn as_duration(&self) -> &Duration;
}

impl<T> AsDuration for T
where
    T: Deref<Target = Duration>,
{
    fn as_duration(&self) -> &Duration {
        self.deref()
    }
}


#[allow(dead_code)]
type DerefDuration = Box<dyn Deref<Target = chrono::Duration>>;


/// The duration in whole seconds, for anything that derefs to a `chrono::Duration`.
///
/// ```
/// use chrono_macros::helpers::{seconds_left, SessionTimeout};
///
/// assert_eq!(seconds_left(SessionTimeout::from_millis(90_999)), 90);
/// ```
pub fn seconds_left<T>(dur: T) -> i64
where
    T: AsDuration,
{
    dur.as_duration().num_seconds()
}

/// The duration in whole seconds.
#[deprecated(since = "0.1.0", note = "use `seconds_left` (or `into_seconds`)")]
pub fn seconds_left_simple<T>(dur: T) -> i64
where
    T: std::ops::Deref<Target = chrono::Duration>,
{
    dur.num_seconds()
}



// let expired = is_expired(TokenExpiry::now());
// let secs = seconds_left(SessionTimeout::from_secs(90));


pub use crate::traits::{IsDuration, IsUtcDatetime};








/// The earlier of `dt` and now: a timestamp in the future becomes now.
///
/// ```
/// use chrono_macros::{chrono::{Duration, Utc}, helpers::clamp_to_now};
///
/// let past = Utc::now() - Duration::hours(1);
/// assert_eq!(clamp_to_now(past), past);
/// assert!(clamp_to_now(Utc::now() + Duration::hours(1)) <= Utc::now());
/// ```
pub fn clamp_to_now<T: IsUtcDatetime>(dt: T) -> chrono::DateTime<Utc> {
    dt.as_utc().min(&Utc::now()).to_owned()
}

/// `dt` plus `dur`.
///
/// ```
/// use chrono_macros::helpers::{extend_by, ExpiresAt, SessionTimeout};
///
/// let t = extend_by(ExpiresAt::from_str("2024-12-25T10:00:00Z"), SessionTimeout::from_secs(90));
/// assert_eq!(t.to_rfc3339(), "2024-12-25T10:01:30+00:00");
/// ```
pub fn extend_by<T: IsUtcDatetime, D: IsDuration>(dt: T, dur: D) -> DateTime<Utc> {
    *dt.as_utc() + *dur.as_duration()
}






















/// `time` plus `duration`, for anything that derefs to the chrono values.
///
/// ```
/// use chrono_macros::helpers::{add_duration, CreatedAt, RetryDelay};
///
/// let t = add_duration(CreatedAt::from_str("2024-12-25T10:00:00Z"), RetryDelay::from_secs(30));
/// assert_eq!(t.to_rfc3339(), "2024-12-25T10:00:30+00:00");
/// ```
pub fn add_duration<
    T: Deref<Target = DateTime<Utc>>,
    D: Deref<Target = chrono::Duration>,
>(time: T, duration: D) -> DateTime<Utc> {
    *time + *duration
}




// let exp = TokenExpiry::now();
// let is_done = chrono_helpers::is_expired(exp);
//
// let t = add_duration(CreatedAt::now(), RetryDelay::from_secs(30));







typed_datetime!(AuditTimestamp, "audit");

typed_datetime!(CreatedAt, "created_at");
// typed_datetime!(CreatedAt, "created");

typed_datetime!(UpdatedAt, "updated_at");
// typed_datetime!(UpdatedAt, "updated");

typed_datetime!(DeletedAt, "deleted_at");
// typed_datetime!(DeletedAt, "deleted");

typed_datetime!(ExpiresAt, "expires_at");
// typed_datetime!(ExpiresAt, "expires");
// typed_datetime!(ExpiresAt, "expires");

typed_duration!(RetryBackoff, "retry_backoff");

typed_duration!(RetryDelay, "retry_delay");

typed_duration!(SessionTimeout, "session_timeout");
// typed_duration!(SessionTimeout, "timeout");

typed_datetime!(TokenExpiry, "expiry");

typed_datetime!(LastLogin, "last_login");

#[cfg(feature = "uuid")]
uuid_macros::typed_uuid!(UserId, "user");



/// A user record with a typed id and typed timestamps (the `uuid` feature).
///
/// The same record with raw fields, as it was written before the typed versions: every field below is a
/// `DateTime<Utc>`, so nothing stops `created` and `expires_at` from being swapped.
///
/// ```
/// use chrono_macros::chrono::{DateTime, Utc};
/// use uuid_macros::uuid::Uuid;
///
/// #[derive(Debug)]
/// pub struct User {
///     pub id: Uuid,
///     pub created: DateTime<Utc>,
///     pub last_login: DateTime<Utc>,
///     pub expires_at: DateTime<Utc>,
/// }
/// # let _ = User { id: Uuid::nil(), created: Utc::now(), last_login: Utc::now(), expires_at: Utc::now() };
/// ```
///
/// The typed one:
///
/// ```
/// use chrono_macros::helpers::{is_future, CreatedAt, ExpiresAt, LastLogin, User, UserId};
///
/// let user = User {
///     id: UserId::named("alice"),
///     created: CreatedAt::from_str("2024-12-25T10:00:00Z"),
///     last_login: LastLogin::now(),
///     expires_at: ExpiresAt::from_str("2999-01-01T00:00:00Z"),
/// };
/// assert!(is_future(user.expires_at));
/// assert_eq!(user.id, UserId::named("alice"));
/// ```
#[cfg(feature = "uuid")]
#[derive(Debug)]
pub struct User {
    /// The user's id.
    pub id: UserId,
    /// When the user was created.
    pub created: CreatedAt,
    /// The user's last login.
    pub last_login: LastLogin,
    /// When the user expires.
    pub expires_at: ExpiresAt,
}





/// A record that carries its creation and last-update timestamps.
///
/// ```
/// use chrono_macros::helpers::{AuditStamped, CreatedAt, UpdatedAt};
///
/// struct Note { created: CreatedAt, updated: UpdatedAt }
///
/// impl AuditStamped for Note {
///     fn created_at(&self) -> CreatedAt { self.created }
///     fn updated_at(&self) -> UpdatedAt { self.updated }
/// }
///
/// let note = Note { created: CreatedAt::from_str("2024-12-25T10:00:00Z"), updated: UpdatedAt::now() };
/// assert!(*note.created_at() < *note.updated_at());
/// ```
pub trait AuditStamped {
    /// When the record was created.
    fn created_at(&self) -> CreatedAt;
    /// When the record was last updated.
    fn updated_at(&self) -> UpdatedAt;
}



use std::ops::Deref;

pub use crate::chrono;
use crate::chrono::{DateTime, Utc}; // the macros are in scope from `#[macro_use] mod macros`

// #[macro_use]
// pub mod macros; // contains `typed_datetime!`, etc.





/// The ready-made types most code uses.
pub mod prelude {
    pub use super::{
        CreatedAt, UpdatedAt, DeletedAt,
        ExpiresAt, SessionTimeout, RetryDelay,
    };
}





/// Whether `val` is before now. The canonical "has it expired" check.
///
/// ```
/// use chrono_macros::{chrono::{Duration, Utc}, helpers::{is_past, ExpiresAt}};
///
/// assert!(is_past(ExpiresAt::from_str("2024-12-25T10:00:00Z")));
/// assert!(!is_past(Utc::now() + Duration::hours(1)));
/// ```
pub fn is_past<T: IsUtcDatetime>(val: T) -> bool {
    *val.as_utc() < Utc::now()
}

/// Whether `val` is after now.
///
/// ```
/// use chrono_macros::{chrono::{Duration, Utc}, helpers::{is_future, ExpiresAt}};
///
/// assert!(is_future(ExpiresAt::from(Utc::now() + Duration::hours(1))));
/// assert!(!is_future(ExpiresAt::from_str("2024-12-25T10:00:00Z")));
/// ```
pub fn is_future<T: IsUtcDatetime>(val: T) -> bool {
    *val.as_utc() > Utc::now()
}


/// How long until `deadline`; zero once it has passed.
///
/// ```
/// use chrono_macros::{chrono::{Duration, Utc}, helpers::{time_remaining, ExpiresAt}};
///
/// let left = time_remaining(ExpiresAt::from(Utc::now() + Duration::hours(1)));
/// assert!(left > Duration::minutes(59) && left <= Duration::hours(1));
/// assert_eq!(time_remaining(ExpiresAt::from_str("2024-12-25T10:00:00Z")), Duration::zero());
/// ```
pub fn time_remaining<T: IsUtcDatetime>(deadline: T) -> chrono::Duration {
    (*deadline.as_utc() - Utc::now()).max(chrono::Duration::zero())
}


/// The duration in whole seconds.
///
/// ```
/// use chrono_macros::helpers::{into_seconds, RetryBackoff};
///
/// assert_eq!(into_seconds(RetryBackoff::from_secs(45)), 45);
/// ```
pub fn into_seconds<D: IsDuration>(dur: D) -> i64 {
    dur.as_duration().num_seconds()
}

// use std::ops::Deref;
// use chrono::{DateTime, Duration, Utc};
use chrono::Duration;

/// Returns true if the given `DateTime<Utc>` is in the past.
#[deprecated(since = "0.1.0", note = "use `is_past`")]
pub fn is_expired_5<T: Deref<Target = DateTime<Utc>>>(val: T) -> bool {
    *val < Utc::now()
}

/// Returns true if the given `DateTime<Utc>` is in the future.
#[deprecated(since = "0.1.0", note = "use `is_future`")]
pub fn is_future_2<T: Deref<Target = DateTime<Utc>>>(dt: T) -> bool {
    *dt > Utc::now()
}

/// Returns true if the given `DateTime<Utc>` is in the past.
#[deprecated(since = "0.1.0", note = "use `is_past`")]
pub fn is_past_2<T: Deref<Target = DateTime<Utc>>>(dt: T) -> bool {
    *dt < Utc::now()
}

/// Returns the earlier of the given `DateTime<Utc>` or now (i.e., clamps future timestamps to now).
#[deprecated(since = "0.1.0", note = "use `clamp_to_now`")]
pub fn clamp_to_now_2<T: Deref<Target = DateTime<Utc>>>(dt: T) -> DateTime<Utc> {
    (*dt).min(Utc::now())
}

/// Adds a duration to a given `DateTime<Utc>`.
#[deprecated(since = "0.1.0", note = "use `add_duration` (or `extend_by`)")]
pub fn add_duration_2<
    T: Deref<Target = DateTime<Utc>>,
    D: Deref<Target = Duration>,
>(dt: T, dur: D) -> DateTime<Utc> {
    *dt + *dur
}

/// Returns the time remaining until the deadline. Returns zero if already past.
#[deprecated(since = "0.1.0", note = "use `time_remaining`")]
pub fn time_remaining_2<T: Deref<Target = DateTime<Utc>>>(deadline: T) -> Duration {
    (*deadline - Utc::now()).max(Duration::zero())
}
