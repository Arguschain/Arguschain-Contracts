# ArgusChain Contracts

Soroban smart contracts for ArgusChain fraud detection.

## Overview

On-chain risk registry for Stellar DEX wash trading detection.

## Features

- RiskScore Registry
- Authorized Submission
- Zero-Knowledge Proofs
- Event Logging
- Composable Design

## Architecture

RiskScore struct stores:
- score (0-100)
- benford_flag (bool)
- ml_flag (bool)
- timestamp (u64)
- confidence (0-100)

## Contract Functions

### submit_score
Submit fraud detection score on-chain.

### get_score
Retrieve score for wallet/asset pair.

### set_service_account
Configure authorized account.

### verify_score_proof
Verify zero-knowledge proofs.

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing

```bash
cargo test
```
