# MallowBTC

A Rust library for creating timelocked bitcoin gifts using taproot. Enables key-only operations to create taproot addresses with timelock and multi-signature capabilities.

## Current Features

- Public key operations only (no private keys required)
- Taproot descriptor support
- Timelock script generation
- BIP32 key derivation
- Test harness with regtest support

## Prerequisites

- Rust 1.70 or later
- Cargo package manager

## Installation

Clone the repository:
```bash
git clone https://github.com/yourusername/mallowbtc.git
cd mallowbtc
```

Build the project:
```bash
cargo build
```

## Usage Example

```rust
use mallowbtc::{GiftKeys, GiftScript};

// Create keys with giver's tpub
let mut keys = GiftKeys::new(
    "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6"
).unwrap();

// Add receiver's tpub
keys.add_receiver(
    "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6"
).unwrap();

// Create gift script with 1 year timelock
let gift = GiftScript::new(52560); // ~1 year in blocks

// Generate taproot address
let p2tr_script = gift.create_taproot_tree(&keys).unwrap();
let address = bitcoin::Address::from_script(&p2tr_script, bitcoin::Network::Regtest).unwrap();
println!("Gift address: {}", address);
```

## API Documentation

### GiftKeys
Manages the giver and receiver keys using taproot descriptors:
```rust
pub struct GiftKeys {
    giver_descriptor: Descriptor<DescriptorPublicKey>,
    receiver_descriptor: Option<Descriptor<DescriptorPublicKey>>,
}

impl GiftKeys {
    // Create new instance with giver's tpub
    pub fn new(giver_tpub: &str) -> Result<Self, Error>;
    
    // Add receiver's tpub
    pub fn add_receiver(&mut self, receiver_tpub: &str) -> Result<(), Error>;
    
    // Derive public keys
    pub fn derive_giver_pubkey(&self) -> Result<PublicKey, Error>;
    pub fn derive_receiver_pubkey(&self) -> Result<PublicKey, Error>;
}
```

### GiftScript
Creates taproot scripts with timelock conditions:
```rust
pub struct GiftScript {
    timelock_blocks: u32,
}

impl GiftScript {
    // Create new instance with timelock in blocks
    pub fn new(timelock_blocks: u32) -> Self;
    
    // Create taproot output script
    pub fn create_taproot_tree(&self, keys: &GiftKeys) -> Result<ScriptBuf, Error>;
    
    // Create timelock script using miniscript
    pub fn create_timelock_script(&self, receiver_key: PublicKey) -> Result<ScriptBuf, Error>;
}
```

## Testing

Run all tests:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

The test suite includes:
- Key derivation tests
- Script creation tests
- Address generation tests
- Full test harness for regtest

## Validation

You can verify the library output:

1. Using Sparrow Wallet:
   - Import the test tpub
   - Verify the derived addresses match

2. Using Bitcoin Core:
   - Import the taproot descriptors
   - Verify addresses are valid
   - Test spending paths (after transaction support is added)

3. Using the miniscript policy explorer:
   - Visit https://min.sc
   - Paste the generated script
   - Verify spending conditions

## Current Limitations

1. No transaction creation yet (requires wallet integration)
2. Fixed key fingerprints in descriptors
3. Limited path derivation (0/* and 1/*)
4. No MuSig2 support for cooperative spending

## Contributing

1. Run tests before submitting PRs:
```bash
cargo test
cargo clippy
cargo fmt
```

2. Update documentation for any new features

## License

[Add your license]

## Project Status

See project-plan.md for current status and roadmap.