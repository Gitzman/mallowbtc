use bitcoin::XOnlyPublicKey;
use bitcoin::secp256k1::PublicKey;
use musig2::KeyAggContext;
use crate::Error;

/// GiftKeys holds the public keys for the giver and receiver.
#[derive(Debug, Clone)]
pub struct GiftKeys {
    pub giver: PublicKey,
    pub receiver: PublicKey,
}

impl GiftKeys {
    /// Create a new GiftKeys instance.
    pub fn new(giver: PublicKey, receiver: PublicKey) -> Self {
        GiftKeys { giver, receiver }
    }

    /// Derives the giver's public key.
    pub fn derive_giver_pubkey(&self) -> Result<PublicKey, Error> {
        Ok(self.giver)
    }

    /// Derives the receiver's public key.
    pub fn derive_receiver_pubkey(&self) -> Result<PublicKey, Error> {
        Ok(self.receiver)
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
    use std::str::FromStr;

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
}