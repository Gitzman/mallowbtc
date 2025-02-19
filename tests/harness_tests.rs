use mallowbtc::TestHarness;

#[test]
fn test_harness_workflow() {
    let harness = TestHarness::setup().expect("Should set up test harness");
    
    // Test descriptor-based address generation
    let giver_addr = harness.get_address(&harness.giver_descriptor)
        .expect("Should get giver address");
    let receiver_addr = harness.get_address(&harness.receiver_descriptor)
        .expect("Should get receiver address");
    
    println!("\n=== Test Environment ===");
    println!("Giver Address: {}", giver_addr);
    println!("Receiver Address: {}", receiver_addr);
    println!("\nDescriptors:");
    println!("Giver: {}", harness.giver_descriptor);
    println!("Receiver: {}", harness.receiver_descriptor);
    
    // Verify we're getting valid regtest addresses
    assert!(giver_addr.to_string().starts_with("bcrt1"));
    assert!(receiver_addr.to_string().starts_with("bcrt1"));
    assert!(giver_addr.to_string() != receiver_addr.to_string());
}