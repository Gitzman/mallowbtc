use bdk_wallet::{
    bitcoin::secp256k1::PublicKey,
    keys::DescriptorPublicKey,
    descriptor::Descriptor,
};
use miniscript::ToPublicKey;
use std::str::FromStr;
use crate::error::Error;

#[derive(Clone)]
pub struct GiftKeys {
    giver_descriptor: Descriptor<DescriptorPublicKey>,
    receiver_descriptor: Option<Descriptor<DescriptorPublicKey>>,
}

impl GiftKeys {
    /// Create a new GiftKeys instance with a giver's extended public key
    pub fn new(giver_tpub: &str) -> Result<Self, Error> {
        // Create giver descriptor with derivation path /0/0
        let giver_desc_str = format!("tr([73c5da0a/86'/1'/0']{}/0/*)", giver_tpub);
        let giver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(&giver_desc_str)
            .map_err(|e| Error::KeyError(format!("Invalid giver descriptor: {}", e)))?;
        
        Ok(Self {
            giver_descriptor,
            receiver_descriptor: None,
        })
    }
    
    /// Add receiver's extended public key
    pub fn add_receiver(&mut self, receiver_tpub: &str) -> Result<(), Error> {
        // Create receiver descriptor with different derivation path /1/0
        let receiver_desc_str = format!("tr([f8e65a0b/86'/1'/0']{}/1/*)", receiver_tpub);
        let receiver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(&receiver_desc_str)
            .map_err(|e| Error::KeyError(format!("Invalid receiver descriptor: {}", e)))?;
        
        self.receiver_descriptor = Some(receiver_descriptor);
        Ok(())
    }
    
    /// Get giver's descriptor
    pub fn giver_descriptor(&self) -> &Descriptor<DescriptorPublicKey> {
        &self.giver_descriptor
    }
    
    /// Get receiver's descriptor if set
    pub fn receiver_descriptor(&self) -> Option<&Descriptor<DescriptorPublicKey>> {
        self.receiver_descriptor.as_ref()
    }

    /// Get the next derived public key for the giver
    pub fn derive_giver_pubkey(&self) -> Result<PublicKey, Error> {
        self.derive_pubkey_from_descriptor(&self.giver_descriptor, 0)
    }

    /// Get the next derived public key for the receiver
    pub fn derive_receiver_pubkey(&self) -> Result<PublicKey, Error> {
        let descriptor = self.receiver_descriptor
            .as_ref()
            .ok_or_else(|| Error::KeyError("Receiver not set".to_string()))?;
        
        self.derive_pubkey_from_descriptor(descriptor, 1)
    }

    /// Derive a public key from a descriptor at the specified index
    fn derive_pubkey_from_descriptor(&self, descriptor: &Descriptor<DescriptorPublicKey>, index: u32) -> Result<PublicKey, Error> {
        // Get the derived script
        let derived = descriptor
            .at_derivation_index(index)
            .map_err(|e| Error::KeyError(format!("Key derivation failed: {}", e)))?;

        // For taproot descriptors, we can extract the key from the policy
        match &derived {
            Descriptor::Tr(tr) => {
                // Get the internal key and convert to PublicKey
                let key = tr.internal_key();
                let pk = key.to_public_key();
                PublicKey::from_slice(&pk.to_bytes())
                    .map_err(|e| Error::KeyError(format!("Failed to convert key: {}", e)))
            },
            _ => Err(Error::KeyError("Not a taproot descriptor".to_string()))
        }
    }
}