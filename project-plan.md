# Mallow Bitcoin POC Project Plan

## Prerequisites

1. Bitcoin Core installation
2. Sparrow Wallet
3. Rust development environment (Complete)
4. Basic understanding of Bitcoin scripting and Taproot

## Setup Phase

### 1. Bitcoin Core Regtest Setup (Complete)
- Install Bitcoin Core
- Configure bitcoin.conf for regtest:
```
server=1

[regtest]
regtest=1
rpcallowip=127.0.0.1
rpcuser=sparrowuser
rpcpassword=sparrowpass
bind=127.0.0.1
rpcbind=127.0.0.1:18443
rpcport=18443
txindex=1
fallbackfee=0.0001
```
- Start Bitcoin Core in regtest mode
- Verify RPC connection

### 2. Sparrow Wallet Configuration (Complete)
- Configure Sparrow for regtest network
- Connect to local Bitcoin Core
- Create test wallet
- Mine blocks to fund wallet
- Verify balance appears

### 3. Rust Project Setup (Complete)
- Create new cargo project ✓
- Add dependencies:
```toml
[dependencies]
bitcoin = { version = "0.32.5", features = ["rand-std"] }
bdk_wallet = "1.1.0"
miniscript = "12.3.0"
thiserror = "2.0.11"
```
- Configure project structure ✓
- Implement error handling ✓

### 4. Gift Address Creation (Partially Complete)
- Implement XPub validation and parsing ✓
- Key derivation functionality ✓
- Implement timelock script creation using miniscript ✓
- Build taproot tree with:
  - Internal key (Current: single key, TODO: implement MuSig2)
  - Tapscript: `and_v(v:pk(receiver_key),older(blocks))` ✓
- Generate final P2TR address ✓
- Add address validation (TODO)

### 5. Transaction Creation with BDK (Not Started)
- Implement BDK wallet management
- Use BDK's TxBuilder for PSBT creation
- Setup UTXO tracking
- Implement fee estimation
- Create signing workflow

### 6. Testing Framework (Partially Complete)

#### 6.1 Key Generation Tests (Complete)
- Parse tpub ✓
- Validate key derivation ✓
- Invalid key handling ✓
- TODO: MuSig2 tests

#### 6.2 Script Generation Tests (Complete)
- Timelock script creation ✓
- Taproot tree construction ✓
- P2TR address generation ✓

#### 6.3 Transaction Tests (Not Started)
```rust
#[test]
fn test_funding() {
    // Mine blocks to giver address
    // Verify UTXO availability
    // Create and fund gift address
}

#[test]
fn test_cooperative_spend() {
    // Create signatures
    // Build and broadcast transaction
    // Verify spending success 
}

#[test]
fn test_timelock_spend() {
    // Advance blockchain past timelock
    // Create receiver signature
    // Build and broadcast using tapscript
    // Verify spending success
}
```

## Testing Process (Updated)

1. Unit Tests
```bash
# Run all tests
cargo test
```

2. Integration Tests (TODO)
- Setup test Bitcoin Core node
- Create test wallets
- Test complete gift workflow

3. Manual Testing (TODO)
- Use Sparrow to verify transactions
- Check address formats
- Validate script execution paths
- Test error conditions

## Updated Deliverables

1. Working Rust library for:
   - Gift address creation ✓
   - Key management ✓
   - Script generation ✓
   - Transaction construction (TODO)
   - Cooperative signing (TODO)
   - Timelock execution (TODO)

2. Test Suite:
   - Key generation ✓
   - Script creation ✓
   - Address formats ✓
   - Spending paths (TODO)
   - Error handling ✓

3. Documentation:
   - API documentation (TODO)
   - Usage examples (TODO)
   - Test coverage report (TODO)

## Next Steps (Prioritized)

1. Implement MuSig2 for cooperative spending
2. Add BDK transaction creation
3. Complete integration tests
4. Add command-line interface
5. Add mainnet safety checks

## Notes

- MuSig2 implementation might require additional dependencies
- Need to decide on BDK wallet persistence approach
- Consider adding logging framework
- Plan for error recovery in transaction workflows
- Add version compatibility checks for mainnet