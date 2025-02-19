use clap::{Parser, Subcommand};
use bitcoin::base58;
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
        /// The giver's extended public key (tpub)
        #[arg(long, help = "Extended public key (tpub) from the giver's wallet")]
        giver_tpub: Option<String>,

        /// The receiver's extended public key (tpub)
        #[arg(long, help = "Extended public key (tpub) from the receiver's wallet")]
        receiver_tpub: Option<String>,

        /// Timelock period in blocks (approximately 52560 blocks = 1 year)
        #[arg(long, help = "Number of blocks for timelock (52560 ≈ 1 year)")]
        timelock: Option<u32>,
    },
}

fn validate_tpub(tpub: &str) -> Result<(), Error> {
    // Basic format checks
    if !tpub.starts_with("tpub") {
        return Err(Error::KeyError("Extended public key must start with 'tpub'".to_string()));
    }

    if tpub.len() != 111 {
        return Err(Error::KeyError("Invalid tpub length".to_string()));
    }

    // Try to decode base58
    base58::decode_check(tpub)
        .map_err(|e| Error::KeyError(format!("Invalid tpub encoding: {}", e)))?;

    Ok(())
}

fn show_create_requirements() {
    println!("\nTo create a timelocked bitcoin gift, you'll need:");
    println!("1. Giver's Extended Public Key (tpub)");
    println!("   - This is a watch-only key from your wallet");
    println!("   - It allows creating addresses without exposing private keys");
    println!("");
    println!("2. Receiver's Extended Public Key (tpub)");
    println!("   - This is also a watch-only key from their wallet");
    println!("   - They'll need this to receive and eventually spend the gift");
    println!("");
    println!("3. Timelock Period");
    println!("   - How long until the receiver can spend unilaterally");
    println!("   - Specified in blocks (52560 blocks ≈ 1 year)");
    println!("");
    println!("Need help getting these? Visit: https://docs.mallowbtc.org/setup-guide");
    println!("(Tip: Most wallet software can export extended public keys. Look for 'Export xpub' or similar options)");
    println!("");
    println!("Once you have the requirements, run:");
    println!("  mallowbtc create --giver-tpub=<TPUB> --receiver-tpub=<TPUB> --timelock=52560");
}

fn create_gift(giver_tpub: &str, receiver_tpub: &str, timelock: u32) -> Result<(), Error> {
    // Validate inputs
    validate_tpub(giver_tpub)?;
    validate_tpub(receiver_tpub)?;

    if timelock < 1 {
        return Err(Error::KeyError("Timelock must be greater than 0".to_string()));
    }

    // Create gift keys from tpubs
    let gift_keys = GiftKeys::from_tpubs(giver_tpub, receiver_tpub)?;

    // Create script with timelock
    let script = GiftScript::new(timelock);

    // Create the taproot output
    let taproot_script = script.create_taproot_tree(&gift_keys)?;

    // Create address from script
    let address = bitcoin::Address::from_script(&taproot_script, bitcoin::Network::Regtest)
        .map_err(|e| Error::ScriptError(format!("Failed to create address: {}", e)))?;
    
    // Display the results
    println!("\nGift Created Successfully!");
    println!("===========================");
    println!("");
    println!("Deposit Address: {}", address);
    println!("");
    println!("This address uses a Taproot output that enables:");
    println!("1. Cooperative spending between giver and receiver (using MuSig2)");
    println!("2. Receiver-only spending after {} blocks", timelock);
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