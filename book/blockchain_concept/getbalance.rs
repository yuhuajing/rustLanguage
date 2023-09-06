use ethers::{
    core::types::{BlockId, BlockNumber},
    providers::{Http, Middleware, Provider},
};
use eyre::Result;
use std::convert::TryFrom;

const HTTTP_URL: &str = "https://cloudflare-eth.com";
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(HTTTP_URL)?;
    let from_addr: &str = "0xc175006ED9Ee10210f466a043a300789a83C7420";
    //none is the latest blocknumber
    let balance = provider.get_balance(from_addr, None).await?;
    let nonce = provider.get_transaction_count(from_addr, None).await?;
    println!("{balance},{nonce}");

    let balance2 = provider
        .get_balance(from_addr, Some(BlockId::Number(BlockNumber::Number(18074115u64.into()))))
        .await?;
    let nonce2 = provider
        .get_transaction_count(from_addr, Some(BlockId::Number(BlockNumber::Number(18074115u64.into()))))
        .await?;
    println!("{balance2},{nonce2}");

    Ok(())
}
