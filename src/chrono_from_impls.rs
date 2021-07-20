use chrono::{DateTime, Duration, TimeZone, Utc};

impl From<super::services::Duration> for Duration {
    fn from(pb: super::services::Duration) -> Self {
        Duration::seconds(pb.seconds)
    }
}

impl From<super::services::TimestampSeconds> for DateTime<Utc> {
    fn from(pb: super::services::TimestampSeconds) -> Self {
        Utc.timestamp(pb.seconds, 0)
    }
}

impl From<super::services::Timestamp> for DateTime<Utc> {
    #[allow(clippy::cast_sign_loss)]
    fn from(pb: super::services::Timestamp) -> Self {
        Utc.timestamp(pb.seconds, 0) + Duration::nanoseconds(pb.nanos.into())
    }
}
