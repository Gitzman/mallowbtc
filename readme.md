# MallowBTC

A Bitcoin gift service that enables creating timelocked bitcoin gifts using taproot. The service combines MuSig2 for cooperative spending with timelock conditions for receiver-only spending after a set period.

## Features

- Public key operations only (no private keys required)
- MuSig2 key aggregation for cooperative spending
- Taproot descriptor support
- Timelock script generation
- Command-line interface for gift creation
- BIP32 key derivation

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

## Command-Line Usage

Create a new timelocked bitcoin gift:
```bash
mallowbtc create --giver-tpub=<TPUB> --receiver-tpub=<TPUB> --timelock=52560
```

Get help and available commands:
```bash
mallowbtc --help
```

Learn about gift creation requirements:
```bash
mallowbtc create
```

## Library Usage

### Creating a Gift with MuSig2

```rust
use mallowbtc::{GiftKeys, GiftScript};

// Create keys from tpubs
let gift_keys = GiftKeys::from_tpubs(
    "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6",
    "tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6"
).unwrap();

// Create gift script with 1 year timelock
let gift = GiftScript::new(52560); // ~1 year in blocks

// Generate taproot address
let p2tr_script = gift.create_taproot_tree(&gift_keys).unwrap();
let address = bitcoin::Address::from_script(&p2tr_script, bitcoin::Network::Regtest).unwrap();
println!("Gift address: {}", address);
```

## API Documentation

### GiftKeys
Manages gift participant keys and MuSig2 aggregation:
```rust
pub struct GiftKeys {
    pub giver: PublicKey,
    pub receiver: PublicKey,
}

impl GiftKeys {
    // Create new instance from raw public keys
    pub fn new(giver: PublicKey, receiver: PublicKey) -> Self;
    
    // Create from descriptor strings
    pub fn from_descriptors(giver_desc: &str, receiver_desc: &str) -> Result<Self, Error>;
    
    // Create from tpub strings
    pub fn from_tpubs(giver_tpub: &str, receiver_tpub: &str) -> Result<Self, Error>;
    
    // Aggregate keys using MuSig2
    pub fn aggregate_musig2_key(&self) -> Result<XOnlyPublicKey, Error>;
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
    
    // Create timelock script for script path
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
- Direct key aggregation tests
- HD wallet derivation tests
- Script creation tests
- Address generation tests
- Full integration tests

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

## Current State

The project currently supports:
1. ✅ MuSig2 key aggregation
2. ✅ Taproot script creation
3. ✅ CLI interface
4. ✅ HD wallet integration
5. ❌ Transaction creation (planned)
6. ❌ Wallet software integration (planned)

See project-plan.md for detailed status and roadmap.

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