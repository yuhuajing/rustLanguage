use async_std::task;
use ethers::{
    core::{
        abi::AbiDecode,
        types::{Address, BlockNumber, Filter, U256},
    },
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use futures::future::join_all;
use futures::{join, try_join, AsyncWriteExt};
use std::string::String;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const WETH_ADDRESS: &str = "0xff2B4721F997c242fF406a626f17df083Bd2C568";
const WSS_URL: &str = "wss://eth.getblock.io/ab0b1aa0-b490-4dc0-9bda-817c897a4580/mainnet";

async fn get_ws_client() -> Provider<Ws> {
    Provider::<Ws>::connect(WSS_URL).await.unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Arc::new(get_ws_client().await);

    // let last_block = client
    //     .get_block(BlockNumber::Latest)
    //     .await?
    //     .unwrap()
    //     .number
    //     .unwrap();
    // println!("last_block: {last_block}");

    //getlogs(client.clone()).await?;
    //getbalance(client.clone()).await?;

    let latest_blocknumber = getlogs(client.clone());
    let addr_balance = getbalance(client.clone());
    join!(latest_blocknumber, addr_balance); //顺序执行 
    // try_join!(latest_blocknumber, addr_balance); //顺序执行
    //  let client_1 = client.clone();
    // let tasks = vec![
    // // task::spawn(async move { getlogs(client).await }),
    // // task::spawn(async move { getbalance(client_1).await }),
    // task::spawn(getlogs(client.clone())),
    // task::spawn(getbalance(client.clone())),
    // ];
    // // If we do not await these tasks and the function finishes, they will be dropped
    // join_all(tasks).await;

    Ok(())
}

async fn getbalance(client: Arc<Provider<Ws>>) -> Result<()> {
    let from_addr: &str = "0xc175006ED9Ee10210f466a043a300789a83C7420";
    //none is the latest blocknumber

    let balance = client.get_balance(from_addr, None).await?;
    println!("{balance}");
    Ok(())
}

async fn getlogs(client: Arc<Provider<Ws>>) -> Result<()> {
    let last_block = client
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();
    thread::sleep(Duration::from_secs(12));
    println!("last_block: {last_block}");
    Ok(())
    // let erc20_transfer_filter = Filter::new()
    //     .from_block(17943452)
    //     //  .event("Transfer(address,address,uint256)")
    //     .address(ethers::types::ValueOrArray::Value(
    //         WETH_ADDRESS.parse::<Address>()?,
    //     ));

    // let logs = client.get_logs(&erc20_transfer_filter).await?;

    // for log in logs.iter() {
    //     let h256_str = format!("{:?}", log.topics[0]);
    //     match h256_str.as_str() {
    //         "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef" => println!(
    //             "block: {:?}, tx: {:?}, token: {:?}, topic{:?}, from{:?},to{:?},value{:?}",
    //             log.block_number,
    //             log.transaction_hash,
    //             log.address,
    //             format!("{:?}", log.topics[0]),
    //             Address::from(log.topics[1]),
    //             Address::from(log.topics[2]),
    //             U256::decode(log.data.clone())
    //         ),
    //         _ => println!("others"),
    //     }
    // }
    //println!("{} tx found!", logs.iter().len());
}
