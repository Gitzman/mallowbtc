use bitcoin::XOnlyPublicKey;
use bitcoin::secp256k1::PublicKey;
use miniscript::descriptor::DescriptorPublicKey;
use std::str::FromStr;
use crate::Error;

/// GiftKeys holds the public keys for the giver and receiver.
#[derive(Debug, Clone)]
pub struct GiftKeys {
    pub giver: DescriptorPublicKey,
    pub receiver: DescriptorPublicKey,
}

impl GiftKeys {
    /// Create a new GiftKeys instance from raw public keys.
    /// This method is deprecated in favor of descriptor-based initialization.
    pub fn new(_giver: PublicKey, _receiver: PublicKey) -> Self {
        // Not implemented directly - we're using descriptor-based keys
        unimplemented!("Direct PublicKey initialization is no longer supported. Use from_descriptor_strings instead.")
    }

    /// Creates a new GiftKeys from two descriptor strings.
    /// 
    /// # Arguments
    /// 
    /// * `giver_desc` - The descriptor string for the giver's key
    /// * `receiver_desc` - The descriptor string for the receiver's key
    /// 
    /// # Returns
    /// 
    /// Result containing GiftKeys or an Error
    pub fn from_descriptor_strings(giver_desc: &str, receiver_desc: &str) -> Result<Self, Error> {
        let giver = DescriptorPublicKey::from_str(giver_desc)
            .map_err(|e| Error::KeyError(format!("Invalid giver descriptor: {}", e)))?;
        
        let receiver = DescriptorPublicKey::from_str(receiver_desc)
            .map_err(|e| Error::KeyError(format!("Invalid receiver descriptor: {}", e)))?;
        
        Ok(Self { giver, receiver })
    }
    
    /// Returns the x-only public key for the giver.
    ///
    /// # Returns
    /// 
    /// XOnlyPublicKey for the giver or Error if the key is not an XPub.
    pub fn giver_x_only_pub(&self) -> Result<XOnlyPublicKey, Error> {
        match &self.giver {
            DescriptorPublicKey::XPub(xpub) => Ok(xpub.xkey.to_x_only_pub()),
            _ => Err(Error::KeyError("Giver key is not an XPub type".to_string()))
        }
    }
    
    /// Returns the x-only public key for the receiver.
    ///
    /// # Returns
    /// 
    /// XOnlyPublicKey for the receiver or Error if the key is not an XPub.
    pub fn receiver_x_only_pub(&self) -> Result<XOnlyPublicKey, Error> {
        match &self.receiver {
            DescriptorPublicKey::XPub(xpub) => Ok(xpub.xkey.to_x_only_pub()),
            _ => Err(Error::KeyError("Receiver key is not an XPub type".to_string()))
        }
    }
    
    /// For backwards compatibility: create from descriptors
    pub fn from_descriptors(giver_desc: &str, receiver_desc: &str) -> Result<Self, Error> {
        Self::from_descriptor_strings(giver_desc, receiver_desc)
    }

    /// For backwards compatibility: create from tpubs 
    pub fn from_tpubs(giver_tpub: &str, receiver_tpub: &str) -> Result<Self, Error> {
        // Check if the inputs are in the annotated format
        let giver_desc = if giver_tpub.starts_with("[") {
            giver_tpub.to_string()
        } else {
            format!("[73c5da0a/86'/1'/0']{}/0/*", giver_tpub)
        };
        
        let receiver_desc = if receiver_tpub.starts_with("[") {
            receiver_tpub.to_string()
        } else {
            format!("[f8e65a0b/86'/1'/0']{}/0/*", receiver_tpub)
        };

        Self::from_descriptor_strings(&giver_desc, &receiver_desc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_descriptors() -> Result<(), ()> {
        const GIVER_DESC: &str = "[73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*";
        const RECEIVER_DESC: &str = "[143df5a6/86'/1'/1']tpubDCvNAJkUmvjcbaAF57Yp5v53rMMxVo34KYLMjmj6xcdo9r2rf3CkZoGHswZTtA2H6pXJsavhRpeqkwnDs6bSLsHbdycfLJN3N5J4nQP1Kuc/0/*";
        
        let gift_keys = GiftKeys::from_descriptor_strings(GIVER_DESC, RECEIVER_DESC)
            .map_err(|_| ())?;
        
        // Test that we can get the x-only public keys
        let giver_xonly = gift_keys.giver_x_only_pub().map_err(|_| ())?;
        let receiver_xonly = gift_keys.receiver_x_only_pub().map_err(|_| ())?;
        
        // Simply verify we got keys with the right format
        assert_eq!(giver_xonly.to_string().len(), 64, "Should be a 32-byte hex string");
        assert_eq!(receiver_xonly.to_string().len(), 64, "Should be a 32-byte hex string");
        
        Ok(())
    }
}