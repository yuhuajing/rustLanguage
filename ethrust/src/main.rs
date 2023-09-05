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

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?.take(50);
    //let transEv:String = String::from("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

    while let Some(log) = stream.next().await {
        println!("{}",{format!("{:?}",log.topics[0])});
        let h256_str = format!("{:?}", log.topics[0]);
        match h256_str.as_str() {
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef" => println!("Transfer"),
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
