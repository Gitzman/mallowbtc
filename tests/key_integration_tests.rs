use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;
use mallowbtc::GiftKeys;

/// Tests focused on key derivation and aggregation in a full workflow
#[test]
fn test_key_aggregation_workflow() {
    // Use known test vectors for public keys
    let pk1 = PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
        .expect("valid test key 1");
    let pk2 = PublicKey::from_str("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
        .expect("valid test key 2");

    let gift_keys = GiftKeys::new(pk1, pk2);
    let agg = gift_keys.aggregate_musig2_key().expect("Aggregation should succeed");
    
    println!("\n=== Key Aggregation ===");
    println!("Giver Key: {}", pk1);
    println!("Receiver Key: {}", pk2);
    println!("Aggregated Key: {}", agg);
    
    assert_eq!(
        agg.to_string(),
        "3b46d262d2f610e9038b44beabdfe97ab5a0feb89870acc2264edfb7f63ec2ec",
        "Aggregated key should match expected value"
    );
}