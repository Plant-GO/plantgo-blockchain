use crate::{types::block::Block, utils::hasher::block_hasher};

pub fn mine_blocks(block: &mut Block, difficulty: usize) {
    let target = "0".repeat(difficulty);
    let mut block_hash = block.block_hasher();

    // First "difficulty" number or bytes are taken
    while block_hash[..difficulty] != target {
        block.nonce += 1;
        block_hash = block_hasher(block.clone());
    }

    log::info!("Block {} mined: {}", block.index, block_hash);
}
