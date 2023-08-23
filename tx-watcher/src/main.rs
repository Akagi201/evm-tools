use anyhow::Result;
use ethers::providers::{Middleware, Provider, StreamExt, Ws};

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://l2-orderly-l2-4460-sepolia-8tc3sd7dvy.t.conduit.xyz")
            .await?;
    // let provider = Provider::<Ws>::connect("wss://eth-mainnet.g.alchemy.com/v2/2P4bimKx9npB1QY8HxBU8pDghhvr6iMx")
    // .await?;

    let stream = match provider.watch_pending_transactions().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e:?}");
            panic!("Failed to create filter watcher for pending transactions!");
        }
    };

    let mut tx_stream = stream.transactions_unordered(usize::MAX);

    while let Some(tx) = tx_stream.next().await {
        println!("[IN] Received pending transaction: {:?}", tx);
        let tx = match tx {
            Ok(tx) => tx,
            Err(e) => {
                println!("Transaction error: {:?}", e);
                continue;
            }
        };

        // Get the transaction receipt
        match provider.get_transaction_receipt(tx.hash).await {
            Ok(Some(r)) => {
                println!("Found transaction receipt {:?}, skipping...", r);
                continue;
            }
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
            Ok(None) => { /* No Transaction, we can proceed with sandwiching */ }
        }
    }

    Ok(())
}
