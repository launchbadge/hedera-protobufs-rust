use std::env;
use std::fs::create_dir_all;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .type_attribute("proto.Key", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Key.key", "#[derive(Eq, Hash)]")
        .type_attribute("proto.KeyList", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ThresholdKey", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TransactionID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.AccountID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Timestamp", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ResponseCodeEnum", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]")
        .compile(
            &[
                "proto/TransactionRecord.proto",
                "proto/Transaction.proto",
                "proto/TransactionBody.proto",
                "proto/TransactionContents.proto",
                "proto/CryptoGetAccountBalance.proto",
            ],
            &["proto/"],
        )?;

    let mirror_out_dir = Path::new(&env::var("OUT_DIR")?).join("mirror");
    create_dir_all(&mirror_out_dir)?;

    tonic_build::configure()
        .build_server(true)
        .extern_path(".proto.Timestamp", "crate::proto::Timestamp")
        .extern_path(".proto.TopicID", "crate::proto::TopicId")
        .extern_path(".proto.ConsensusMessageChunkInfo", "crate::proto::ConsensusMessageChunkInfo")
        .out_dir(&mirror_out_dir)
        .compile(&["proto/mirror/ConsensusService.proto"], &["proto/mirror/", "proto/"])?;

    let streams_out_dir = Path::new(&env::var("OUT_DIR")?).join("streams");
    create_dir_all(&streams_out_dir)?;

    tonic_build::configure()
        .extern_path(".proto.Timestamp", "crate::proto::Timestamp")
        .extern_path(".proto.AccountID", "crate::proto::AccountId")
        .extern_path(".proto.TokenID", "crate::proto::TokenId")
        .out_dir(&streams_out_dir)
        .compile(&["proto/streams/AccountBalanceFile.proto"], &["proto/streams/", "proto/"])?;

    Ok(())
}