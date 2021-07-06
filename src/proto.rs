#![allow(clippy::default_trait_access, clippy::doc_markdown)]

tonic::include_proto!("proto");

#[allow(non_camel_case_types)]
pub mod mirror {
    tonic::include_proto!("mirror/com.hedera.mirror.api.proto");
}

#[allow(non_camel_case_types)]
pub mod streams {
    tonic::include_proto!("streams/proto");
}

mod convert {
    use chrono::{DateTime, Duration, TimeZone, Utc};

    impl From<super::Duration> for Duration {
        fn from(pb: super::Duration) -> Self {
            Duration::seconds(pb.seconds)
        }
    }

    impl From<super::TimestampSeconds> for DateTime<Utc> {
        fn from(pb: super::TimestampSeconds) -> Self {
            Utc.timestamp(pb.seconds, 0)
        }
    }

    impl From<super::Timestamp> for DateTime<Utc> {
        #[allow(clippy::cast_sign_loss)]
        fn from(pb: super::Timestamp) -> Self {
            Utc.timestamp(pb.seconds, 0) + Duration::nanoseconds(pb.nanos.into())
        }
    }
}