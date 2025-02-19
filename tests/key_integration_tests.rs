use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;
use mallowbtc::GiftKeys;

/// Test basic key aggregation with direct public keys
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

/// Test key aggregation workflow with derived keys
#[test]
fn test_key_aggregation_workflow() {
    const GIVER_TPUB: &str = "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6";
    const RECEIVER_TPUB: &str = "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6";

    let gift_keys = GiftKeys::from_tpubs(GIVER_TPUB, RECEIVER_TPUB)
        .expect("Should create from tpubs");

    let agg = gift_keys.aggregate_musig2_key()
        .expect("Aggregation should succeed");

    println!("\n=== Derived Key Workflow ===");
    println!("Giver Tpub: {}", GIVER_TPUB);
    println!("Receiver Tpub: {}", RECEIVER_TPUB);
    println!("Aggregated Key: {}", agg);

    assert!(!agg.to_string().is_empty(), "Should produce non-empty aggregated key");
    assert_eq!(agg.to_string().len(), 64, "Should be 32-byte hex string");
}