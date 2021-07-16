use anyhow::Error;
use dotenv;
use hedera_crypto::PrivateKey;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use prost::Message;
use serde::Deserialize;
use serde_json;
use std::str::FromStr;
use std::time::Duration;
use std::{env, fs};
use time::OffsetDateTime;
use tonic::transport::Channel;

#[derive(Deserialize, Debug)]
struct NewAccount {
    operator: Operator,
    network: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Operator {
    account_id: String,
    private_key: String,
}

fn get_new_account() -> anyhow::Result<NewAccount> {
    let _ = dotenv::dotenv();
    pretty_env_logger::try_init_timed()?;
    let file_path = env::var("CONFIG_FILE").unwrap();
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    // Read the JSON contents of the file as an instance of 'NewAccount'
    let config: NewAccount = serde_json::from_str(&contents)?;
    Ok(config)
}

fn create_new_account_id(shard_num: i64, realm_num: i64, account_num: i64) -> services::AccountId {
    services::AccountId {
        shard_num,
        realm_num,
        account_num,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // https://docs.hedera.com/guides/testnet/testnet-nodes#testnet-nodes
    let channel = Channel::builder("tcp://0.testnet.hedera.com:50211".parse()?)
        .connect()
        .await?;

    let mut client = CryptoServiceClient::new(channel);

    // Read the JSON contents of the file as an instance of 'NewAccount'
    let config: NewAccount = get_new_account().unwrap();

    // Operator Keypair
    let private_key = PrivateKey::from_str(&config.operator.private_key)?;
    let public_key = private_key.public_key().to_bytes();

    // Generated Keypair for New Account
    let private_key_generated = PrivateKey::generate();
    let public_key_generated = private_key_generated.public_key().to_bytes();

    let operator_account_vec: Vec<&str> = config.operator.account_id.rsplit('.').collect();

    // Define and populate the structs for create account
    let node_account_id = create_new_account_id(0, 0, 3);

    let account_id = create_new_account_id(
        operator_account_vec[0].parse::<i64>().unwrap(),
        operator_account_vec[1].parse::<i64>().unwrap(),
        operator_account_vec[2].parse::<i64>().unwrap(),
    );

    // 90 day duration
    let duration = services::Duration { seconds: 7776000 };

    let data = services::transaction_body::Data::CryptoCreateAccount(
        services::CryptoCreateTransactionBody {
            key: Some(services::Key {
                key: Some(services::key::Key::Ed25519(public_key_generated.to_vec())),
            }),
            initial_balance: 1000,
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
        transaction_valid_start: Some(services::Timestamp {
            seconds: now_in_seconds,
            nanos: now_in_nanos,
        }),
        account_id: Some(account_id),
        scheduled: false,
    };

    let new_account = services::TransactionBody {
        transaction_id: Some(transaction_id.clone()),
        node_account_id: Some(node_account_id),
        transaction_fee: 2000000000000,
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
    let sig_pair = vec![services::SignaturePair {
        pub_key_prefix: public_key.to_vec(),
        signature: Some(services::signature_pair::Signature::Ed25519(
            signature.to_vec(),
        )),
    }];

    let signed_transaction = services::SignedTransaction {
        body_bytes: account_bytes,
        sig_map: Some(services::SignatureMap { sig_pair: sig_pair }),
    };

    // construct a transaction
    let transaction = services::Transaction {
        signed_transaction_bytes: signed_transaction.encode_to_vec(),
        ..Default::default()
    };

    // get response to creating the account
    let response = client.create_account(transaction).await?;

    // view account
    let t_response = Some(response);
    println!("{:#?}", t_response);

    let query = services::Query {
        query: Some(services::query::Query::TransactionGetReceipt(
            services::TransactionGetReceiptQuery {
                header: None,
                transaction_id: Some(transaction_id.clone()),
                include_duplicates: false,
            },
        )),
    };

    // wait for consensus
    tokio::time::sleep(Duration::from_secs(15)).await;

    // query the receipt
    let receipt = client.get_transaction_receipts(query).await?;
    println!("{:#?}", receipt);

    Ok(())
}
