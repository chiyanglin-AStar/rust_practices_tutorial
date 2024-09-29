// src/block.rs

use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub difficulty_target: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        // Generate random values for demonstration purposes
        let prev_block_hash = rng.gen::<[u8; 32]>();
        let merkle_root = rng.gen::<[u8; 32]>();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
        let difficulty_target = 0xFFFFFFF; // Placeholder value
        let nonce = 0; // Placeholder value

        BlockHeader {
            version: 1,
            prev_block_hash,
            merkle_root,
            timestamp,
            difficulty_target,
            nonce,
        }
    }
}
