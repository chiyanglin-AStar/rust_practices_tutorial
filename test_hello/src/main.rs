mod block;

fn main() {
    let block_header = block::BlockHeader::new();

    println!("Constructed Block Header:");
    println!("Version: {}", block_header.version);
    println!("Previous Block Hash: {:?}", block_header.prev_block_hash);
    println!("Merkle Root: {:?}", block_header.merkle_root);
    println!("Timestamp: {}", block_header.timestamp);
    println!("Difficulty Target: {}", block_header.difficulty_target);
    println!("Nonce: {}", block_header.nonce);
}