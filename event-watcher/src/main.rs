use anyhow::Result;
use ethers::providers::{Middleware, Provider, StreamExt, Ws};

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://l2-orderly-l2-4460-sepolia-8tc3sd7dvy.t.conduit.xyz")
            .await?;
    let mut stream = provider.subscribe_blocks().await?.take(1);
    while let Some(block) = stream.next().await {
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }

    Ok(())
}
