pub fn is_expired_a<T>(timestamp: T) -> bool
where
    T: std::ops::Deref<Target = chrono::DateTime<chrono::Utc>>,
{
    *timestamp < chrono::Utc::now()
}
pub fn is_expired_2<T: IsUtcDatetime>(val: T) -> bool {
    val.as_utc() < &chrono::Utc::now()
}
fn is_expired_3<T: Deref<Target = DateTime<Utc>>>(val: T) -> bool {
    *val < Utc::now()
}
pub fn is_expired_4<T: Deref<Target = DateTime<Utc>>>(t: T) -> bool {
    *t < Utc::now()
}



trait AsDuration {
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


type DerefDuration = Box<dyn Deref<Target = chrono::Duration>>;


pub fn seconds_left<T>(dur: T) -> i64
where
    T: AsDuration,
{
    dur.as_duration().num_seconds()
}

pub fn seconds_left_simple<T>(dur: T) -> i64
where
    T: std::ops::Deref<Target = chrono::Duration>,
{
    dur.num_seconds()
}



// let expired = is_expired(TokenExpiry::now());
// let secs = seconds_left(SessionTimeout::from_secs(90));


pub trait IsUtcDatetime {
    fn as_utc(&self) -> &chrono::DateTime<chrono::Utc>;
}

pub trait IsDuration {
    fn as_duration(&self) -> &chrono::Duration;
}








pub fn clamp_to_now<T: IsUtcDatetime>(dt: T) -> chrono::DateTime<Utc> {
    dt.as_utc().min(&Utc::now()).to_owned()
}

pub fn extend_by<T: IsUtcDatetime, D: IsDuration>(dt: T, dur: D) -> DateTime<Utc> {
    *dt.as_utc() + *dur.as_duration()
}







struct SomeInnerType;
struct MyType(SomeInnerType);
impl Deref for MyType {
    type Target = SomeInnerType;
    fn deref(&self) -> &Self::Target { &self.0 }
}


// pub struct TokenExpiry(pub chrono::DateTime<Utc>);

// impl Deref for TokenExpiry {
//     type Target = chrono::DateTime<Utc>;
//     fn deref(&self) -> &Self::Target { &self.0 }
// }


// let expiry = TokenExpiry(Utc::now());
// let dt: &DateTime<Utc> = &expiry; // this works!
// let is_past = *expiry < Utc::now(); // this works too!
// 


// impl std::ops::Deref for TokenExpiry {
//     type Target = chrono::DateTime<chrono::Utc>;
//     fn deref(&self) -> &Self::Target { &self.0 }
// }










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

typed_uuid!(UserId, "user");



// #[derive(Debug)]
// pub struct User {
//     pub id: Uuid,
//     pub created: DateTime<Utc>,
//     pub last_login: DateTime<Utc>,
//     pub expires_at: DateTime<Utc>,
// }



#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub created: CreatedAt,
    pub last_login: LastLogin,
    pub expires_at: ExpiresAt,
}





pub trait AuditStamped {
    fn created_at(&self) -> CreatedAt;
    fn updated_at(&self) -> UpdatedAt;
}



use std::ops::Deref;

pub use chrono_macros::chrono;
use chrono_macros::{chrono::{DateTime, Utc}, typed_datetime, typed_duration};
use uuid_macros::typed_uuid;

// #[macro_use]
// pub mod macros; // contains `typed_datetime!`, etc.





pub mod prelude {
    pub use super::{
        CreatedAt, UpdatedAt, DeletedAt,
        ExpiresAt, SessionTimeout, RetryDelay,
    };
}





pub fn is_past<T: IsUtcDatetime>(val: T) -> bool {
    *val.as_utc() < Utc::now()
}

pub fn is_future<T: IsUtcDatetime>(val: T) -> bool {
    *val.as_utc() > Utc::now()
}


pub fn time_remaining<T: IsUtcDatetime>(deadline: T) -> chrono::Duration {
    (*deadline.as_utc() - Utc::now()).max(chrono::Duration::zero())
}


pub fn into_seconds<D: IsDuration>(dur: D) -> i64 {
    dur.as_duration().num_seconds()
}

// use std::ops::Deref;
// use chrono::{DateTime, Duration, Utc};
use chrono::Duration;

/// Returns true if the given `DateTime<Utc>` is in the past.
pub fn is_expired_5<T: Deref<Target = DateTime<Utc>>>(val: T) -> bool {
    *val < Utc::now()
}

/// Returns true if the given `DateTime<Utc>` is in the future.
pub fn is_future_2<T: Deref<Target = DateTime<Utc>>>(dt: T) -> bool {
    *dt > Utc::now()
}

/// Returns true if the given `DateTime<Utc>` is in the past.
pub fn is_past_2<T: Deref<Target = DateTime<Utc>>>(dt: T) -> bool {
    *dt < Utc::now()
}

/// Returns the earlier of the given `DateTime<Utc>` or now (i.e., clamps future timestamps to now).
pub fn clamp_to_now_2<T: Deref<Target = DateTime<Utc>>>(dt: T) -> DateTime<Utc> {
    (*dt).min(Utc::now())
}

/// Adds a duration to a given `DateTime<Utc>`.
pub fn add_duration_2<
    T: Deref<Target = DateTime<Utc>>,
    D: Deref<Target = Duration>,
>(dt: T, dur: D) -> DateTime<Utc> {
    *dt + *dur
}

/// Returns the time remaining until the deadline. Returns zero if already past.
pub fn time_remaining_2<T: Deref<Target = DateTime<Utc>>>(deadline: T) -> Duration {
    (*deadline - Utc::now()).max(Duration::zero())
}
