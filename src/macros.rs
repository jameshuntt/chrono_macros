// #![deny(missing_docs)]

//! Macros for creating strongly-typed chrono wrappers (DateTime, Duration).
//!
//! The wrappers deref to the chrono value, so chrono's own methods and comparisons work on them:
//!
//! ```
//! use chrono_macros::{chrono, typed_datetime, typed_duration};
//!
//! typed_datetime!(TokenExpiry, "expiry");
//! typed_duration!(SessionTimeout, "timeout");
//!
//! fn check_expiry(expiry: TokenExpiry) -> bool {
//!     *expiry < chrono::Utc::now()
//! }
//!
//! fn wait_time(timeout: SessionTimeout) -> i64 {
//!     timeout.num_seconds()
//! }
//!
//! assert!(check_expiry(TokenExpiry::from_str("2024-12-25T10:00:00Z")));
//! assert_eq!(wait_time(SessionTimeout::from_secs(90)), 90);
//! ```

pub use chrono;


/// Declares a newtype around `DateTime<Utc>` for one kind of timestamp.
///
/// `typed_datetime!(CreatedAt, "created_at")` declares `pub struct CreatedAt(pub DateTime<Utc>)` with:
///
/// - derives `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash`;
/// - `CreatedAt::KIND`, the label (`"created_at"`);
/// - `now()`, and `from_str(s)`, which parses RFC 3339 (any offset, converted to UTC) and panics with
///   `invalid datetime` on bad text;
/// - `Display` as RFC 3339 (`2024-12-25T10:00:00+00:00`), `FromStr` (`s.parse::<CreatedAt>()`, returning the
///   error instead of panicking), `Deref<Target = DateTime<Utc>>`, `From` both ways, and
///   [`IsUtcDatetime`](crate::IsUtcDatetime);
/// - with this crate's `serde` feature, `Serialize` and `Deserialize`, written the way
///   `#[derive(Serialize, Deserialize)]` writes a newtype struct (chrono writes the timestamp as RFC 3339).
///
/// ```
/// use chrono_macros::{chrono::{TimeZone, Utc}, typed_datetime};
///
/// typed_datetime!(CreatedAt, "created_at");
///
/// let t = CreatedAt::from_str("2024-12-25T12:00:00+02:00");
/// assert_eq!(*t, Utc.with_ymd_and_hms(2024, 12, 25, 10, 0, 0).unwrap());
/// assert_eq!(t.to_string(), "2024-12-25T10:00:00+00:00");
/// assert_eq!(t.to_string().parse::<CreatedAt>().unwrap(), t);
/// assert!(t < CreatedAt::now());
/// assert_eq!(CreatedAt::KIND, "created_at");
/// ```
///
/// Two kinds are two types; one is not accepted where the other is expected:
///
/// ```compile_fail,E0308
/// chrono_macros::typed_datetime!(CreatedAt, "created_at");
/// chrono_macros::typed_datetime!(ExpiresAt, "expires_at");
///
/// fn takes_expiry(_: ExpiresAt) {}
/// takes_expiry(CreatedAt::now());
/// ```
#[macro_export]
macro_rules! typed_datetime {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("A `", $kind, "` timestamp: a `DateTime<Utc>` newtype declared by `chrono_macros::typed_datetime!`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $crate::chrono::DateTime<$crate::chrono::Utc>);

        #[allow(clippy::should_implement_trait)]
        impl $name {
            /// The kind label given to the macro.
            pub const KIND: &'static str = $kind;

            /// The current time.
            pub fn now() -> Self {
                Self($crate::chrono::Utc::now())
            }

            /// Parses RFC 3339 text, converting its offset to UTC.
            ///
            /// Panics with `invalid datetime` on bad text; `s.parse()` returns the error instead.
            pub fn from_str(s: &str) -> Self {
                Self($crate::chrono::DateTime::parse_from_rfc3339(s)
                    .expect("invalid datetime")
                    .with_timezone(&$crate::chrono::Utc))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0.to_rfc3339())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::chrono::ParseError;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                $crate::chrono::DateTime::parse_from_rfc3339(s).map(|t| Self(t.with_timezone(&$crate::chrono::Utc)))
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $crate::chrono::DateTime<$crate::chrono::Utc>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$crate::chrono::DateTime<$crate::chrono::Utc>> for $name {
            fn from(value: $crate::chrono::DateTime<$crate::chrono::Utc>) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $crate::chrono::DateTime<$crate::chrono::Utc> {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $crate::IsUtcDatetime for $name {
            fn as_utc(&self) -> &$crate::chrono::DateTime<$crate::chrono::Utc> {
                &self.0
            }
        }

        $crate::__newtype_serde!($name, $crate::chrono::DateTime<$crate::chrono::Utc>);
    };
}

/// Declares a newtype around `chrono::Duration` for one kind of duration.
///
/// `typed_duration!(RetryDelay, "retry_delay")` declares `pub struct RetryDelay(pub chrono::Duration)` with:
///
/// - derives `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash`;
/// - `RetryDelay::KIND`, the label;
/// - `from_secs(i64)`, `from_millis(i64)`, `zero()` and `is_zero()`;
/// - `Display` as whole seconds with an `s` (`90s`; milliseconds are not shown), `Deref<Target = Duration>`,
///   `From` both ways, and [`IsDuration`](crate::IsDuration);
/// - with this crate's `serde` feature, `Serialize` and `Deserialize`, written the way
///   `#[derive(Serialize, Deserialize)]` writes a newtype struct (chrono writes a duration as
///   `(seconds, nanoseconds)`).
///
/// ```
/// use chrono_macros::typed_duration;
///
/// typed_duration!(RetryDelay, "retry_delay");
///
/// let d = RetryDelay::from_millis(90_500);
/// assert_eq!(d.num_milliseconds(), 90_500);
/// assert_eq!(d.to_string(), "90s");
/// assert_eq!(d, RetryDelay::from(chrono_macros::chrono::Duration::milliseconds(90_500)));
/// assert!(RetryDelay::zero().is_zero());
/// assert_eq!(RetryDelay::KIND, "retry_delay");
/// ```
#[macro_export]
macro_rules! typed_duration {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("A `", $kind, "` duration: a `chrono::Duration` newtype declared by `chrono_macros::typed_duration!`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $crate::chrono::Duration);

        impl $name {
            /// The kind label given to the macro.
            pub const KIND: &'static str = $kind;

            /// A duration of `secs` seconds.
            pub fn from_secs(secs: i64) -> Self {
                Self($crate::chrono::Duration::seconds(secs))
            }

            /// A duration of `ms` milliseconds.
            pub fn from_millis(ms: i64) -> Self {
                Self($crate::chrono::Duration::milliseconds(ms))
            }

            /// The zero duration.
            pub fn zero() -> Self {
                Self($crate::chrono::Duration::zero())
            }

            /// Whether this is the zero duration.
            pub fn is_zero(&self) -> bool {
                self.0 == $crate::chrono::Duration::zero()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}s", self.0.num_seconds())
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $crate::chrono::Duration;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$crate::chrono::Duration> for $name {
            fn from(value: $crate::chrono::Duration) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $crate::chrono::Duration {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $crate::IsDuration for $name {
            fn as_duration(&self) -> &$crate::chrono::Duration {
                &self.0
            }
        }

        $crate::__newtype_serde!($name, $crate::chrono::Duration);
    };
}


#[cfg(test)]
typed_datetime!(TestTime, "test_time");
#[cfg(test)]
typed_duration!(TestDur, "test_dur");

#[test]
fn test_datetime_macro() {
    let now = TestTime::now();
    let from_str = TestTime::from_str("2024-12-25T10:00:00Z");
    assert!(from_str < now);
    assert_eq!(TestTime::KIND, "test_time");
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_duration_macro() {
    let d1 = TestDur::from_secs(30);
    let d2 = TestDur::from_millis(30000);
    assert_eq!(d1.0, d2.0);
    assert!(!d1.is_zero());
    assert_eq!(TestDur::zero().is_zero(), true);
}





/// Implements [`IsUtcDatetime`](crate::IsUtcDatetime) for a tuple newtype whose field `.0` is a `DateTime<Utc>`.
///
/// For a newtype of your own; the types [`typed_datetime!`](crate::typed_datetime!) declares already have it.
///
/// ```
/// use chrono_macros::{chrono::{DateTime, Utc}, helpers, impl_utc_trait};
///
/// struct Stamp(DateTime<Utc>);
/// impl_utc_trait!(Stamp);
///
/// assert!(helpers::is_past(Stamp(DateTime::UNIX_EPOCH)));
/// ```
#[macro_export]
macro_rules! impl_utc_trait {
    ($t:ty) => {
        impl $crate::IsUtcDatetime for $t {
            fn as_utc(&self) -> &$crate::chrono::DateTime<$crate::chrono::Utc> {
                &self.0
            }
        }
    };
}

/// Implements [`IsDuration`](crate::IsDuration) for a tuple newtype whose field `.0` is a `chrono::Duration`.
///
/// For a newtype of your own; the types [`typed_duration!`](crate::typed_duration!) declares already have it.
///
/// ```
/// use chrono_macros::{chrono::Duration, helpers, impl_duration_trait};
///
/// struct Wait(Duration);
/// impl_duration_trait!(Wait);
///
/// assert_eq!(helpers::into_seconds(Wait(Duration::minutes(2))), 120);
/// ```
#[macro_export]
macro_rules! impl_duration_trait {
    ($t:ty) => {
        impl $crate::IsDuration for $t {
            fn as_duration(&self) -> &$crate::chrono::Duration {
                &self.0
            }
        }
    };
}

/// `Serialize`/`Deserialize` for `struct $name(pub $inner)`, as `#[derive]` writes them for a newtype struct
/// (`serialize_newtype_struct` / `deserialize_newtype_struct`). Decided by this crate's `serde` feature, not the
/// calling crate's.
#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! __newtype_serde {
    ($name:ident, $inner:ty) => {
        impl $crate::serde::Serialize for $name {
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_newtype_struct(stringify!($name), &self.0)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Self, D::Error> {
                struct Visitor;
                impl<'de> $crate::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;
                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(concat!("tuple struct ", stringify!($name)))
                    }
                    fn visit_newtype_struct<E: $crate::serde::Deserializer<'de>>(
                        self,
                        inner: E,
                    ) -> ::std::result::Result<$name, E::Error> {
                        <$inner as $crate::serde::Deserialize>::deserialize(inner).map($name)
                    }
                    fn visit_seq<A: $crate::serde::de::SeqAccess<'de>>(
                        self,
                        mut seq: A,
                    ) -> ::std::result::Result<$name, A::Error> {
                        match seq.next_element::<$inner>()? {
                            Some(inner) => Ok($name(inner)),
                            None => Err($crate::serde::de::Error::invalid_length(0, &self)),
                        }
                    }
                }
                deserializer.deserialize_newtype_struct(stringify!($name), Visitor)
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __newtype_serde {
    ($name:ident, $inner:ty) => {};
}
