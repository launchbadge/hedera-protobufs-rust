use std::env;
use std::fs::{create_dir_all, read_dir};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // collect every proto file in services/_

    let services_protos: Vec<_> = read_dir("./proto/services")?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    // services

    let mut config = tonic_build::configure()
        .build_server(true)
        .type_attribute("proto.Key", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Key.key", "#[derive(Eq, Hash)]")
        .type_attribute("proto.KeyList", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ThresholdKey", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TransactionID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.AccountID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Timestamp", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractID", "#[derive(Eq, Hash)]");

    if cfg!(feature = "serde") {
        config = config
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .type_attribute(
                "proto.ResponseCodeEnum",
                "#[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]",
            );
    }

    config.compile(&services_protos, &[Path::new("proto/services/").to_owned()])?;

    // mirror
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let mirror_out_dir = Path::new(&env::var("OUT_DIR")?).join("mirror");
    create_dir_all(&mirror_out_dir)?;

    tonic_build::configure()
        .build_server(true)
        .extern_path(".proto.Timestamp", "crate::services::Timestamp")
        .extern_path(".proto.TopicID", "crate::services::TopicId")
        .extern_path(
            ".proto.ConsensusMessageChunkInfo",
            "crate::services::ConsensusMessageChunkInfo",
        )
        .out_dir(&mirror_out_dir)
        .compile(
            &["proto/mirror/ConsensusService.proto"],
            &["proto/mirror/", "proto/services/"],
        )?;

    // streams
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    // FIXME: the streams module is building way too much, its generating code for a large chunk of types
    //        from the services module

    let streams_out_dir = Path::new(&env::var("OUT_DIR")?).join("streams");
    create_dir_all(&streams_out_dir)?;

    tonic_build::configure()
        .extern_path(".proto.Timestamp", "crate::services::Timestamp")
        .extern_path(".proto.AccountID", "crate::services::AccountId")
        .extern_path(".proto.TokenID", "crate::services::TokenId")
        .out_dir(&streams_out_dir)
        .compile(
            &["proto/streams/AccountBalanceFile.proto"],
            &["proto/streams/", "proto/services/"],
        )?;

    Ok(())
}
