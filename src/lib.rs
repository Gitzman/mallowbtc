pub mod error;
pub mod keys;
pub mod script;
pub mod test_harness;

pub use error::Error;
pub use keys::GiftKeys;
pub use script::GiftScript;
pub use test_harness::TestHarness;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TPUB: &str = "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6";

    #[test]
    fn test_key_derivation() {
        let giver_tpub = test_harness::extract_tpub_from_descriptor(GIVER_DESC).unwrap();
        let receiver_tpub = test_harness::extract_tpub_from_descriptor(RECEIVER_DESC).unwrap();

        // Test key derivation with test vectors
        let mut keys = GiftKeys::new(&giver_tpub).unwrap();
        let pubkey = keys.derive_giver_pubkey().unwrap();
        
        println!("\n=== Derived Keys ===");
        println!("Giver Public Key (hex): {}", hex::encode(pubkey.serialize()));

        // Add receiver and test both keys
        keys.add_receiver(&receiver_tpub).unwrap();
        
        let giver_key = keys.derive_giver_pubkey().unwrap();
        let receiver_key = keys.derive_receiver_pubkey().unwrap();
        
        println!("\nGiver Key: {}", hex::encode(giver_key.serialize()));
        println!("Receiver Key: {}", hex::encode(receiver_key.serialize()));
        
        assert!(hex::encode(giver_key.serialize()) != hex::encode(receiver_key.serialize()));
    }

    #[test]
    fn test_timelock_and_taproot() {
        let script = GiftScript::new(52560); // ~1 year in blocks
        let giver_tpub = test_harness::extract_tpub_from_descriptor(GIVER_DESC).unwrap();
        let receiver_tpub = test_harness::extract_tpub_from_descriptor(RECEIVER_DESC).unwrap();

        let mut keys = GiftKeys::new(&giver_tpub).unwrap();
        keys.add_receiver(&receiver_tpub).unwrap();

        // Create the timelock script
        let receiver_key = keys.derive_receiver_pubkey().unwrap();
        let timelock_script = script.create_timelock_script(receiver_key).unwrap();
        
        println!("\n=== Scripts ===");
        println!("Timelock Script:");
        println!("Hex: {}", hex::encode(timelock_script.as_bytes()));
        println!("ASM: {}", timelock_script.to_asm_string());

        // Create the taproot tree
        let p2tr_script = script.create_taproot_tree(&keys).unwrap();
        let address = bitcoin::Address::from_script(&p2tr_script, bitcoin::Network::Regtest).unwrap();
        
        println!("\nTaproot Output:");
        println!("Script (hex): {}", hex::encode(p2tr_script.as_bytes()));
        println!("Script (asm): {}", p2tr_script.to_asm_string());
        println!("Address: {}", address);
        
        assert!(address.to_string().starts_with("bcrt1"));
    }

    #[test]
    fn test_harness_addresses() {
        let harness = TestHarness::setup().unwrap();
        
        let giver_addr = harness.get_address(&harness.giver_descriptor).unwrap();
        let receiver_addr = harness.get_address(&harness.receiver_descriptor).unwrap();
        
        println!("\n=== Test Addresses ===");
        println!("Giver Address: {}", giver_addr);
        println!("Receiver Address: {}", receiver_addr);
        println!("\nDescriptors:");
        println!("Giver: {}", harness.giver_descriptor);
        println!("Receiver: {}", harness.receiver_descriptor);
        
        assert!(giver_addr.to_string().starts_with("bcrt1"));
        assert!(receiver_addr.to_string().starts_with("bcrt1"));
    }

    const GIVER_DESC: &str = "tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)";
    const RECEIVER_DESC: &str = "tr([f8e65a0b/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/1/*)";
}