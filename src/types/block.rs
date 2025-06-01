use std::sync::mpsc::{Receiver, Sender};

use chrono::{DateTime, Utc};
use clap::builder::Str;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Block {
    // Self explanatory
    pub index: u32,

    // hash of last block in order to prevent tampering in the previous block
    pub prev_hash: String,

    // random number used for mining
    pub nonce: u32,

    // the time when the block was created
    pub timestamp: DateTime<Utc>,

    // the group of transactions take for this block from mempool
    pub transactions: Vec<Transaction>,

    // the hash of all the transactions in the block
    pub merkle_root: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    // the address of the sender
    pub sender: String,

    // the address of the recipient
    pub receiver: String,

    // the amount of money being sent
    pub amount: u32,

    // the amount of commission the miner will get
    pub transaction_fee: u32,

    // the time when this transaction occurred(vague in terms of whether it is the start or end
    // time) TBD
    pub timestamp: DateTime<Utc>,
}
