use base58::ToBase58;
use rayon::prelude::*;
use solana_sdk::signature::{Keypair, Signer};

fn find_keypair_with_prefix( num_threads: usize) -> Option<[u8; 64]> {
    // 并行计算，使用 rayon 库创建 num_threads 个线程
    let result = (0..num_threads).into_par_iter().find_map_any(|_| {
        // 在循环内，每个线程都将持续生成新的密钥对
        loop {
            let keypair = Keypair::new();
            let public_key = keypair.pubkey();
            let public_key_str = public_key.to_string();
            let first_char = public_key_str.chars().next().unwrap();
            // 比较前 4 个字符是否都相同
            if public_key_str[0..6]
                .chars()
                .eq(std::iter::repeat(first_char).take(6))
            {
                // 输出符合条件的公钥以及对应的密钥对
                println!("'{}': {}", public_key_str, keypair.to_bytes().to_base58());
            }
        }
    });
    result
}

fn main() {
    let num_threads = 32; // 使用32个线程来并行计算
    match find_keypair_with_prefix( num_threads) {
        Some(private_key) => {
            println!("Found matching keypair: {}", private_key.to_base58());
        }
        None => {
            println!("No matching keypair found.");
        }
    }
}
