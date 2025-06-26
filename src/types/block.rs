use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::blockchain::block;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    // Self explanatory
    pub index: u32,

    // hash of last block in order to prevent tampering in the previous block
    pub prev_hash: String,

    // hash of this block
    pub hash: Option<String>,

    // random number used for mining
    pub nonce: u32,

    // the time when the block was created
    pub timestamp: DateTime<Utc>,

    // the group of transactions take for this block from mempool
    pub transactions: Vec<Transaction>,

    // the hash of all the transactions in the block
    pub merkle_root: String,
}

impl Block {
    pub fn block_hasher(&self) -> String {
        let mut hasher = Sha256::new();
        let transactions = match serde_json::to_string(&self.transactions) {
            Ok(transactions) => transactions,
            Err(e) => {
                log::error!("Error occured while serializing transactions: {e}");
                return Default::default();
            }
        };
        let input = format!(
            "{}{}{}{}{}{}",
            self.index, self.prev_hash, self.nonce, self.timestamp, transactions, self.merkle_root
        );
        hasher.update(input.as_bytes());
        let digest = hasher.finalize();
        let hex_digest = hex::encode(digest);

        hex_digest
    }

    pub fn proof_of_work(&mut self, difficulty: usize) -> Block {
        let target = "0".repeat(difficulty);
        let mut block_hash = self.clone().hash.expect("No hash found for the block");

        while block_hash[..difficulty] != target {
            self.nonce += 1;
            block_hash = self.block_hasher()
        }

        self.hash = Some(block_hash.clone());

        log::info!("Block {} mined: {}", self.index, block_hash);

        self.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    // the public key of the sender
    pub sender_public_key: String,

    // the public key of the recipient
    pub receiver_public_key: String,

    // the amount of money being sent
    pub amount: u32,

    // the amount of commission the miner will get
    pub transaction_fee: u32,

    // the time when this transaction occurred(vague in terms of whether it is the start or end
    // time) TBD
    pub timestamp: DateTime<Utc>,
}
