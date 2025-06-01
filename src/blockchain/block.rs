use chrono::{TimeDelta, Utc};
use std::{collections::LinkedList, usize};

use crate::{
    types::{
        args::Args,
        block::{Block, Transaction},
    },
    utils::hasher::{block_hasher, transactions_hasher},
};

use super::miner::mine_blocks;

pub struct Blockchain {
    // List of peers/blocks in the chain
    pub blocks: LinkedList<Block>,

    // The list of transactions which are to be added in the next block to be mined
    // It's a mempool of transactions
    pub current_transactions: Vec<Transaction>,

    // Archived transactions, similar to a log/ledger for transactions
    pub archived_transactions: Vec<Transaction>,

    // reference for the last block which was added to the chain
    pub(crate) last_block: Option<Block>,

    // Difficulty level for mining the block
    pub(crate) difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: LinkedList::new(),
            current_transactions: Vec::new(),
            archived_transactions: Vec::new(),
            last_block: None,
            difficulty: 0,
        }
    }

    // Genesis Block: First Block of the Chain
    pub fn init(&mut self) -> Blockchain {
        let genesis_block = Block {
            index: 0,
            prev_hash: "0".to_string(),
            nonce: 0,
            timestamp: Utc::now(),
            transactions: Vec::new(),
            merkle_root: String::new(),
        };

        self.blocks.push_back(genesis_block);

        Blockchain {
            blocks: self.blocks.clone(),
            current_transactions: Vec::new(),
            archived_transactions: Vec::new(),
            last_block: None,
            // Default difficulty is set to 3
            difficulty: 3,
        }
    }

    pub fn add_new_block(&mut self, args: Args) {
        let mut block = Block {
            index: self.last_block.clone().unwrap().index + 1,
            prev_hash: block_hasher(self.last_block.clone().unwrap().clone()),
            nonce: 0,
            timestamp: Utc::now(),
            transactions: self.current_transactions.clone(),
            merkle_root: transactions_hasher(self.current_transactions.clone()),
        };

        if self.last_block.clone().unwrap().index % 10 == 9 {
            self.adjust_difficulty();
        }

        mine_blocks(&mut block, self.difficulty);
    }

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

    pub fn get_last_block(self) -> Option<Block> {
        if self.last_block.is_some() {
            self.last_block
        } else {
            log::error!("Genesis block doesn't have a previous block!");
            None
        }
    }

    // For now we are checking at the mining time of last 10 blocks
    pub fn adjust_difficulty(&mut self) {
        let n = 10;
        // for now I am setting the time per block to 5 minutes
        let target_time_per_block = 30;
        let tot_time: TimeDelta = self
            .blocks
            .iter()
            .rev()
            .take(n)
            .map(|block| block.timestamp)
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| w[0] - w[1])
            .sum();
        let total_time = tot_time.as_seconds_f64() as usize;
        let expected_time = target_time_per_block * n;

        if total_time < expected_time / 2 {
            self.difficulty += 1;
        } else if total_time > expected_time * 2 {
            self.difficulty -= 1;
        }
    }
}
