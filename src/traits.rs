//! The two traits the helpers take: a value that is a UTC timestamp, and a value that is a duration.
//!
//! [`typed_datetime!`](crate::typed_datetime!) and [`typed_duration!`](crate::typed_duration!) implement them
//! for the types they declare; [`impl_utc_trait!`](crate::impl_utc_trait!) and
//! [`impl_duration_trait!`](crate::impl_duration_trait!) implement them for a newtype of your own. chrono's
//! own `DateTime<Utc>` and `Duration`, and a reference to any implementor, implement them too.

/// A value that is a `DateTime<Utc>`.
pub trait IsUtcDatetime {
    /// The timestamp.
    fn as_utc(&self) -> &chrono::DateTime<chrono::Utc>;
}

/// A value that is a `chrono::Duration`.
pub trait IsDuration {
    /// The duration.
    fn as_duration(&self) -> &chrono::Duration;
}

impl IsUtcDatetime for chrono::DateTime<chrono::Utc> {
    fn as_utc(&self) -> &chrono::DateTime<chrono::Utc> {
        self
    }
}

impl<T: IsUtcDatetime + ?Sized> IsUtcDatetime for &T {
    fn as_utc(&self) -> &chrono::DateTime<chrono::Utc> {
        (**self).as_utc()
    }
}

impl IsDuration for chrono::Duration {
    fn as_duration(&self) -> &chrono::Duration {
        self
    }
}

impl<T: IsDuration + ?Sized> IsDuration for &T {
    fn as_duration(&self) -> &chrono::Duration {
        (**self).as_duration()
    }
}
