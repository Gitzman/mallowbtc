use bdk_wallet::{
    descriptor::Descriptor,
    descriptor::DescriptorPublicKey,
};
use bitcoin::Network;
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
        // Our test descriptors
        let giver_desc = "tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)";
        let receiver_desc = "tr([f8e65a0b/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/1/*)";
        
        // Parse descriptors
        let giver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(giver_desc)
            .map_err(|e| Error::KeyError(format!("Failed to parse giver descriptor: {}", e)))?;
            
        let receiver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(receiver_desc)
            .map_err(|e| Error::KeyError(format!("Failed to parse receiver descriptor: {}", e)))?;

        // Create GiftKeys from descriptors
        let gift_keys = GiftKeys::from_descriptors(giver_desc, receiver_desc)?;
            
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