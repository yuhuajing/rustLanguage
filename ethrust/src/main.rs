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
