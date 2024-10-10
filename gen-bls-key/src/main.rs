use blst::min_pk::SecretKey;
use rand::RngCore;

pub fn gen_bls_keypair() {
    let mut rng = rand::thread_rng();
    let mut ikm = [0u8; 32];
    rng.fill_bytes(&mut ikm);

    let sk = SecretKey::key_gen(&ikm, &[]).unwrap();
    let pk = sk.sk_to_pk();
    let sk_hex = hex::encode(sk.to_bytes());
    let pk_hex = hex::encode(pk.to_bytes());

    println!("BLS Secret Key: {}, len: {}", sk_hex, sk_hex.len() / 2);
    println!("BLS Public Key: {}, len: {}", pk_hex, pk_hex.len() / 2);
}
fn main() {
    gen_bls_keypair()
}
