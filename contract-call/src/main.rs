use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug)]
//     Counter,
//     "abi/Counter.json"
// );

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set");
    let counter_contract: Address = "0xcadA49f8CCb2BAD5FEE58b7A4C2d626C2FEbbDAC".parse()?;
    let rpc_url = "https://ethereum-holesky-rpc.publicnode.com";
    let signer: PrivateKeySigner = private_key.parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::new(signer))
        .on_builtin(rpc_url)
        .await?;
    let counter = Counter::new(counter_contract, provider.clone());
    let hash = counter
        .setNumber(U256::from(42))
        .send()
        .await?
        .watch()
        .await?;
    println!("Set number hash: {:?}", hash);
    let number = counter.number().call().await?;
    println!("Get number: {:?}", number);
    Ok(())
}
