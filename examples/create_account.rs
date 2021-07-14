use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use hedera_crypto::PrivateKey;

#[derive(Deserialize, Debug)]
struct NewAccount {
    operator: Operator,
    network: String,
    mirrorNetwork: String,
}

#[derive(Deserialize, Debug)]
struct Operator {
    accountId: String,
    privateKey: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // https://docs.hedera.com/guides/testnet/testnet-nodes#testnet-nodes
    let channel = Channel::builder("tcp://0.testnet.hedera.com:50211".parse()?)
        .connect()
        .await?;

    let mut client = CryptoServiceClient::new(channel);

    // Open the file in read-only mode with buffer.
    let file = File::open("/home/leahwhalen/Documents/hedera-protobufs-rust/CONFIG_FILE.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let config : NewAccount = serde_json::from_reader(reader)?;


    // let config: serde_json::Value =
    //     serde_json::from_str("/home/leahwhalen/Documents/hedera-protobufs-rust/CONFIG_FILE.json").expect("JSON was not well-formatted");

    println!("{:?}", config);
    let key_bytes = config.operator.privateKey.as_bytes();

    let key_to_bytes = PrivateKey::from_bytes(&key_bytes).unwrap();

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L35


    let data = services::TransactionBody {
        transactionID : TransactionID  = 1, // The ID for this transaction, which includes the payer's account (the account paying the transaction fee). If two transactions have the same transactionID, they won't both have an effect
        nodeAccountID : AccountID  = config.operator.accountId, // The account of the node that submits the client's transaction to the network
        transactionFee : u64 = 3, // The maximum transaction fee the client is willing to pay
        transactionValidDuration : Duration = 4, //The transaction is invalid if consensusTimestamp > transactionID.transactionValidStart + transactionValidDuration
        bool generateRecord = 5 [deprecated = true]; // Should a record of this transaction be generated? (A receipt is always generated, but the record is optional)
        string memo
        data: Some(services::transaction_body::Data::CryptoCreateAccount (
            services::CryptoCreateTransactionBody {
                key: Some( services::Key {
                    key: Some(services::key::Key::Ed25519(key_to_bytes.to_vec()))
                }),
                initial_balance: 0,
                proxy_account_id: None,
                send_record_threshold: 1000000,
                receive_record_threshold: 1000000,
                receiver_sig_required: false,
                auto_renew_period: None,
                shard_id: None,
                realm_id: None,
                new_realm_admin_key: None,
                memo: config.operator.accountId,
            }
        ) )
    };

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoService.proto#L52
    let response = client.create_account(data).await?;

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L47
    let response = match response.into_inner().response {
        Some(services::response::Response::CryptoCreateTransactionBody(response)) => response,
        _ => unreachable!(),
    };

    println!("{}", response.to_string());

    Ok(())
}
