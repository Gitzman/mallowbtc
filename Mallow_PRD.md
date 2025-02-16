Here's a PRD for Mallow Bitcoin:

# Mallow Bitcoin - Product Requirements Document

## Overview
Mallow Bitcoin is a service that enables users to create timelocked bitcoin gifts using public keys only. It generates taproot transactions that combine timelock and multi-signature capabilities, allowing either cooperative spending before the timelock or unilateral recipient spending after the timelock expires.

## Problem Statement
Currently, gifting bitcoin with timelock restrictions requires complex script creation and technical knowledge. Users need a simple way to create timelocked gifts that:
- Protect both the giver and receiver's interests
- Allow cooperative early withdrawal
- Guarantee recipient access after the timelock expires
- Maintain security by using only public keys

## Product Goals
1. Simplify creation of timelocked bitcoin gifts
2. Ensure security through public key-only operations
3. Provide flexibility in gift redemption
4. Make bitcoin gifting more accessible to mainstream users

## Core Features

### Input Requirements
- UTXO details from giver
- Giver's public key (xpub)
- Receiver's public key (xpub)
- Gift amount in bitcoin
- Timelock date

### Transaction Structure
- Uses Taproot P2TR output
- MuSig2 aggregate key in internal key position
- Single tapscript leaf with timelock and receiver's key
- Script structure: `tr(musig_giver_and_receiver_xpub,and_v(v:pk(receiver_key),older(timelock_date)))`

### Spending Paths
1. Cooperative Path (Before Timelock)
   - Requires signatures from both giver and receiver
   - Available immediately after creation
   - No timelock restrictions

2. Recipient Path (After Timelock)
   - Only requires receiver's signature
   - Available after timelock expiration
   - Uses tapscript revelation

## User Flow

1. Gift Creation
   - Giver inputs UTXO details
   - Enters public keys
   - Specifies amount and timelock date
   - Reviews transaction details
   - Receives transaction for signing

2. Gift Redemption
   - Before timelock: Both parties coordinate to sign
   - After timelock: Receiver independently signs

## Technical Requirements

### Security
- No private key handling by the service
- All cryptographic operations done client-side
- Input validation for all parameters
- Secure taproot address generation

### Bitcoin Network
- Support for mainnet and testnet
- Fee estimation integration
- UTXO validation
- Transaction broadcast options

## Future Considerations
1. Multi-UTXO input support
2. Integration with popular wallet software
3. Batch gift creation
4. Gift tracking and notification system
5. Mobile app development

## Success Metrics
1. Number of gifts created
2. Transaction success rate
3. User feedback on ease of use
4. Average gift value
5. Percentage of early vs. timelocked redemptions

## Limitations and Risks
1. Requires both parties to understand public key management
2. Network fees must be considered in gift amount
3. Timelock period is irreversible once set
4. Users must securely store their keys

This PRD outlines the core functionality needed to create a secure and user-friendly bitcoin gifting service using timelocks and taproot technology.