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
