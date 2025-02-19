# Mallow Bitcoin POC Project Plan

## Prerequisites

1. Bitcoin Core installation
2. Sparrow Wallet
3. Rust development environment (Complete)
4. Basic understanding of Bitcoin scripting and Taproot

## Setup Phase

### 1. Bitcoin Core Regtest Setup (Complete)
- Install Bitcoin Core
- Configure bitcoin.conf for regtest
- Start Bitcoin Core in regtest mode
- Verify RPC connection

### 2. Sparrow Wallet Configuration (Complete)
- Configure Sparrow for regtest network
- Connect to local Bitcoin Core
- Create test wallet
- Mine blocks to fund wallet
- Verify balance appears

### 3. Project Setup (Complete)
- Create new cargo project ✓
- Add dependencies ✓
  - Bitcoin library
  - BDK
  - Miniscript
  - MuSig2
  - CLAP for CLI
- Configure project structure ✓
- Implement error handling ✓

### 4. Key Management (Complete)
- XPub validation and parsing ✓
- Key derivation functionality ✓
- MuSig2 aggregation for cooperative spending ✓
- Basic tpub validation ✓
- Support for descriptor-based derivation ✓

### 5. Script Creation (Complete)
- Timelock script creation ✓
- Taproot tree with MuSig2 internal key ✓
- Generation of P2TR addresses ✓
- Script validation ✓

### 6. CLI Implementation (Complete)
- Basic command structure ✓
- Create gift command ✓
- Input validation ✓
- User guidance and documentation ✓

### 7. Testing Framework (Partially Complete)

#### 7.1 Unit Tests (Complete)
- Key generation ✓
- Script generation ✓
- MuSig2 aggregation ✓
- tpub validation ✓

#### 7.2 Integration Tests (Complete)
- Complete gift workflow ✓
- Descriptor derivation ✓
- Address generation ✓

## Next Steps (Prioritized)

1. Additional Key Validation
   - Verify derivation paths in tpubs
   - Check key fingerprints
   - Validate network prefixes

2. Network Support
   - Add network selection (mainnet/testnet/regtest)
   - Network-specific address formatting
   - Safety checks for mainnet usage

3. Enhanced CLI Features
   - Add debug output options
   - Script hex display for verification
   - Progress indicators for key operations
   - Color output for better readability

4. Transaction Creation
   - Implement BDK wallet management
   - UTXO selection and tracking
   - Fee estimation
   - PSBT creation

5. User Experience
   - Better error messages
   - Interactive mode
   - Config file support
   - Wallet software integration guide

6. Testing Improvements
   - Add more edge cases
   - Network-specific tests
   - Performance benchmarks
   - Fuzzing for key and script inputs

## Success Metrics
1. Number of gifts created
2. Transaction success rate
3. User feedback on ease of use
4. Average gift value
5. Percentage of early vs. timelocked redemptions

## Notes

- Need to decide on config file format
- Plan for error recovery in transaction workflows
- Consider adding logging framework
- Need mainnet safety guidelines