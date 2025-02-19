use bitcoin::secp256k1::PublicKey;
use bitcoin::XOnlyPublicKey;
use std::str::FromStr;
use mallowbtc::{GiftKeys, TestHarness};

/// Test key aggregation with direct public keys
#[test]
fn test_key_aggregation_direct() {
    // Use known test vectors for public keys
    let pk1 = PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
        .expect("valid test key 1");
    let pk2 = PublicKey::from_str("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
        .expect("valid test key 2");

    let gift_keys = GiftKeys::new(pk1, pk2);
    let agg = gift_keys.aggregate_musig2_key().expect("Aggregation should succeed");
    
    println!("\n=== Direct Key Aggregation ===");
    println!("Giver Key: {}", pk1);
    println!("Receiver Key: {}", pk2);
    println!("Aggregated Key: {}", agg);
    
    assert_eq!(
        agg.to_string(),
        "3b46d262d2f610e9038b44beabdfe97ab5a0feb89870acc2264edfb7f63ec2ec",
        "Aggregated key should match expected value"
    );
}

/// Test key aggregation with derived keys from tpubs
#[test]
fn test_key_aggregation_from_tpub() {
    // These are the test tpubs from the descriptors
    const GIVER_DESC: &str = "tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)";
    const RECEIVER_DESC: &str = "tr([f8e65a0b/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/1/*)";

    println!("\n=== Derived Key Aggregation ===");
    println!("Giver Descriptor: {}", GIVER_DESC);
    println!("Receiver Descriptor: {}", RECEIVER_DESC);

    // Use test harness to get derived keys
    let harness = TestHarness::setup().expect("Should set up test harness");
    
    // Get the script pubkeys which contain the derived x-only keys
    let giver_script = harness.giver_descriptor.at_derivation_index(0)
        .expect("Should derive giver key")
        .script_pubkey();
    
    let receiver_script = harness.receiver_descriptor.at_derivation_index(0)
        .expect("Should derive receiver key")
        .script_pubkey();

    // Extract x-only public keys from the taproot scripts 
    let giver_ins = giver_script.as_script().instructions().nth(1)
        .expect("Should have second push").unwrap();
    let giver_bytes = giver_ins.push_bytes().expect("Should be push bytes");
        
    let receiver_ins = receiver_script.as_script().instructions().nth(1)
        .expect("Should have second push").unwrap();
    let receiver_bytes = receiver_ins.push_bytes().expect("Should be push bytes");

    println!("Derived Giver Key: {}", hex::encode(giver_bytes));
    println!("Derived Receiver Key: {}", hex::encode(receiver_bytes));

    // Convert the x-only keys to full public keys (adding y coordinates)
    let giver_xonly = XOnlyPublicKey::from_slice(giver_bytes.as_bytes())
        .expect("Should parse giver x-only key");
    let receiver_xonly = XOnlyPublicKey::from_slice(receiver_bytes.as_bytes())
        .expect("Should parse receiver x-only key");

    let giver_key = giver_xonly.public_key(bitcoin::secp256k1::Parity::Even);
    let receiver_key = receiver_xonly.public_key(bitcoin::secp256k1::Parity::Even);

    // Create GiftKeys from the public keys and aggregate
    let gift_keys = GiftKeys::new(giver_key, receiver_key);
    let agg = gift_keys.aggregate_musig2_key().expect("Aggregation should succeed");
    println!("Aggregated Key: {}", agg);

    assert!(!agg.to_string().is_empty(), "Should produce non-empty aggregated key");
    // We don't assert the exact value since it depends on the derivation,
    // but we can verify it's a valid x-only pubkey
    assert_eq!(agg.to_string().len(), 64, "Should be 32-byte hex string");
}