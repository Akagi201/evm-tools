use alloy::signers::{k256::SecretKey, local::PrivateKeySigner};

pub fn gen_ecdsa_private_key() {
    let signer = PrivateKeySigner::random();
    let address = signer.address().to_checksum(None);
    let secret_key: SecretKey = signer.credential().into();
    let private_key = hex::encode(secret_key.to_bytes());
    let public_key = hex::encode(secret_key.public_key().to_sec1_bytes());
    println!("ECDSA Address: {}", address);
    println!("ECDSA Private Key: 0x{}", private_key);
    println!("ECDSA Public Key: 0x{}", public_key);
}

fn main() {
    gen_ecdsa_private_key();
}
