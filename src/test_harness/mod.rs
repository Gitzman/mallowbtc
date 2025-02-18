use bdk_wallet::{
    descriptor::Descriptor,
    descriptor::DescriptorPublicKey,
};
use bitcoin::{secp256k1::PublicKey, Network};
use std::str::FromStr;
use crate::keys::GiftKeys;
use crate::error::Error;

#[derive(Clone)]
pub struct TestHarness {
    pub giver_descriptor: Descriptor<DescriptorPublicKey>,
    pub receiver_descriptor: Descriptor<DescriptorPublicKey>,
    pub gift_keys: GiftKeys,
}

impl TestHarness {
    pub fn setup() -> Result<Self, Error> {
        // Generate deterministic test descriptors
        let giver_desc_str = "tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)";
        let receiver_desc_str = "tr([f8e65a0b/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/1/*)";
        
        // Parse descriptors
        let giver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(giver_desc_str)
            .map_err(|e| Error::KeyError(format!("Failed to parse giver descriptor: {}", e)))?;
            
        let receiver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(receiver_desc_str)
            .map_err(|e| Error::KeyError(format!("Failed to parse receiver descriptor: {}", e)))?;

        // Extract public keys for GiftKeys
        let giver_key = PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
            .map_err(|e| Error::KeyError(format!("Failed to parse giver key: {}", e)))?;
        let receiver_key = PublicKey::from_str("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
            .map_err(|e| Error::KeyError(format!("Failed to parse receiver key: {}", e)))?;
            
        let gift_keys = GiftKeys::new(giver_key, receiver_key);
            
        Ok(Self {
            giver_descriptor,
            receiver_descriptor,
            gift_keys,
        })
    }

    /// Get next address from a descriptor
    pub fn get_address(&self, descriptor: &Descriptor<DescriptorPublicKey>) -> Result<bitcoin::Address, Error> {
        let derived = descriptor
            .at_derivation_index(0)
            .map_err(|e| Error::KeyError(format!("Failed to derive address: {}", e)))?;
            
        let script = derived.script_pubkey();
        
        bitcoin::Address::from_script(&script, Network::Regtest)
            .map_err(|e| Error::KeyError(format!("Failed to create address: {}", e)))
    }
}

/// Helper to extract tpub from a descriptor string
pub fn extract_tpub_from_descriptor(desc: &str) -> Result<String, Error> {
    // Extract tpub from "tr([73c5da0a/86'/1'/0']tpubD.../0/*)"
    let start = desc.find("tpub").ok_or_else(|| Error::KeyError("No tpub found".to_string()))?;
    let end = desc[start..].find("/").ok_or_else(|| Error::KeyError("Invalid descriptor format".to_string()))?;
    Ok(desc[start..start+end].to_string())
}