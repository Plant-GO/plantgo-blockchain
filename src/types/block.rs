use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Block {
    prev_hash: Option<Box<Block>>,
    nonce: u32,
    timestamp: DateTime<Utc>,
    transactions: Transactions,
    merkle_root: String,
}

#[derive(Deserialize, Serialize)]
pub struct Transactions {
    sender: Box<Block>,
    receiver: Box<Block>,
    amount: u32,
    transaction_fee: u32,
    timestamp: DateTime<Utc>,
}

// pub impl Transactions {
//     f
// }
