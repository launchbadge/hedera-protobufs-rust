use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use hedera-crypto::PrivateKey;

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
    let key_to_bytes = PrivateKey::to_bytes(&config.operator.privateKey);

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L35


    let data = services::TransactionBody {
        // declare new enum Data
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
