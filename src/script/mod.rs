use bitcoin::ScriptBuf;
use bitcoin::secp256k1::{PublicKey, Secp256k1};
use bitcoin::taproot::TaprootBuilder;
use bitcoin::Address;
use bitcoin::Network;
use crate::keys::GiftKeys;
use crate::Error;

pub struct GiftScript {
    timelock_blocks: u32,
}

impl GiftScript {
    pub fn new(timelock_blocks: u32) -> Self {
        GiftScript { timelock_blocks }
    }

    pub fn create_timelock_script(&self, receiver_key: PublicKey) -> Result<ScriptBuf, Error> {
        // Create a simple script that checks receiver's signature and timelock
        let script = bitcoin::script::Builder::new()
            .push_slice(&receiver_key.serialize())
            .push_opcode(bitcoin::opcodes::all::OP_CHECKSIGVERIFY)
            .push_int(self.timelock_blocks as i64)
            .push_opcode(bitcoin::opcodes::all::OP_CSV)
            .into_script();

        Ok(script.into())
    }

    pub fn create_taproot_tree(&self, keys: &GiftKeys) -> Result<ScriptBuf, Error> {
        // Get the aggregated MuSig2 key for the keypath
        let internal_key = keys.aggregate_musig2_key()?;
        
        // Create the timelock script for the script path
        let timelock_script = self.create_timelock_script(keys.receiver)?;

        // Initialize secp context
        let secp = Secp256k1::new();

        // Build taproot tree with our script
        let spend_info = TaprootBuilder::new()
            .add_leaf(0, timelock_script)
            .map_err(|e| Error::ScriptError(format!("Failed to add script to tree: {:?}", e)))?
            .finalize(&secp, internal_key)
            .map_err(|e| Error::ScriptError(format!("Failed to finalize taproot: {:?}", e)))?;

        // Convert to P2TR address
        let address = Address::p2tr(&secp, internal_key, spend_info.merkle_root(), Network::Regtest);
        Ok(address.script_pubkey())
    }
}