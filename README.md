# chrono_macros

Macros for strongly typed chrono wrappers: one newtype per kind of timestamp or duration, so an `ExpiresAt` is
never passed where a `CreatedAt` is expected; and the helpers that work on them (is it past, how long is left,
extend a deadline).

```toml
[dependencies]
chrono_macros = "0.1"                                     # features: "serde", "uuid"
```

```rust
use chrono_macros::{helpers, typed_datetime, typed_duration};

typed_datetime!(TokenExpiry, "expiry");
typed_duration!(Grace, "grace");

let expiry = TokenExpiry::from_str("2024-12-25T10:00:00Z");
assert!(helpers::is_past(expiry));
assert_eq!(helpers::extend_by(expiry, Grace::from_secs(60)).to_rfc3339(), "2024-12-25T10:01:00+00:00");
```

## The macros

| macro | what it gives you |
|---|---|
| `typed_datetime!(Name, "kind")` | `pub struct Name(pub DateTime<Utc>)`: `KIND`, `now`, `from_str` (RFC 3339; panics on bad text), `Display` as RFC 3339, `FromStr`, `Deref`, `From` both ways, `IsUtcDatetime`; serde with the `serde` feature |
| `typed_duration!(Name, "kind")` | `pub struct Name(pub chrono::Duration)`: `KIND`, `from_secs`, `from_millis`, `zero`, `is_zero`, `Display` as whole seconds (`90s`), `Deref`, `From` both ways, `IsDuration`; serde with the `serde` feature |
| `impl_utc_trait!(T)` | `IsUtcDatetime` for your own newtype around a `DateTime<Utc>` |
| `impl_duration_trait!(T)` | `IsDuration` for your own newtype around a `chrono::Duration` |

## The helpers

`chrono_macros::helpers` was the `chrono_helpers` crate; every function it had is there under its name.

| job | canonical | still there, deprecated in favour of it |
|---|---|---|
| is it before now | `is_past` | `is_expired_a`, `is_expired_2`, `is_expired_3`, `is_expired_4`, `is_expired_5`, `is_past_2` |
| is it after now | `is_future` | `is_future_2` |
| how long until it (zero once passed) | `time_remaining` | `time_remaining_2` |
| the earlier of it and now | `clamp_to_now` | `clamp_to_now_2` |
| a timestamp plus a duration | `extend_by` (`IsUtcDatetime` + `IsDuration`), `add_duration` (`Deref`) | `add_duration_2` |
| a duration in whole seconds | `into_seconds` (`IsDuration`), `seconds_left` (`Deref`) | `seconds_left_simple` |

The canonical functions take the macro types, chrono's own `DateTime<Utc>` and `Duration`, and references to
them. The module also has ready-made types (`CreatedAt`, `UpdatedAt`, `DeletedAt`, `ExpiresAt`, `TokenExpiry`,
`LastLogin`, `AuditTimestamp`, `SessionTimeout`, `RetryDelay`, `RetryBackoff`), a `prelude`, the `AuditStamped`
trait, and, with the `uuid` feature, `UserId` (a [uuid_macros](https://crates.io/crates/uuid_macros) id) and a
`User` record.

The macros reach chrono through `chrono_macros::chrono`, so your crate does not need its own `chrono` dependency.
The `serde` feature is this crate's: turning it on here is what gives the macro types their serde impls.

## Releasing

Bump `version` in Cargo.toml and give CHANGELOG.md a dated `## <version> (YYYY-MM-DD)` entry, then push to main.
CI (`.github/workflows/ci.yml`) tests on stable and Rust 1.85, runs clippy, and publishes to crates.io
through trusted publishing; a version already on crates.io, or an entry still marked `(unreleased)`, publishes nothing.

## Ownership

Copyright © 2026 1o. All rights reserved.

Developed and published by James Hunt. The libraries are published individually, through his crates.io account
(`jameshuntt`) and his GitHub account; he intends the rights in them to go to the `1o` organization
([github.com/1o-llc](https://github.com/1o-llc)). The assignments are pending documentation. The license texts
(LICENSE-MIT, LICENSE-APACHE) are left as they are in his other crates.

## License

MIT OR Apache-2.0
