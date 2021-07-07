#![allow(non_camel_case_types)]
#![allow(clippy::default_trait_access, clippy::doc_markdown)]

pub mod services {
    tonic::include_proto!("proto");
}

pub mod mirror {
    tonic::include_proto!("mirror/com.hedera.mirror.api.proto");
}

pub mod streams {
    tonic::include_proto!("streams/proto");
}
