use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Block {
    // hash of last block in order to prevent tampering in the previous block
    prev_hash: String,

    // random number used for mining
    nonce: u32,

    // the time when the block was created
    timestamp: DateTime<Utc>,

    // the group of transactions take for this block from mempool
    transactions: Transactions,

    // the hash of all the transactions in the block
    merkle_root: String,
}

#[derive(Deserialize, Serialize)]
pub struct Transactions {
    // the address of the sender
    sender: String,

    // the address of the recipient
    receiver: Box<Block>,

    // the amount of money being sent
    amount: u32,

    // the amount of commission the miner will get
    transaction_fee: u32,

    // the time when this transaction occurred(vague in terms of whether it is the start or end
    // time) TBD
    timestamp: DateTime<Utc>,
}
