use ethers::{
    prelude::{rand, LocalWallet},
    signers::Signer,
    utils::hex::ToHex,
};

pub fn generate_evm_private_key() -> (String, String) {
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let full_address = format!("0x{:x}", wallet.address());
    let private_key = wallet.signer().to_bytes().encode_hex::<String>();
    let private_key = format!("0x{}", private_key);
    // println!("EVM Private Key: {}", private_key);
    // println!("EVM Address: {}", full_address);
    (full_address, private_key)
}

fn main() {
    let (full_address, private_key) = generate_evm_private_key();
    println!("EVM Private Key: {}", private_key);
    println!("EVM Address: {}", full_address);
}
