// crates/ledger/src/crypto.rs
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}

pub fn sign_message(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn verify_signature(public_key: &PublicKey, message: &[u8], sig: &Signature) -> bool {
    public_key.verify(message, sig).is_ok()
}
