use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // https://docs.hedera.com/guides/testnet/testnet-nodes#testnet-nodes
    let channel = Channel::builder("tcp://0.testnet.hedera.com:50211".parse()?)
        .connect()
        .await?;

    let mut client = CryptoServiceClient::new(channel);

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L35
    let query = services::Query {
        query: Some(services::query::Query::CryptogetAccountBalance(
            services::CryptoGetAccountBalanceQuery {
                header: None,
                balance_source: Some(
                    services::crypto_get_account_balance_query::BalanceSource::AccountId(
                        services::AccountId {
                            account_num: 2,
                            shard_num: 0,
                            realm_num: 0,
                        },
                    ),
                ),
            },
        )),
    };

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoService.proto#L52
    let response = client.crypto_get_balance(query).await?;

    // https://github.com/hashgraph/hedera-protobufs/blob/main/services/CryptoGetAccountBalance.proto#L47
    let response = match response.into_inner().response {
        Some(services::response::Response::CryptogetAccountBalance(response)) => response,
        _ => unreachable!(),
    };

    println!("balance = {} tℏ", response.balance);

    Ok(())
}
