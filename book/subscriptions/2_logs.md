# Subscribing to Logs

To subscribe to logs, create a Filter object that specifies the criteria for the logs you want to listen to. Then, pass the filter to the Provider's subscribe_logs method:

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let filter = Filter::new().address("0xcontract_address_here".parse()?);

    let mut stream = provider.subscribe_logs(filter).await?;

    // Your code to handle logs goes here.

    Ok(())

}
```

You can now listen to logs that match your filter criteria:

```rust
while let Some(log) = stream.next().await {
    match log {
        Ok(log) => {
            println!("New log: {:?}", log);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

Here is another example of subscribing to logs:

```rust
use ethers::{
    core::{
        abi::AbiDecode,
        types::{Address, BlockNumber, Filter, U256},
    },
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;
use std::string::String;

const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Provider::<Ws>::connect(
        "wss://eth.getblock.io/ab0b1aa0-b490-4dc0-9bda-817c897a4580/mainnet",
    )
    .await?;
    let client = Arc::new(client);

    let last_block = client
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();
    println!("last_block: {last_block}");

    let erc20_transfer_filter = Filter::new()
        .from_block(last_block - 25)
      //  .event("Transfer(address,address,uint256)")
        .address(ethers::types::ValueOrArray::Value(
            WETH_ADDRESS.parse::<Address>()?,
        ));

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?.take(500);
    let transEv:String = String::from("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

    while let Some(log) = stream.next().await {
        match format!("{:?}",log.topics[0]){
            transEv => println!("Transfer"),
            _ => println!("others"),
        }
        // println!(
        //     "block: {:?}, tx: {:?}, token: {:?}, topic{:?}, from{:?},to{:?},value{:?}",
        //     log.block_number,
        //     log.transaction_hash,
        //     log.address,
        //     format!("{:?}",log.topics[0]),
        //    Address::from(log.topics[1]),
        //    Address::from(log.topics[2]),
        //    U256::decode(log.data)
        // );
    }

    Ok(())
}
```
