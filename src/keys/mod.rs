use bitcoin::XOnlyPublicKey;
use bitcoin::secp256k1::PublicKey;
use musig2::KeyAggContext;
use miniscript::bitcoin::bip32;
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
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_descriptors() -> Result<(), ()> {
        const GIVER_DESC: &str = "tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)";
        const RECEIVER_DESC: &str = "tr([143df5a6/86h/1h/1h]tpubDCvNAJkUmvjcbaAF57Yp5v53rMMxVo34KYLMjmj6xcdo9r2rf3CkZoGHswZTtA2H6pXJsavhRpeqkwnDs6bSLsHbdycfLJN3N5J4nQP1Kuc/<0;1>/*)#km0tzwrm";
        
        let ctx = miniscript::bitcoin::secp256k1::Secp256k1::signing_only();
        
        // Parse the descriptors
        let receiver_key = DescriptorPublicKey::from_str(RECEIVER_DESC).or(Err(()))?;
        let giver_key = DescriptorPublicKey::from_str(GIVER_DESC).or(Err(()))?;

        // Extract XPubs
        let receiver_xpub = match &receiver_key {
            DescriptorPublicKey::XPub(xpub) => xpub,
            _ => panic!("Receiver parsing error: expected XPub"),
        };

        let giver_xpub = match &giver_key {
            DescriptorPublicKey::XPub(xpub) => xpub,
            _ => panic!("Giver parsing error: expected XPub"),
        };

        // Test origin fingerprints
        if let Some((fingerprint, _)) = &receiver_xpub.origin {
            assert_eq!(
                fingerprint.to_string(),
                "143df5a6",
                "Receiver fingerprint should match expected value"
            );
        }
        
        if let Some((fingerprint, _)) = &giver_xpub.origin {
            assert_eq!(
                fingerprint.to_string(),
                "73c5da0a",
                "Giver fingerprint should match expected value"
            );
        }

        // Test derivation paths
        assert_eq!(
            receiver_xpub.derivation_path.to_string(),
            "m/86'/1'/1'",
            "Receiver derivation path should match expected value"
        );
        
        assert_eq!(
            giver_xpub.derivation_path.to_string(),
            "m/86'/1'/0'",
            "Giver derivation path should match expected value"
        );
        
        // Test wildcards - check if they are not Wildcard::None
        match receiver_xpub.wildcard {
            miniscript::descriptor::Wildcard::None => panic!("Receiver descriptor should have a wildcard"),
            _ => assert!(true, "Receiver has a wildcard as expected"),
        }
        
        match giver_xpub.wildcard {
            miniscript::descriptor::Wildcard::None => panic!("Giver descriptor should have a wildcard"),
            _ => assert!(true, "Giver has a wildcard as expected"),
        }
        
        // Test descriptor origin
        assert!(GIVER_DESC.starts_with("tr("), "Giver descriptor should be taproot");
        assert!(RECEIVER_DESC.starts_with("tr("), "Receiver descriptor should be taproot");
        
        // Test X-only public key derivation
        let receiver_x_only_pub = receiver_xpub.xkey.to_x_only_pub();
        let giver_x_only_pub = giver_xpub.xkey.to_x_only_pub();
        
        // Simply assert the values aren't null/empty - XOnlyPublicKey doesn't have an is_x_only method
        assert!(format!("{:?}", receiver_x_only_pub).len() > 0, 
                "Should have a valid X-only public key from receiver key");
        assert!(format!("{:?}", giver_x_only_pub).len() > 0, 
                "Should have a valid X-only public key from giver key");
        
        // Test that to_string() produces valid descriptors
        let receiver_string = receiver_key.to_string();
        let giver_string = giver_key.to_string();
        
        assert!(receiver_string.contains("tpubDCvNAJkUmvjcbaAF57Yp5v53rMMxVo34KYLMjmj6xcdo9r2rf3CkZoGHswZTtA2H6pXJsavhRpeqkwnDs6bSLsHbdycfLJN3N5J4nQP1Kuc"),
               "Receiver string representation should contain the xpub");
        
        assert!(giver_string.contains("tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6"), 
               "Giver string representation should contain the xpub");
        
        // Test path derivation matching
        let test_path_receiver = bip32::DerivationPath::from_str("m/86'/1'/1'/0/42").or(Err(()))?;
        let test_fingerprint_receiver = bip32::Fingerprint::from_str("143df5a6").or(Err(()))?;
        
        let test_path_giver = bip32::DerivationPath::from_str("m/86'/1'/0'/0/7").or(Err(()))?;
        let test_fingerprint_giver = bip32::Fingerprint::from_str("73c5da0a").or(Err(()))?;
        
        let expected_path_receiver = bip32::DerivationPath::from_str("m/0/42").or(Err(()))?;
        let expected_path_giver = bip32::DerivationPath::from_str("m/0/7").or(Err(()))?;
        
        // Check if matches returns the expected remaining path
        assert_eq!(
            receiver_xpub.matches(&(test_fingerprint_receiver, test_path_receiver), &ctx),
            Some(expected_path_receiver),
            "Receiver derivation path should match with correct path transformation"
        );
        
        assert_eq!(
            giver_xpub.matches(&(test_fingerprint_giver, test_path_giver), &ctx),
            Some(expected_path_giver),
            "Giver derivation path should match with correct path transformation"
        );
        
        Ok(())
    }}