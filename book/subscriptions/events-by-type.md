# Subscribe events by type

```rust
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

const WSS_URL: &str = "wss://eth.getblock.io/ab0b1aa0-b490-4dc0-9bda-817c897a4580/mainnet";
const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let address: Address = WETH_ADDRESS.parse()?;
    let contract = IERC20::new(address, client);

    listen_all_events(&contract).await?;
   //listen_specific_events(&contract).await?;

    Ok(())
}

/// Given a contract instance, subscribe to all possible events.
/// This allows to centralize the event handling logic and dispatch
/// proper actions.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_all_events(contract: &IERC20<Provider<Ws>>) -> Result<()> {
    let events = contract.events().from_block(18069075).address(ethers::types::ValueOrArray::Value(WETH_ADDRESS.parse::<Address>()?));
    let mut stream = events.stream().await?.with_meta();

    while let Some(Ok((evt,meta))) = stream.next().await {
        match evt {
            IERC20Events::ApprovalFilter(f) => println!("{f:?}, {0:?}", meta.block_number),
            IERC20Events::TransferFilter(f) => println!("{f:?}, {0:?}", meta.block_number),
        }
    }

    Ok(())
}

/// Given a contract instance subscribe to a single type of event.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_specific_events(contract: &IERC20<Provider<Ws>>) -> Result<()> {
    let events = contract.event::<TransferFilter>().from_block(18069075).address(ethers::types::ValueOrArray::Value(WETH_ADDRESS.parse::<Address>()?));
    let mut stream = events.stream().await?.with_meta();

    while let Some(Ok((event, meta))) = stream.next().await {
        println!("src: {:?}, dst: {:?}, wad: {:?}", event.from, event.to, event.value);
        println!(
            r#"address: {:?}, 
               block_number: {:?}, 
               block_hash: {:?}, 
               transaction_hash: {:?}, 
               transaction_index: {:?}, 
               log_index: {:?}
            "#,
            meta.address,
            meta.block_number,
            meta.block_hash,
            meta.transaction_hash,
            meta.transaction_index,
            meta.log_index
        );
    }

    Ok(())
}
```
