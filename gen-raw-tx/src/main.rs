use std::env;

use alloy::{
    consensus::TxEnvelope,
    eips::eip2718::Encodable2718,
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
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
    let counter_contract: Address = "0xcadA49f8CCb2BAD5FEE58b7A4C2d626C2FEbbDAC".parse()?; // on holesky
    let rpc_url = "http://176.9.142.29:38545";
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_builtin(rpc_url)
        .await?;
    let chain_id = provider.get_chain_id().await?;
    let nonce = provider.get_transaction_count(signer.address()).await?;
    println!("addr: {:?}, nonce: {:?}", signer.address(), nonce);
    let counter = Counter::new(counter_contract, provider.clone());
    let tx_req = counter.setNumber(U256::from(42)).into_transaction_request();
    println!("tx_req: {:?}", tx_req);
    let tx_type = tx_req.clone().preferred_type() as u8;
    let gas_limit = provider.estimate_gas(&tx_req).await?;
    let estimate_fee = provider.estimate_eip1559_fees(None).await?;
    let tx_req = tx_req
        .from(signer.address())
        .transaction_type(tx_type)
        .gas_limit(gas_limit)
        .nonce(nonce + 1)
        .max_fee_per_gas(estimate_fee.max_fee_per_gas)
        .max_priority_fee_per_gas(estimate_fee.max_priority_fee_per_gas);
    println!("tx_req: {:?}", tx_req);
    let envelope_tx =
        match <TransactionRequest as TransactionBuilder<Ethereum>>::with_chain_id(tx_req, chain_id)
            .build(&wallet)
            .await
        {
            Ok(tx) => tx,
            Err(e) => {
                println!("Error building transaction: {:?}", e);
                return Ok(());
            }
        };
    println!("envelope tx: {:?}", envelope_tx);
    let raw_bytes = envelope_to_raw_bytes(&envelope_tx);
    println!("raw bytes: {:?}", raw_bytes);
    let tx = provider
        .send_raw_transaction(raw_bytes.to_vec().as_slice())
        .await?;
    println!("tx: {:?}", tx);
    Ok(())
}

pub fn envelope_to_raw_bytes(tx: &TxEnvelope) -> Bytes {
    let mut encoded = Vec::new();
    // tx.network_encode(&mut encoded);
    tx.encode_2718(&mut encoded);
    encoded.into()
}
