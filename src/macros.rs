// #![deny(missing_docs)]

//! Macros for creating strongly-typed chrono wrappers (DateTime, Duration).

pub use chrono;


#[macro_export]
macro_rules! typed_datetime {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive($crate::chrono::serde::Serialize, $crate::chrono::serde::Deserialize))]
        pub struct $name(pub $crate::chrono::DateTime<$crate::chrono::Utc>);

        impl $name {
            pub const KIND: &'static str = $kind;

            pub fn now() -> Self {
                Self($crate::chrono::Utc::now())
            }

            pub fn from_str(s: &str) -> Self {
                Self($crate::chrono::DateTime::parse_from_rfc3339(s)
                    .expect("invalid datetime")
                    .with_timezone(&$crate::chrono::Utc))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.to_rfc3339())
            }
        }

        impl std::ops::Deref for $name {
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
    };
}

#[macro_export]
macro_rules! typed_duration {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive($crate::chrono::serde::Serialize, $crate::chrono::serde::Deserialize))]
        pub struct $name(pub $crate::chrono::Duration);

        impl $name {
            pub const KIND: &'static str = $kind;

            pub fn from_secs(secs: i64) -> Self {
                Self($crate::chrono::Duration::seconds(secs))
            }

            pub fn from_millis(ms: i64) -> Self {
                Self($crate::chrono::Duration::milliseconds(ms))
            }

            pub fn zero() -> Self {
                Self($crate::chrono::Duration::zero())
            }

            pub fn is_zero(&self) -> bool {
                self.0 == $crate::chrono::Duration::zero()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}s", self.0.num_seconds())
            }
        }

        impl std::ops::Deref for $name {
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
    };
}


typed_datetime!(TokenExpiry, "expiry");
typed_duration!(SessionTimeout, "timeout");

fn check_expiry(expiry: TokenExpiry) -> bool {
    *expiry < chrono::Utc::now()
}

fn wait_time(timeout: SessionTimeout) -> i64 {
    timeout.num_seconds()
}


typed_datetime!(TestTime, "test_time");
typed_duration!(TestDur, "test_dur");

#[test]
fn test_datetime_macro() {
    let now = TestTime::now();
    let from_str = TestTime::from_str("2024-12-25T10:00:00Z");
    assert!(from_str < now);
    assert_eq!(TestTime::KIND, "test_time");
}

#[test]
fn test_duration_macro() {
    let d1 = TestDur::from_secs(30);
    let d2 = TestDur::from_millis(30000);
    assert_eq!(d1.0, d2.0);
    assert!(!d1.is_zero());
    assert_eq!(TestDur::zero().is_zero(), true);
}





#[macro_export]
macro_rules! impl_utc_trait {
    ($t:ty) => {
        impl $crate::IsUtcDatetime for $t {
            fn as_utc(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_duration_trait {
    ($t:ty) => {
        impl $crate::IsDuration for $t {
            fn as_duration(&self) -> &chrono::Duration {
                &self.0
            }
        }
    };
}