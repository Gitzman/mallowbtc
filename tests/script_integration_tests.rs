use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;
use mallowbtc::{GiftKeys, GiftScript};

/// Tests the complete script creation workflow including timelock and taproot construction
#[test]
fn test_script_creation_workflow() {
    // Initialize with ~1 year timelock
    let script = GiftScript::new(52560);

    // Use known test vectors for keys
    let giver = PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
        .expect("valid giver key");
    let receiver = PublicKey::from_str("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
        .expect("valid receiver key");

    let gift_keys = GiftKeys::new(giver, receiver);

    // Create and verify the timelock script
    let timelock_script = script.create_timelock_script(gift_keys.receiver)
        .expect("Should create timelock script");
    
    println!("\n=== Script Creation ===");
    println!("Timelock Script:");
    println!("Hex: {}", hex::encode(timelock_script.as_bytes()));
    println!("ASM: {}", timelock_script.to_asm_string());

    // Create and verify the complete taproot output
    let p2tr_script = script.create_taproot_tree(&gift_keys)
        .expect("Should create taproot tree");
    let address = bitcoin::Address::from_script(&p2tr_script, bitcoin::Network::Regtest)
        .expect("Should create valid address");
    
    println!("\nTaproot Output:");
    println!("Script (hex): {}", hex::encode(p2tr_script.as_bytes()));
    println!("Script (asm): {}", p2tr_script.to_asm_string());
    println!("Address: {}", address);
    
    // Basic sanity checks
    assert!(address.to_string().starts_with("bcrt1"));
    assert!(timelock_script.to_asm_string().contains("OP_CSV"));
}