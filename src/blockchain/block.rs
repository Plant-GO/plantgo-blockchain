use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::LinkedList, env, usize};

use crate::{
    types::{
        args::Args,
        block::{Block, Transaction},
    },
    utils::hasher::{block_hasher, transactions_hasher},
};

use super::miner::mine_blocks;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain {
    // List of peers/blocks in the chain
    pub blocks: LinkedList<Block>,

    // The list of transactions which are to be added in the next block to be mined
    // It's a mempool of transactions
    pub current_transactions: Vec<Transaction>,

    // Archived transactions, similar to a log/ledger for transactions
    pub archived_transactions: Vec<Vec<Transaction>>,

    // Difficulty level for mining the block
    pub(crate) difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: LinkedList::new(),
            current_transactions: Vec::new(),
            archived_transactions: Vec::new(),
            difficulty: 0,
        }
    }

    // Genesis Block: First Block of the Chain
    pub fn init(&mut self) -> Blockchain {
        let mut genesis_block = Block {
            index: 0,
            prev_hash: "0".to_string(),
            hash: None,
            nonce: 0,
            timestamp: Utc::now(),
            transactions: Vec::new(),
            merkle_root: String::new(),
        };
        genesis_block.hash = Some(genesis_block.block_hasher());

        log::info!("Creating Genesis Block!!!");
        self.blocks.push_back(genesis_block);

        Blockchain {
            blocks: self.blocks.clone(),
            current_transactions: Vec::new(),
            archived_transactions: Vec::new(),
            // Default difficulty is set to 3
            difficulty: 3,
        }
    }

    pub fn add_new_block(&mut self) {
        println!("Blockchain: {:?}\n", self);
        println!("Last Block: {:?}\n", self.clone().get_last_block());

        let mut block = Block {
            index: self.clone().get_last_block().unwrap().index + 1,
            prev_hash: block_hasher(self.clone().get_last_block().unwrap().clone()),
            hash: None,
            nonce: 0,
            timestamp: Utc::now(),
            transactions: self.current_transactions.clone(),
            merkle_root: transactions_hasher(self.current_transactions.clone()),
        };
        block.hash = Some(block.block_hasher());

        // it checks whether to adjust the difficulty or not after every 10 blocks
        if self.clone().get_last_block().unwrap().index
            % env::var("NUMBER_OF_BLOCKS_GROUPED")
                .unwrap_or_else(|_| "10".to_string())
                .parse::<u32>()
                .expect("Invalid Number of blocks")
            == env::var("REMAINDER")
                .unwrap_or_else(|_| "9".to_string())
                .parse::<u32>()
                .expect("Invalid REMAINDER type")
        {
            self.adjust_difficulty();
        }

        self.blocks.push_back(block);
        self.archived_transactions
            .push(self.current_transactions.clone());
        self.current_transactions.clear();
        // mine_blocks(&mut block, self.difficulty);
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

    // pub fn get_mempool(self) -> Vec<Transaction> {
    //     self.current_transactions
    // }

    pub fn get_last_block(self) -> Option<Block> {
        if !self.blocks.is_empty() {
            self.blocks.back().cloned()
        } else {
            None
        }
    }

    // For now we are checking at the mining time of last 10 blocks
    pub fn adjust_difficulty(&mut self) {
        let n = 10;
        let window_size = env::var("WINDOW_SIZE")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<usize>()
            .expect("Invalid Window size");
        // for now I am setting the time per block to 5 minutes
        let target_time_per_block = 30;
        let tot_time: TimeDelta = self
            .blocks
            .iter()
            .rev()
            .take(n)
            .map(|block| block.timestamp)
            .collect::<Vec<_>>()
            .windows(window_size)
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

    pub fn proof_of_work(self) {
        todo!()
    }

    pub fn valid_hash(self) {
        todo!()
    }
}
