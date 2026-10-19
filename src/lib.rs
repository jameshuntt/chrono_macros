//! Macros for strongly typed chrono wrappers, and helpers that work on them.
//!
//! Each kind of timestamp or duration gets its own newtype, so an `ExpiresAt` is never passed where a
//! `CreatedAt` is expected:
//!
//! - [`typed_datetime!`] declares a newtype around `DateTime<Utc>` (`KIND`, `now`, `from_str`, `Display` as
//!   RFC 3339, `FromStr`, `Deref`, `From` both ways, [`IsUtcDatetime`]).
//! - [`typed_duration!`] declares a newtype around `chrono::Duration` (`KIND`, `from_secs`, `from_millis`,
//!   `zero`, `is_zero`, `Display` as whole seconds, `Deref`, `From` both ways, [`IsDuration`]).
//! - [`impl_utc_trait!`] and [`impl_duration_trait!`] give a newtype of your own the two traits.
//!
//! With the `serde` feature both macros' types get `Serialize` and `Deserialize`.
//!
//! [`helpers`] holds the functions (is it past, how long is left, extend a deadline, ...) and a set of
//! ready-made types (`CreatedAt`, `ExpiresAt`, `SessionTimeout`, ...). It was the `chrono_helpers` crate.
//!
//! ```
//! use chrono_macros::{helpers, typed_datetime, typed_duration};
//!
//! typed_datetime!(TokenExpiry, "expiry");
//! typed_duration!(Grace, "grace");
//!
//! let expiry = TokenExpiry::from_str("2024-12-25T10:00:00Z");
//! assert!(helpers::is_past(expiry));
//! assert_eq!(expiry.to_string(), "2024-12-25T10:00:00+00:00");
//! assert_eq!(helpers::extend_by(expiry, Grace::from_secs(60)).to_rfc3339(), "2024-12-25T10:01:00+00:00");
//! ```
//!
//! The macros reach chrono through this crate (`chrono_macros::chrono`), so a crate using them does not need
//! its own `chrono` dependency.
#![warn(missing_docs)]

#[macro_use]
pub mod macros;
pub use chrono;

pub mod traits;
pub use traits::*;

pub mod helpers;

/// The serde this crate's `serde` feature builds against; the generated impls name it through here.
#[cfg(feature = "serde")]
#[doc(hidden)]
pub use serde;
