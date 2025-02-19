# MallowBTC

A Bitcoin gift service that enables creating timelocked bitcoin gifts using taproot. The service combines MuSig2 for cooperative spending with timelock conditions for receiver-only spending after a set period.

## Features

- Public key operations only (no private keys required)
- MuSig2 key aggregation for cooperative spending
- Taproot descriptor support
- Timelock script generation
- Command-line interface for gift creation
- BIP32 key derivation

## Installation & Setup

Clone the repository:
```bash
git clone https://github.com/yourusername/mallowbtc.git
cd mallowbtc
```

You have two options to run the program:

### Option 1: Run through cargo
```bash
# Build the project
cargo build

# Run commands using cargo run
cargo run -- --help
cargo run -- create --help
```

### Option 2: Install the binary
```bash
# Install the binary to your system
cargo install --path .

# Now you can run directly
mallowbtc --help
mallowbtc create --help
```

## Command-Line Usage

Using cargo run:
```bash
# Show help
cargo run -- --help

# Create a gift
cargo run -- create --giver-tpub=<TPUB> --receiver-tpub=<TPUB> --timelock=52560

# Learn about requirements
cargo run -- create
```

Or if installed:
```bash
# Show help
mallowbtc --help

# Create a gift
mallowbtc create --giver-tpub=<TPUB> --receiver-tpub=<TPUB> --timelock=52560

# Learn about requirements
mallowbtc create
```

[Rest of README remains the same...]