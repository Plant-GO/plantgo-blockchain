use chrono::Utc;
use std::collections::LinkedList;

use crate::types::block::{Block, Transaction};

pub struct Blockchain {
    // List of peers/blocks in the chain
    pub blocks: LinkedList<Block>,

    // The list of transactions which are to be added in the next block to be mined
    // It's a mempool of transactions
    pub current_transactions: Vec<Transaction>,

    // reference for the last block which was added to the chain
    pub(crate) last_block: Block,
}

impl Blockchain {
    pub fn set_new_transaction(
        &mut self,
        sender: String,
        receiver: String,
        amount: u32,
        transaction_fee: u32,
    ) -> Transaction {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
            transaction_fee,
            timestamp: Utc::now(),
        };

        self.current_transactions.push(transaction.clone());
        transaction
    }

    pub fn get_mempool(self) -> Vec<Transaction> {
        self.current_transactions
    }
}
