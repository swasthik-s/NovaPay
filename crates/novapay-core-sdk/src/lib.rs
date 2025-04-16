// novapay-core-sdk/src/lib.rs

pub mod types;
pub mod engine;
pub mod crypto;

// This is the main SDK interface
use crate::{engine::Ledger, crypto::{generate_keypair, sign_message, verify_signature}};
use ed25519_dalek::{Keypair, Signature, PublicKey};
use uuid::Uuid;
use chrono::Utc;

pub fn create_ledger() -> Ledger {
    Ledger::new()
}

pub fn new_keypair() -> Keypair {
    generate_keypair()
}

pub fn create_transaction(
    from: Uuid,
    to: Uuid,
    amount: u64,
    keypair: &Keypair
) -> crate::types::Transaction {
    let timestamp = Utc::now();
    let msg = format!("{}{}{}{}", from, to, amount, timestamp);
    let sig = sign_message(keypair, msg.as_bytes());

    crate::types::Transaction {
        id: Uuid::new_v4(),
        from,
        to,
        amount,
        timestamp,
        public_key: keypair.public,
        signature: sig,
    }
}

pub fn verify_tx_signature(tx: &crate::types::Transaction) -> bool {
    let msg = format!("{}{}{}{}", tx.from, tx.to, tx.amount, tx.timestamp);
    verify_signature(&tx.public_key, msg.as_bytes(), &tx.signature)
}
