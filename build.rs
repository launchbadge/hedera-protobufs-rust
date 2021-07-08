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
        .extern_path(".proto.AccountAmount", "crate::services::AccountAmount")
        .extern_path(".proto.AllAccountBalances", "crate::services::AllAccountBalances")
        .extern_path(".proto.CurrentAndNextFeeSchedule", "crate::services::CurrentAndNextFeeSchedule")
        .extern_path(".proto.FeeComponents", "crate::services::FeeComponents")
        .extern_path(".proto.FeeData", "crate::services::FeeData")
        .extern_path(".proto.FeeSchedule", "crate::services::FeeSchedule")
        .extern_path(".proto.Key", "crate::services::Key")
        .extern_path(".proto.FileId", "crate::services::FileId")
        .extern_path(".proto.KeyList", "crate::services::KeyList")
        .extern_path(".proto.NftTransfer", "crate::services::NftTransfer")
        .extern_path(".proto.NodeAddress", "crate::services::NodeAddress")
        .extern_path(".proto.NodeAddressBook", "crate::services::NodeAddressBook")
        .extern_path(".proto.RealmId", "crate::services::RealmId")
        .extern_path(".proto.ScheduleId", "crate::services::ScheduleId")
        .extern_path(".proto.SemanticVersion", "crate::services::SemanticVersion")
        .extern_path(".proto.ServiceEndpoint", "crate::services::ServiceEndpoint")
        .extern_path(".proto.ServicesConfigurationList", "crate::services::ServicesConfigurationList")
        .extern_path(".proto.Setting", "crate::services::Setting")
        .extern_path(".proto.ShardId", "crate::services::ShardId")
        .extern_path(".proto.Signature", "crate::services::Signature")
        .extern_path(".proto.SignatureList", "crate::services::SignatureList")
        .extern_path(".proto.SignatureMap", "crate::services::SignatureMap")
        .extern_path(".proto.SignaturePair", "crate::services::SignaturePair")
        .extern_path(".proto.SingleAccountBalances", "crate::services::SingleAccountBalances")
        .extern_path(".proto.ThresholdKey", "crate::services::ThresholdKey")
        .extern_path(".proto.ThresholdSignature", "crate::services::ThresholdSignature")
        .extern_path(".proto.TimestampSeconds", "crate::services::TimestampSeconds")
        .extern_path(".proto.TokenBalance", "crate::services::TokenBalance")
        .extern_path(".proto.TokenBalances", "crate::services::TokenBalances")
        .extern_path(".proto.TokenRelationship", "crate::services::TokenRelationship")
        .extern_path(".proto.TokenTransferList", "crate::services::TokenTransferList")
        .extern_path(".proto.TokenUnitBalance", "crate::services::TokenUnitBalance")
        .extern_path(".proto.TopicId", "crate::services::TopicId")
        .extern_path(".proto.TransactionFeeSchedule", "crate::services::TransactionFeeSchedule")
        .extern_path(".proto.TransactionId", "crate::services::TransactionId")
        .extern_path(".proto.TransferList", "crate::services::TransferList")
        .extern_path(".proto.HederaFunctionality", "crate::services::HederaFunctionality")
        .extern_path(".proto.SubType", "crate::services::SubType")
        .extern_path(".proto.TokenFreezeStatus", "crate::services::TokenFreezeStatus")
        .extern_path(".proto.TokenKycStatus", "crate::services::TokenKycStatus")
        .extern_path(".proto.TokenSupplyType", "crate::services::TokenSupplyType")
        .extern_path(".proto.TokenType", "crate::services::TokenType")
        .extern_path(".proto.ContractId", "crate::services::ContractId")
        .out_dir(&streams_out_dir)
        .compile(
            &["proto/streams/AccountBalanceFile.proto"],
            &["proto/streams/", "proto/services/"],
        )?;

    Ok(())
}
