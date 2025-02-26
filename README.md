# MallowBTC - Timelocked Bitcoin Gift Tool

MallowBTC is a tool for creating timelocked Bitcoin gifts using public keys only. It allows you to create taproot addresses that combine timelock and multi-signature capabilities, enabling either cooperative spending or unilateral recipient spending after a timelock period.

## Features

- Create taproot addresses from public key information only
- Set custom timelock periods for automatic fund release
- Generate spending instructions for both cooperative and timelock spending
- Works with extended public keys (xpub/tpub) from standard wallets

## Usage

### Creating a timelocked gift

```bash
mallowbtc create --giver-tpub="[FINGERPRINT/PATH]TPUB" --receiver-tpub="[FINGERPRINT/PATH]TPUB" --timelock=52560
```

> **Important Note on Descriptor Format**: When providing descriptor strings, use only the key part without the `tr()` wrapper. For example:
> - Correct: `[73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*`  
> - Incorrect: `tr([73c5da0a/86'/1'/0']tpubDCvNAJkUmvjcXrTzyui9M7ehe1EXGkUmF12jTuJ9JxiAmg3tuVgocse3x5zx87WeydqwJWftYkyRQ4d7wF2F5Gs8AdzhJHVXAnMYG9QzmQ6/0/*)`

### Parameters

- `--giver-tpub`: Extended public key of the gift giver with fingerprint and derivation path
- `--receiver-tpub`: Extended public key of the gift receiver with fingerprint and derivation path
- `--timelock`: Number of blocks for the timelock period (approximately 52560 blocks ≈ 1 year)

## Build

```bash
cargo build --release
```

## License

[MIT](LICENSE)