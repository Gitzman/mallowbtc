use bitcoin::XOnlyPublicKey;
use bitcoin::secp256k1::PublicKey;
use musig2::KeyAggContext;
use bdk_wallet::{
    descriptor::Descriptor,
    descriptor::DescriptorPublicKey,
};
use std::str::FromStr;
use crate::Error;

/// GiftKeys holds the public keys for the giver and receiver.
#[derive(Debug, Clone)]
pub struct GiftKeys {
    pub giver: PublicKey,
    pub receiver: PublicKey,
}

impl GiftKeys {
    /// Create a new GiftKeys instance from raw public keys.
    pub fn new(giver: PublicKey, receiver: PublicKey) -> Self {
        GiftKeys { giver, receiver }
    }

    /// Create a new GiftKeys instance from descriptor strings
    pub fn from_descriptors(giver_desc: &str, receiver_desc: &str) -> Result<Self, Error> {
        // Parse descriptors
        let giver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(giver_desc)
            .map_err(|e| Error::KeyError(format!("Failed to parse giver descriptor: {}", e)))?;
            
        let receiver_descriptor = Descriptor::<DescriptorPublicKey>::from_str(receiver_desc)
            .map_err(|e| Error::KeyError(format!("Failed to parse receiver descriptor: {}", e)))?;

        // Use index 0 for both keys
        let giver_derived = giver_descriptor.at_derivation_index(0)
            .map_err(|e| Error::KeyError(format!("Failed to derive giver key: {}", e)))?;

        let receiver_derived = receiver_descriptor.at_derivation_index(0)
            .map_err(|e| Error::KeyError(format!("Failed to derive receiver key: {}", e)))?;

        // Extract x-only public keys from derived scripts
        let giver_script = giver_derived.script_pubkey();
        let receiver_script = receiver_derived.script_pubkey();

        // Get the keys from the script (they're the second item in P2TR scripts)
        let giver_ins = giver_script.as_script().instructions().nth(1)
            .ok_or_else(|| Error::KeyError("No key in giver script".to_string()))?
            .map_err(|e| Error::KeyError(format!("Invalid giver script: {}", e)))?;
        let giver_bytes = giver_ins.push_bytes()
            .ok_or_else(|| Error::KeyError("No push bytes in giver script".to_string()))?;

        let receiver_ins = receiver_script.as_script().instructions().nth(1)
            .ok_or_else(|| Error::KeyError("No key in receiver script".to_string()))?
            .map_err(|e| Error::KeyError(format!("Invalid receiver script: {}", e)))?;
        let receiver_bytes = receiver_ins.push_bytes()
            .ok_or_else(|| Error::KeyError("No push bytes in receiver script".to_string()))?;

        // Convert x-only keys to full public keys
        let giver_xonly = XOnlyPublicKey::from_slice(giver_bytes.as_bytes())
            .map_err(|e| Error::KeyError(format!("Invalid giver key bytes: {}", e)))?;
        let receiver_xonly = XOnlyPublicKey::from_slice(receiver_bytes.as_bytes())
            .map_err(|e| Error::KeyError(format!("Invalid receiver key bytes: {}", e)))?;

        Ok(GiftKeys {
            giver: giver_xonly.public_key(bitcoin::secp256k1::Parity::Even),
            receiver: receiver_xonly.public_key(bitcoin::secp256k1::Parity::Even),
        })
    }

    /// Create a new GiftKeys instance from tpub strings
    pub fn from_tpubs(giver_tpub: &str, receiver_tpub: &str) -> Result<Self, Error> {
        // Convert tpubs to descriptors
        let giver_desc = format!("tr([73c5da0a/86'/1'/0']{}/0/*)", giver_tpub);
        let receiver_desc = format!("tr([f8e65a0b/86'/1'/0']{}/1/*)", receiver_tpub);

        Self::from_descriptors(&giver_desc, &receiver_desc)
    }

    /// Create aggregated MuSig2 key from giver and receiver keys.
    pub fn aggregate_musig2_key(&self) -> Result<XOnlyPublicKey, Error> {
        // Convert bitcoin::PublicKey directly to musig2::secp256k1::PublicKey
        let giver_musig = musig2::secp256k1::PublicKey::from_slice(&self.giver.serialize())
            .map_err(|e| Error::KeyError(format!("Failed to convert giver key for MuSig2: {}", e)))?;
        let receiver_musig = musig2::secp256k1::PublicKey::from_slice(&self.receiver.serialize())
            .map_err(|e| Error::KeyError(format!("Failed to convert receiver key for MuSig2: {}", e)))?;
        
        // Aggregate the public keys
        let pubkeys = vec![giver_musig, receiver_musig];
        let context = KeyAggContext::new(pubkeys)
            .map_err(|e| Error::KeyError(format!("Failed to create MuSig2 context: {}", e)))?;

        // Get the aggregated key
        let agg_musig: musig2::secp256k1::XOnlyPublicKey = context.aggregated_pubkey();
        
        // Convert back to bitcoin's XOnlyPublicKey
        XOnlyPublicKey::from_slice(&agg_musig.serialize())
            .map_err(|e| Error::KeyError(format!("Failed to convert aggregated key: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_creation() {
        let pk1 = PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
            .expect("valid test key 1");
        let pk2 = PublicKey::from_str("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
            .expect("valid test key 2");

        let keys = GiftKeys::new(pk1, pk2);
        assert_eq!(keys.giver, pk1);
        assert_eq!(keys.receiver, pk2);
    }

    #[test]
    fn test_musig2_aggregation() {
        let pk1 = PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
            .expect("valid test key 1");
        let pk2 = PublicKey::from_str("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
            .expect("valid test key 2");

        let gift_keys = GiftKeys::new(pk1, pk2);
        let agg = gift_keys.aggregate_musig2_key().expect("Aggregation should succeed");
        
        // Known good aggregate key for these inputs
        assert_eq!(
            agg.to_string(),
            "3b46d262d2f610e9038b44beabdfe97ab5a0feb89870acc2264edfb7f63ec2ec",
            "Aggregated key should match expected value"
        );
    }

    #[test]
    fn test_from_descriptors() {
        const GIVER_DESC: &str = "tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)";
        const RECEIVER_DESC: &str = "tr([f8e65a0b/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/1/*)";

        let gift_keys = GiftKeys::from_descriptors(GIVER_DESC, RECEIVER_DESC)
            .expect("Should create from descriptors");
        
        let agg = gift_keys.aggregate_musig2_key()
            .expect("Should aggregate derived keys");

        // Verify the key format
        assert_eq!(agg.to_string().len(), 64, "Should be 32-byte hex string");
    }

    #[test]
    fn test_from_tpubs() {
        const GIVER_TPUB: &str = "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6";
        const RECEIVER_TPUB: &str = "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6";

        let gift_keys = GiftKeys::from_tpubs(GIVER_TPUB, RECEIVER_TPUB)
            .expect("Should create from tpubs");
        
        let agg = gift_keys.aggregate_musig2_key()
            .expect("Should aggregate derived keys");

        // Verify the key format
        assert_eq!(agg.to_string().len(), 64, "Should be 32-byte hex string");
    }
}