
pub trait IsUtcDatetime {
    fn as_utc(&self) -> &chrono::DateTime<chrono::Utc>;
}
