use std::time::Duration;
use hedera_crypto::PrivateKey;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use prost::Message;
use std::env;
use std::str::FromStr;
use time::OffsetDateTime;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let _ = dotenv::dotenv();

    // https://docs.hedera.com/guides/testnet/testnet-nodes#testnet-nodes
    let channel = Channel::builder("tcp://0.testnet.hedera.com:50211".parse()?)
        .connect()
        .await?;

    let mut client = CryptoServiceClient::new(channel);

    let my_account_id = &env::var("TESTNET_ID")?[4..10];
    let my_account_private_key = PrivateKey::from_str(&env::var("TESTNET_PRIVATE_KEY")?)?;
    let my_account_public_key = my_account_private_key.public_key().to_bytes();

    // Generated Keypair for New Account
    let new_account_private_key = PrivateKey::generate();
    let new_account_public_key= new_account_private_key.public_key().to_bytes();

    // Define and populate the structs for create account
    let node_account_id = services::AccountId {
        shard_num: 0,
        realm_num: 0,
        account_num: 3,
    };

    let account_id = services::AccountId {
        shard_num: 0,
        realm_num: 0,
        account_num: my_account_id.parse::<i64>()?,
    };

    // 90 day duration
    let duration = services::Duration { seconds: 7776000 };

    let data = services::transaction_body::Data::CryptoCreateAccount(
        services::CryptoCreateTransactionBody {
            key: Some(services::Key {
                key: Some(services::key::Key::Ed25519(new_account_public_key.to_vec())),
            }),
            auto_renew_period: Some(duration),
            ..Default::default()
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
        account_id: Some(account_id.clone()),
        scheduled: false,
    };

    let new_account = services::TransactionBody {
        transaction_id: Some(transaction_id.clone()),
        node_account_id: Some(node_account_id.clone()),
        transaction_fee: 200000000,
        transaction_valid_duration: Some(services::Duration { seconds: 120 }),
        memo: String::new(),
        data: Some(data),
        ..Default::default()
    };

    // serialize account struct
    let account_bytes = new_account.encode_to_vec();

    // sign the bytes
    let signature = my_account_private_key.sign(&account_bytes);

    // creates signed transaction so the account can be created
    let sig_pair = vec![services::SignaturePair {
        pub_key_prefix: my_account_public_key.to_vec(),
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
    let response= client.get_transaction_receipts(query).await?;
    
    let new_account_num = match response.into_inner().response {
        Some(services::response::Response::TransactionGetReceipt(response)) => {
            //response.receipt,
            match response.receipt {
                Some(response) => {
                    match response.account_id {
                        Some(response) => response.account_num,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };

    let new_account_id = services::AccountId {
        shard_num: 0,
        realm_num: 0,
        account_num: new_account_num as i64,
    };

    println!("New Account number Account Number = 0.0.{:#?} ", new_account_num);

    // check the new account's balance
    let query = services::Query {
        query: Some(services::query::Query::CryptogetAccountBalance(
            services::CryptoGetAccountBalanceQuery {
                header: None,
                balance_source: Some(
                    services::crypto_get_account_balance_query::BalanceSource::AccountId(
                        services::AccountId {
                            account_num: new_account_num as i64,
                            shard_num: 0,
                            realm_num: 0,
                        },
                    ),
                ),
            },
        )),
    };

    let response = client.crypto_get_balance(query).await?;

    let response = match response.into_inner().response {
        Some(services::response::Response::CryptogetAccountBalance(response)) => response,
        _ => unreachable!(),
    };
    println!("The new account's balance is: {} tℏ", response.balance);

    // transfer hbar
    let transfer_transaction_data = services::transaction_body::Data::CryptoTransfer(
        services::CryptoTransferTransactionBody {
            transfers: Some(services::TransferList {
                account_amounts: vec![
                services::AccountAmount {
                    account_id: Some(account_id.clone()),
                    amount: -1000,
                },
                services::AccountAmount {
                    account_id: Some(new_account_id.clone()),
                    amount: 1000,
                }],
            }),
            token_transfers: Vec::new(),
        }
    );

    let transfer_transaction_id = services::TransactionId {
        transaction_valid_start: Some(services::Timestamp {
            seconds: now_in_seconds + 1,
            nanos: now_in_nanos,
        }),
        account_id: Some(account_id.clone()),
        scheduled: false,
    };

    let transfer_transaction_body = services::TransactionBody {
        transaction_id: Some(transfer_transaction_id.clone()),
        node_account_id: Some(node_account_id.clone()),
        transaction_fee: 200000000,
        transaction_valid_duration: Some(services::Duration { seconds: 120 }),
        memo: String::new(),
        data: Some(transfer_transaction_data),
        ..Default::default()
    };

    let transfer_account_bytes = transfer_transaction_body.encode_to_vec();

    let transfer_signature = my_account_private_key.sign(&transfer_account_bytes);
    let receive_signature = new_account_private_key.sign(&transfer_account_bytes);
    
    let transfer_sig_pair = vec![
        services::SignaturePair {
            pub_key_prefix: my_account_public_key.to_vec(),
            signature: Some(services::signature_pair::Signature::Ed25519(
                transfer_signature.to_vec(),
        )),
        },
        services::SignaturePair {
            pub_key_prefix: new_account_public_key.to_vec(),
            signature: Some(services::signature_pair::Signature::Ed25519(
                receive_signature.to_vec(),
        )),
    }];

    let transfer_signed_transaction = services::SignedTransaction {
        body_bytes: transfer_account_bytes,
        sig_map: Some(services::SignatureMap { sig_pair: transfer_sig_pair.clone() }),
    };

    let transfer_transaction = services::Transaction {
        signed_transaction_bytes: transfer_signed_transaction.encode_to_vec(),
        ..Default::default()
    };

    let transfer_response = client.crypto_transfer(transfer_transaction).await?;

    println!("Transfer Hbar Response: {:#?}", Some(transfer_response));

    // Query Receipt for Hbar transfer to new account
    let transfer_query = services::Query {
        query: Some(services::query::Query::TransactionGetReceipt(
            services::TransactionGetReceiptQuery {
                header: None,
                transaction_id: Some(transfer_transaction_id.clone()),
                include_duplicates: false,
            },
        )),
    };

    // wait for consensus
    tokio::time::sleep(Duration::from_secs(15)).await;

    println!("Transfer Hbar receipt: {:#?}", &transfer_query);

    // Query the new account's balance
    let new_account_balance_query = services::Query {
        query: Some(services::query::Query::CryptogetAccountBalance(
            services::CryptoGetAccountBalanceQuery {
                header: None,
                balance_source: Some(
                    services::crypto_get_account_balance_query::BalanceSource::AccountId(
                        services::AccountId {
                            account_num: new_account_num,
                            shard_num: 0,
                            realm_num: 0,
                        },
                    ),
                ),
            },
        )),
    };

    let new_account_balance_response = client.crypto_get_balance(new_account_balance_query).await?;
    
    // wait for consensus
    tokio::time::sleep(Duration::from_secs(15)).await;
    
    let new_account_balance_response = match new_account_balance_response.into_inner().response {
        Some(services::response::Response::CryptogetAccountBalance(response)) => response,
        _ => unreachable!(),
    };

    println!("The new account balance after transfer is: {} tℏ", new_account_balance_response.balance);

    Ok(())
}