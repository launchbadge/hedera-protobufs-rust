use http::Uri;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect a CryptoClient to a testnet node
    //0.testnet.hedera.com:50211
    let test_net_uri = "http:://0.testnet.hedera.com:50211".parse().unwrap();

    let channel = tonic::transport::Channel::builder(test_net_uri)
        .connect()
        .await?;

    // FIXME: Create a AccountBalanceQuery
    

    // FIXME: Send the request to get_account_balance
    // FIXME: Print the result in tiny bars

    Ok(())
}

