// crates/ledger/src/types.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub balance: u64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: uuid::Uuid,
    pub from: uuid::Uuid,
    pub to: uuid::Uuid,
    pub amount: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub public_key: PublicKey,
    pub signature: Signature,
}
