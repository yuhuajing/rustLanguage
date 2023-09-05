## 建立连接
```rust
    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);
```
## 功能
```rust
    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;
    let tx_pool_content = provider.txpool_content().await?;
```

通过 abi 或者 可视化的函数方法 call 合约函数
```rust
use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::Address,
};
use std::sync::Arc;

abigen!(
    IUniswapV2Pair, // 结构体
    r#"[function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)]"#
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

    // Initialize a new instance of the Weth/Dai Uniswap V2 pair contract
    let pair_address: Address = "0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11".parse()?;
    let uniswap_v2_pair = IUniswapV2Pair::new(pair_address, provider);

    // Use the get_reserves() function to fetch the pool reserves
    let (reserve_0, reserve_1, block_timestamp_last) =
        uniswap_v2_pair.get_reserves().call().await?;
    println!("{reserve_0},{reserve_1},{block_timestamp_last}");

    Ok(())
}
```
## 多线程
在多线程中存在数据的 `move` 操作，所以数据的 ownership 发生变化，因此在多线程的操作中，需要用关键字 `Arc` 引用计数，处理数据拷贝和引用的问题。
```rust
use ethers::providers::{Http, Middleware, Provider};
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

    let current_block_number = provider.get_block_number().await?;
    let prev_block_number = current_block_number - 1;

    // Clone the Arc<Provider> and pass it into a new thread to get the uncle count of the current block
    let provider_1 = provider.clone();
    // move 会导致数据的
    let task_0 = tokio::spawn(async move { provider.get_uncle_count(current_block_number).await });

    // Spin up a new thread to get the uncle count of the previous block
    let task_1 = tokio::spawn(async move { provider_1.get_uncle_count(prev_block_number).await });

    // Wait for the tasks to finish
    for task in [task_0, task_1] {
        if let Ok(uncle_count) = task.await? {
            println!("{uncle_count}");
        }
    }
    // let block_number = provider.get_block_number().await?; // Error: value borrowed here after move
    // println!("{block_number}");
    Ok(())
}
```
