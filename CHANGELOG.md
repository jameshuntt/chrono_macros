# Changelog

## 0.1.0 (2026-10-19)

First release: `chrono_macros` and `chrono_helpers` from qweres (`crates/utils/`, the first commit of this
repository) as one crate. The helpers are the module `chrono_macros::helpers`; every macro, function, trait and
type is kept under its name.

- One `IsUtcDatetime` and one `IsDuration` (src/traits.rs). The source had two `IsUtcDatetime` traits, one in each
  crate: `typed_datetime!` implemented chrono_macros' and every helper took chrono_helpers', so no helper accepted
  a `typed_datetime!` value through the trait. `IsDuration` lived only in chrono_helpers, so `impl_duration_trait!`
  (which names `$crate::IsDuration`) did not compile when used. `helpers::IsUtcDatetime` and `helpers::IsDuration`
  are now re-exports of the one pair.
- `typed_duration!` implements `IsDuration`, as `typed_datetime!` implements `IsUtcDatetime`. Both traits are
  implemented for chrono's `DateTime<Utc>` and `Duration` and for references to an implementor.
- `impl_utc_trait!` and `impl_duration_trait!` name chrono through `$crate::chrono`; the source wrote `chrono::...`,
  which needs `chrono` in the caller's scope.
- serde: the source wrote `#[cfg_attr(feature = "serde", derive($crate::chrono::serde::Serialize, ...))]`, which
  the *caller's* feature decided and which named a derive chrono does not have. Now this crate's `serde` feature
  decides it (it turns on `chrono/serde`), and the impls (`__newtype_serde!` in src/macros.rs) write what the
  derive writes for a newtype struct: RFC 3339 text for a timestamp, `[seconds, nanoseconds]` for a duration
  (tests/round_trip.rs, `serde_round_trips`).
- Added `FromStr` (error: `chrono::ParseError`) to `typed_datetime!`; its inherent `from_str`, which panics, is
  kept, and `Name::from_str(s)` still calls it.
- Duplicate helpers: the canonical name per job is `is_past`, `is_future`, `time_remaining`, `clamp_to_now`,
  `extend_by` / `add_duration`, `into_seconds` / `seconds_left`; `is_expired_a`, `is_expired_2`, `is_expired_3`
  (private in the source, now public so it is callable), `is_expired_4`, `is_expired_5`, `is_past_2`,
  `is_future_2`, `time_remaining_2`, `clamp_to_now_2`, `add_duration_2` and `seconds_left_simple` are
  `#[deprecated]` in favour of them and return the same answers (tests/helpers.rs checks each against its
  canonical one). `AsDuration`, the bound of `seconds_left`, was private and is public.
- `UserId` and `User` need uuid_macros and are behind the `uuid` feature.
- The commented-out examples became doctests: the `TokenExpiry` `Deref` example and the calls
  `is_expired(...)` / `seconds_left(...)` / `add_duration(...)` (module docs of `helpers`; `is_expired` was never
  defined, the doctest calls `is_past`), the raw-field `User` (on `User`), and `check_expiry` / `wait_time` (module
  docs of `macros`). The types they declared in the crate itself (`macros::TokenExpiry`, `macros::SessionTimeout`)
  went into the doctest with them; `helpers::TokenExpiry` and `helpers::SessionTimeout` are the crate's.
  `TestTime` and `TestDur` are under `#[cfg(test)]`.
- Edition 2024, rust-version 1.85 and chrono 0.4.41 (the test suite passes on Rust 1.85.1, and on chrono 0.4.41
  and 0.4.45).
- Tests: the source's 2 unit tests (one gained `#[allow(clippy::bool_assert_comparison)]`), 11 helper tests
  (every helper, canonical and deprecated), 5 round-trip tests, and 20 doctests (one of them `compile_fail`: two
  `typed_datetime!` kinds do not unify).
