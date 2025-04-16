// crates/ledger/src/engine.rs
use crate::crypto::verify_signature;

impl Ledger {
    pub fn apply_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Generate canonical message
        let message = format!("{}{}{}{}", tx.from, tx.to, tx.amount, tx.timestamp)
            .as_bytes()
            .to_vec();

        if !verify_signature(&tx.public_key, &message, &tx.signature) {
            return Err("Invalid signature".into());
        }

        let from_acc = self.accounts.get_mut(&tx.from);
        let to_acc = self.accounts.get_mut(&tx.to);

        match (from_acc, to_acc) {
            (Some(from), Some(to)) => {
                if from.balance < tx.amount {
                    return Err("Insufficient balance".into());
                }
                from.balance -= tx.amount;
                to.balance += tx.amount;
                self.transactions.push(tx);
                Ok(())
            }
            _ => Err("Account not found".into()),
        }
    }
}
