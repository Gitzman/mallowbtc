use bitcoin::secp256k1::{Secp256k1, PublicKey};
use bitcoin::taproot::TaprootBuilder;
use bitcoin::ScriptBuf;
use miniscript::{Miniscript, Tap};
use crate::error::Error;
use crate::keys::GiftKeys;

pub struct GiftScript {
    pub(crate) timelock_blocks: u32,
}

impl GiftScript {
    pub fn new(timelock_blocks: u32) -> Self {
        Self { timelock_blocks }
    }

    pub fn create_taproot_tree(&self, keys: &GiftKeys) -> Result<ScriptBuf, Error> {
        let secp = Secp256k1::new();
        
        // Get receiver's key for the timelock branch
        let receiver_pubkey = keys.derive_receiver_pubkey()?;
        
        // Create the timelock script
        let timelock_script = self.create_timelock_script(receiver_pubkey)?;

        // Get aggregated MuSig2 key for internal key
        let internal_key = keys.aggregate_musig2_key()?;
        
        // Build taproot tree with single leaf
        let tree = TaprootBuilder::new()
            .add_leaf(0, timelock_script.clone())
            .map_err(|e| Error::ScriptError(format!("Failed to add leaf: {:?}", e)))?
            .finalize(&secp, internal_key)
            .map_err(|e| Error::ScriptError(format!("Failed to build taproot tree: {:?}", e)))?;

        // Create P2TR output script
        Ok(ScriptBuf::new_p2tr(
            &secp,
            internal_key,
            tree.merkle_root()
        ))
    }

    /// Create the timelock script using miniscript
    pub fn create_timelock_script(&self, receiver_key: PublicKey) -> Result<ScriptBuf, Error> {
        let script = format!(
            "and_v(v:pk({}),older({}))",
            receiver_key,
            self.timelock_blocks
        );

        let ms = Miniscript::<PublicKey, Tap>::from_str_insane(&script)
            .map_err(|e| Error::ScriptError(format!("Invalid miniscript: {}", e)))?;

        Ok(ms.encode())
    }
}