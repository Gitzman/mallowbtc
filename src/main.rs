use clap::{Parser, Subcommand};
use mallowbtc::{GiftKeys, GiftScript, Error};

/// Mallow Bitcoin - Timelocked Bitcoin Gift Service
#[derive(Parser, Debug)]
#[command(
    name = "mallowbtc",
    author = "Block",
    version = "0.1.0",
    about = "Create timelocked bitcoin gifts using public keys only",
    long_about = "Mallow Bitcoin is a service that enables users to create timelocked bitcoin gifts using public keys only. 
    It generates taproot transactions that combine timelock and multi-signature capabilities, 
    allowing either cooperative spending before the timelock or unilateral recipient spending after the timelock expires."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new timelocked bitcoin gift
    Create {
        /// The giver's extended public key with fingerprint and path
        #[arg(long, help = "Extended public key with fingerprint and path: [fingerprint/path]tpub...")]
        giver_tpub: Option<String>,

        /// The receiver's extended public key with fingerprint and path
        #[arg(long, help = "Extended public key with fingerprint and path: [fingerprint/path]tpub...")]
        receiver_tpub: Option<String>,

        /// Timelock period in blocks (approximately 52560 blocks = 1 year)
        #[arg(long, help = "Number of blocks for timelock (52560 ≈ 1 year)")]
        timelock: Option<u32>,
    },
}

fn show_create_requirements() {
    println!("\nTo create a timelocked bitcoin gift, you'll need:");
    println!("1. Giver's Extended Public Key (tpub)");
    println!("   - This is a watch-only key from your wallet");
    println!("   - Format: [fingerprint/derivation_path]tpub...");
    println!("   - Example: [73c5da0a/86'/1'/0']tpubD...");
    println!("");
    println!("2. Receiver's Extended Public Key (tpub)");
    println!("   - This is also a watch-only key from their wallet");
    println!("   - Format: [fingerprint/derivation_path]tpub...");
    println!("   - Example: [143df5a6/86'/1'/1']tpubD...");
    println!("");
    println!("3. Timelock Period");
    println!("   - How long until the receiver can spend unilaterally");
    println!("   - Specified in blocks (52560 blocks ≈ 1 year)");
    println!("");
    println!("Need help getting these? Visit: https://docs.mallowbtc.org/setup-guide");
    println!("(Tip: Most wallet software can export extended public keys with fingerprints. Look for 'Export xpub' or similar options)");
    println!("");
    println!("Once you have the requirements, run:");
    println!("  mallowbtc create --giver-tpub=\"[FINGERPRINT/PATH]TPUB\" --receiver-tpub=\"[FINGERPRINT/PATH]TPUB\" --timelock=52560");
}

fn create_gift(giver_pk: &str, receiver_pk: &str, timelock: u32) -> Result<(), Error> {
    // Create gift keys from descriptors
    let gift_keys = GiftKeys::from_descriptor_strings(giver_pk, receiver_pk)?;

    if timelock < 1 {
        return Err(Error::KeyError("Timelock must be greater than 0".to_string()));
    }

    // Create script with timelock
    let script = GiftScript::new(timelock);
    
    // Get the taproot output and spend info
    let (taproot_script, spend_info) = script.create_taproot_tree(&gift_keys)?;

    // Create address from script
    let address = bitcoin::Address::from_script(&taproot_script, bitcoin::Network::Regtest)
        .map_err(|e| Error::ScriptError(format!("Failed to create address: {}", e)))?;
    
    // Get descriptor string for the gift
    let descriptor = script.create_gift_descriptor(&gift_keys)?;
    
    // Get the timelock script for reference
    let timelock_script = script.create_timelock_script(gift_keys.receiver_x_only_pub()?)?;
    
    // Get the policy format for the script
    let policy_format = script.script_policy_format(gift_keys.receiver_x_only_pub()?);
    
    // Get control block information
    let merkle_root = spend_info.merkle_root();
    // Create control block with the leaf version
    let leaf_version = bitcoin::taproot::LeafVersion::from_consensus(0xc0)
        .map_err(|e| Error::ScriptError(format!("Invalid leaf version: {:?}", e)))?;
    let control_block = spend_info.control_block(&(timelock_script.clone(), leaf_version))
        .ok_or_else(|| Error::ScriptError("Failed to create control block".to_string()))?;
    
    // Display the results
    println!("\nGift Created Successfully!");
    println!("===========================");
    println!("");
    println!("Deposit Address: {}", address);
    println!("Timelock Period: {} blocks", timelock);
    
    // Key information
    println!("\nSpending Information:");
    println!("---------------------");
    println!("Internal Key (Giver): {}", gift_keys.giver_x_only_pub()?);
    println!("Receiver Public Key: {}", gift_keys.receiver_x_only_pub()?);
    
    // Script information
    println!("\nTaproot Script Information:");
    println!("-------------------------");
    println!("Timelock Script (hex): {}", hex::encode(timelock_script.as_bytes()));
    println!("Script ASM: {}", timelock_script);
    println!("Policy Format: {}", policy_format);
    println!("Merkle Root: {:?}", merkle_root);
    println!("Descriptor: {}", descriptor);
    
    // Control block information
    println!("\nTapscript Spending Details:");
    println!("--------------------------");
    println!("Control Block (hex): {}", hex::encode(control_block.serialize()));
    println!("Leaf Version: 0xc0 (Tapscript)");
    println!("Script position in tree: 0");
    
    // Spending instructions
    println!("\nTo Spend After Timelock:");
    println!("----------------------");
    println!("1. Create a transaction spending from this address");
    println!("2. Set the transaction's nSequence to at least: {}", timelock);
    println!("3. Witness stack should be (in order):");
    println!("   - Receiver's signature (schnorr, 64 bytes)");
    println!("   - Timelock script (shown above)");
    println!("   - Control block (shown above)");
    
    // Additional information
    println!("\nKey Usage Information:");
    println!("--------------------");
    println!("This address uses a Taproot output that enables:");
    println!("1. Cooperative spending using the internal key (giver)");
    println!("2. Receiver-only spending after {} blocks using the script path", timelock);
    println!("");
    println!("Share this address with the giver to receive the gift amount.");
    println!("Keep your wallet's private keys secure - they'll be needed to spend the funds.");

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { giver_tpub, receiver_tpub, timelock } => {
            if giver_tpub.is_none() || receiver_tpub.is_none() || timelock.is_none() {
                println!("Welcome to Mallow Bitcoin Gift Creation!");
                println!("=====================================");
                show_create_requirements();
                return;
            }

            match create_gift(
                &giver_tpub.unwrap(),
                &receiver_tpub.unwrap(),
                timelock.unwrap()
            ) {
                Ok(()) => {},
                Err(e) => {
                    println!("\nError creating gift: {}", e);
                    println!("Please check your inputs and try again.");
                }
            }
        }
    }
}