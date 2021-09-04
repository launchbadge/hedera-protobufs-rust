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
        .type_attribute("proto.AccountAmount", "#[derive(Eq, Hash)]")
        .type_attribute("proto.AccountID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ConsensusCreateTopicTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ConsensusDeleteTopicTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ConsensusMessageChunkInfo", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ConsensusSubmitMessageTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ConsensusUpdateTopicTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractCallTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractCreateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractDeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractDeleteTransactionBody.obtainers", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractUpdateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ContractUpdateTransactionBody.memoField", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoCreateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoDeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoTransferTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoUpdateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoUpdateTransactionBody.receiveRecordThresholdField", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoUpdateTransactionBody.receiverSigRequiredField", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CryptoUpdateTransactionBody.sendRecordThresholdField", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CustomFee", "#[derive(Eq, Hash)]")
        .type_attribute("proto.CustomFee.fee", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Duration", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FileAppendTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FileCreateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FileDeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FileID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FileUpdateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FixedFee", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Fraction", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FractionalFee", "#[derive(Eq, Hash)]")
        .type_attribute("proto.FreezeTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Key", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Key.key", "#[derive(Eq, Hash)]")
        .type_attribute("proto.KeyList", "#[derive(Eq, Hash)]")
        .type_attribute("proto.NftTransfer", "#[derive(Eq, Hash)]")
        .type_attribute("proto.RealmID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.RoyaltyFee", "#[derive(Eq, Hash)]")
        .type_attribute("proto.SchedulableTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.SchedulableTransactionBody.data", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ScheduleDeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ScheduleID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ShardID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.SystemDeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.SystemDeleteTransactionBody.id", "#[derive(Eq, Hash)]")
        .type_attribute("proto.SystemUndeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.SystemUndeleteTransactionBody.id", "#[derive(Eq, Hash)]")
        .type_attribute("proto.ThresholdKey", "#[derive(Eq, Hash)]")
        .type_attribute("proto.Timestamp", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TimestampSeconds", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenAssociateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenBurnTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenCreateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenDeleteTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenDissociateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenFreezeAccountTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenGrantKycTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenMintTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenRevokeKycTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenTransferList", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenUnfreezeAccountTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenUpdateTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TokenWipeAccountTransactionBody", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TopicID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TransactionID", "#[derive(Eq, Hash)]")
        .type_attribute("proto.TransferList", "#[derive(Eq, Hash)]");

    if cfg!(feature = "serde") {
        //TODO: Ideally we'd apply to everything, but that didn't work, so new types are added here on an as-needed basis
        config = config
            .type_attribute(
                "proto.ResponseCodeEnum",
                "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]",
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
            &["proto/mirror/consensus_service.proto"],
            &["proto/mirror/", "proto/services/"],
        )?;

    // streams
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let streams_out_dir = Path::new(&env::var("OUT_DIR")?).join("streams");
    create_dir_all(&streams_out_dir)?;

    tonic_build::configure()
        .extern_path(".proto.Fraction", "crate::services::Fraction")
        .extern_path(".proto.Timestamp", "crate::services::Timestamp")
        .extern_path(".proto.AccountID", "crate::services::AccountId")
        .extern_path(".proto.TokenID", "crate::services::TokenId")
        .extern_path(".proto.AccountAmount", "crate::services::AccountAmount")
        .extern_path(
            ".proto.CurrentAndNextFeeSchedule",
            "crate::services::CurrentAndNextFeeSchedule",
        )
        .extern_path(".proto.FeeComponents", "crate::services::FeeComponents")
        .extern_path(".proto.FeeData", "crate::services::FeeData")
        .extern_path(".proto.FeeSchedule", "crate::services::FeeSchedule")
        .extern_path(".proto.Key", "crate::services::Key")
        .extern_path(".proto.FileID", "crate::services::FileId")
        .extern_path(".proto.KeyList", "crate::services::KeyList")
        .extern_path(".proto.NftTransfer", "crate::services::NftTransfer")
        .extern_path(".proto.NodeAddress", "crate::services::NodeAddress")
        .extern_path(".proto.NodeAddressBook", "crate::services::NodeAddressBook")
        .extern_path(".proto.RealmID", "crate::services::RealmId")
        .extern_path(".proto.ScheduleID", "crate::services::ScheduleId")
        .extern_path(".proto.SemanticVersion", "crate::services::SemanticVersion")
        .extern_path(".proto.ServiceEndpoint", "crate::services::ServiceEndpoint")
        .extern_path(
            ".proto.ServicesConfigurationList",
            "crate::services::ServicesConfigurationList",
        )
        .extern_path(".proto.Setting", "crate::services::Setting")
        .extern_path(".proto.ShardID", "crate::services::ShardId")
        .extern_path(".proto.Signature", "crate::services::Signature")
        .extern_path(".proto.SignatureList", "crate::services::SignatureList")
        .extern_path(".proto.SignatureMap", "crate::services::SignatureMap")
        .extern_path(".proto.SignaturePair", "crate::services::SignaturePair")
        .extern_path(".proto.ThresholdKey", "crate::services::ThresholdKey")
        .extern_path(
            ".proto.ThresholdSignature",
            "crate::services::ThresholdSignature",
        )
        .extern_path(
            ".proto.TimestampSeconds",
            "crate::services::TimestampSeconds",
        )
        .extern_path(".proto.TokenBalance", "crate::services::TokenBalance")
        .extern_path(".proto.TokenBalances", "crate::services::TokenBalances")
        .extern_path(
            ".proto.TokenRelationship",
            "crate::services::TokenRelationship",
        )
        .extern_path(
            ".proto.TokenTransferList",
            "crate::services::TokenTransferList",
        )
        .extern_path(".proto.TopicID", "crate::services::TopicId")
        .extern_path(
            ".proto.TransactionFeeSchedule",
            "crate::services::TransactionFeeSchedule",
        )
        .extern_path(".proto.TransactionID", "crate::services::TransactionId")
        .extern_path(".proto.TransferList", "crate::services::TransferList")
        .extern_path(
            ".proto.HederaFunctionality",
            "crate::services::HederaFunctionality",
        )
        .extern_path(".proto.SubType", "crate::services::SubType")
        .extern_path(
            ".proto.TokenFreezeStatus",
            "crate::services::TokenFreezeStatus",
        )
        .extern_path(".proto.TokenKycStatus", "crate::services::TokenKycStatus")
        .extern_path(".proto.TokenSupplyType", "crate::services::TokenSupplyType")
        .extern_path(".proto.TokenType", "crate::services::TokenType")
        .extern_path(".proto.ContractID", "crate::services::ContractId")
        .out_dir(&streams_out_dir)
        .compile(
            &["proto/streams/account_balance_file.proto"],
            &["proto/streams/", "proto/services/"],
        )?;

    Ok(())
}
