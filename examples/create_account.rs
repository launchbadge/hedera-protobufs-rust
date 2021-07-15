use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use prost::Message;
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use tonic::transport::Channel;
use time::{NumericalDuration, OffsetDateTime};
use hedera_crypto::PrivateKey;
use std::str::FromStr;

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

    // Read the JSON contents of the file as an instance of 'NewAccount'
    let config: NewAccount = serde_json::from_reader(reader)?;

    println!("{:?}", config);
    let private_key = PrivateKey::from_str(&config.operator.privateKey)?;
    let public_key = private_key.public_key().to_bytes();

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L35
    // Define and populate the structs for create account
    let node_account_id = services::AccountId {
        shard_num: 0,
        realm_num: 0,
        account_num: 3,
    };

    let account_id = services::AccountId {
        shard_num: 0,
        realm_num: 0,
        account_num: 2104142,
    };

    // 90 day duration
    let duration = services::Duration{
        seconds: 90 * 24 * 60 * 60,
    };

    let data = services::transaction_body::Data::CryptoCreateAccount(
        services::CryptoCreateTransactionBody {
            key: Some(services::Key {
                key: Some(services::key::Key::Ed25519(public_key.to_vec())),
            }),
            initial_balance: 0,
            proxy_account_id: None,
            send_record_threshold: 1000000,
            receive_record_threshold: 1000000,
            receiver_sig_required: false,
            auto_renew_period: Some(duration),
            shard_id: None,
            realm_id: None,
            new_realm_admin_key: None,
            memo: String::new(),
        },
    );

    let now = OffsetDateTime::now_utc();
    let now_in_seconds = now.unix_timestamp() as i64;
    let now_in_nanos = now.nanosecond() as i32;

    let transaction_id = services::TransactionId {
        transaction_valid_start: Some(services::Timestamp { seconds: now_in_seconds, nanos: now_in_nanos }),
        account_id: Some(account_id),
        scheduled: false,
    };

    let new_account = services::TransactionBody {
        transaction_id: Some(transaction_id),
        node_account_id: Some(node_account_id),
        transaction_fee: 10000000000000,
        transaction_valid_duration: Some(services::Duration { seconds: 120 }),
        memo: String::new(),
        data: Some(data),
        generate_record: false,
    };

    // serialize account struct
    let account_bytes = new_account.encode_to_vec();

    // sign the bytes
    let signature = private_key.sign(&account_bytes);

    // creates signed transaction so the account can be created
    let sig_pair = vec!(services::SignaturePair {
        pub_key_prefix: public_key.to_vec(),
        signature: Some(services::signature_pair::Signature::Ed25519(signature.to_vec())),
    });

    let signed_transaction = services::SignedTransaction {
        body_bytes: account_bytes,
        sig_map: Some(services::SignatureMap { 
            sig_pair: sig_pair,
        })
    };

    // construct a transaction
    let transaction = services::Transaction {
        signed_transaction_bytes: signed_transaction.encode_to_vec(),
        ..Default::default()
    };

    // // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoService.proto#L52
    let response = client.create_account(transaction).await?;

    // // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L47

    let t_response = Some(response);
    println!("{:#?}", t_response);

    // query the reciept
    

    Ok(())
}
