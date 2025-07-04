use crate::types::block::{Block, Transaction};
use sha2::{Digest, Sha256};

pub fn block_hasher(block: Block) -> String {
    let mut hasher = Sha256::new();
    let transactions = match serde_json::to_string(&block.transactions) {
        Ok(transactions) => transactions,
        Err(e) => {
            log::error!("Error occured while serializing transactions: {e}");
            return Default::default();
        }
    };
    let input = format!(
        "{}{}{}{}{}{}",
        block.index, block.prev_hash, block.nonce, block.timestamp, transactions, block.merkle_root
    );
    hasher.update(input.as_bytes());
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    hex_digest
}

// Merkle Root: Hash of all the transactions included in the block
pub fn transactions_hasher(transactions: Vec<Transaction>) -> String {
    let mut hasher = Sha256::new();
    let transactions = match serde_json::to_string(&transactions) {
        Ok(transactions) => transactions,
        Err(e) => {
            log::error!("Error occured while serializing transactions: {e}");
            return Default::default();
        }
    };

    hasher.update(transactions.as_bytes());
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    hex_digest
}
