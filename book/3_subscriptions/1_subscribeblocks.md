# Subscribing to New Blocks

To subscribe to new blocks, create a Provider instance and call the subscribe_blocks method:

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let mut stream = provider.subscribe_blocks().await?;

    // Your code to handle new blocks goes here.

    Ok(())

}
```

You can now listen to new blocks as they are mined:

```rust
while let Some(block) = stream.next().await {
    match block {
        Ok(block) => {
            println!("New block: {:?}", block);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

Here is another example of subscribing to new blocks:

```rust
use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://eth.getblock.io/ab0b1aa0-b490-4dc0-9bda-817c897a4580/mainnet")
            .await?;
    let mut stream = provider.subscribe_blocks().await?;
    while let Some(block) = stream.next().await {
        match block {
            Ok(block) => {
                println!(
                    "Ts: {:?}, block number: {} -> {:?}",
                    block.timestamp,
                    block.number.unwrap(),
                    block.hash.unwrap()
                );
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    Ok(())
}
```
