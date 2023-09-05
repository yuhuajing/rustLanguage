# WebSocket provider

The Ws provider allows you to send JSON-RPC requests and receive responses over WebSocket connections. The WS provider can be used with any Ethereum node that supports WebSocket connections. This allows programs interact with the network in real-time without the need for HTTP polling for things like new block headers and filter logs. Ethers-rs has support for WebSockets via Tokio. Make sure that you have the “ws” and “rustls” / “openssl” features enabled in your project's toml file if you wish to use WebSockets.

## Initializing a WS Provider

Lets look at a few ways to create a new `WS` provider. Below is the most straightforward way to initialize a new `Ws` provider.

```rust
use ethers::providers::{Provider, Ws};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Provider::<Ws>::connect("wss://...").await?;
    Ok(())
}
```

Similar to the other providers, you can also establish an authorized connection with a node via websockets.

```rust
use ethers::providers::{Authorization, Provider, Ws};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let url = "wss://...";
    let auth = Authorization::basic("username", "password");
    let provider = Provider::<Ws>::connect_with_auth(url, auth).await?;
    Ok(())
}
```

## Usage

The `Ws` provider allows a user to send requests to the node just like the other providers. In addition to these methods, the `Ws` provider can also subscribe to new logs and events, watch transactions in the mempool and other types of data streams from the node.

In the snippet below, a new `Ws` provider is used to subscribe to new pending transactions in the mempool as well as new block headers in two separate threads.

```rust
//! The Ws transport allows you to send JSON-RPC requests and receive responses over
//! [WebSocket](https://en.wikipedia.org/wiki/WebSocket).
//!
//! This allows to interact with the network in real-time without the need for HTTP
//! polling.

use ethers::prelude::*;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // A Ws provider can be created from a ws(s) URI.
    // In case of wss you must add the "rustls" or "openssl" feature
    // to the ethers library dependency in `Cargo.toml`.
    let provider = Provider::<Ws>::connect(WSS_URL).await?;

    // let mut stream = provider.subscribe_blocks().await?.take(1); // 仅仅监听一次
    let mut stream = provider.subscribe_blocks().await?; //持续监听

    while let Some(block) = stream.next().await {
        println!("{:?}", block.hash);
    }

    Ok(())
}
```
