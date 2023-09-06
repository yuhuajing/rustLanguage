返回logs数组
```rust
use ethers::{
    core::types::{Address, Filter, H160, H256, U256,U64},
    providers::{Http, Middleware, Provider},
};
use eyre::Result;
use std::sync::Arc;

const HTTP_URL: &str = "https://rpc.flashbots.net";
const V3FACTORY_ADDRESS: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
const DAI_ADDRESS: &str = "0x04C54cC7d44c93f5Be2B1f549611bA60a8252251";
const USDC_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

/// This example demonstrates filtering and parsing event logs by fetching all Uniswap V3 pools
/// where both tokens are in the set [USDC, USDT, DAI].
///
/// V3 factory reference: https://github.com/Uniswap/v3-core/blob/main/contracts/interfaces/IUniswapV3Factory.sol
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);
    let token_topics = [
        H256::from(USDC_ADDRESS.parse::<H160>()?),
        H256::from(USDT_ADDRESS.parse::<H160>()?),
        H256::from(DAI_ADDRESS.parse::<H160>()?),
    ];
    let filter = Filter::new()
        .address(V3FACTORY_ADDRESS.parse::<Address>()?)
        .event("PoolCreated(address,address,uint24,int24,address)")
        .topic1(token_topics.to_vec())
        .topic2(token_topics.to_vec())
        .from_block(0);
    let logs = client.get_logs(&filter).await?;
    println!("{} pools found!", logs.iter().len());
    for log in logs.iter() {
        let token0 = Address::from(log.topics[1]);
        let token1 = Address::from(log.topics[2]);
        let fee_tier = U256::from_big_endian(&log.topics[3].as_bytes()[29..32]);
        let tick_spacing = U256::from_big_endian(&log.data[29..32]);
        let pool = Address::from(&log.data[44..64].try_into()?);
        let block_number = match log.block_number {
            Some(value) => value,
            None => {
                // 处理 log.block_number 为 None 的情况，这里可以返回一个默认值或执行其他操作
                U64::zero() // 作为示例，返回 U64 类型的零值
            }
        };

        println!(
            "block_number={block_number}, pool = {pool}, token0 = {token0}, token1 = {token1}, fee = {fee_tier}, spacing = {tick_spacing}"
        );
    }
    Ok(())
}
```