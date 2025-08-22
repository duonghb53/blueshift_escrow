# Pinocchio Escrow Program

A secure and trustless escrow program that enables token swaps between two parties on the Pinocchio blockchain.

## Overview

This escrow program implements a secure token swap mechanism where:
- One user (maker) can lock Token A while specifying the desired amount of Token B
- Another user (taker) can complete the swap by providing Token B
- The maker can refund their tokens if no taker is found

## Features

The program provides three main instructions:

1. **Make** - Create an escrow:
   - Maker deposits Token A into a secure vault
   - Specifies the desired Token B and amount
   - Sets up the escrow terms

2. **Take** - Complete the swap:
   - Taker sends Token B to the maker
   - Receives Token A from the escrow vault
   - Completes the atomic swap

3. **Refund** - Cancel the escrow:
   - Maker can retrieve their Token A
   - Only possible if no taker has completed the swap

## Project Structure

```
src/
├── instructions/
│   ├── make.rs     - Create escrow logic
│   ├── helpers.rs  - Shared helper functions
│   ├── mod.rs      - Module definitions
│   ├── refund.rs   - Refund logic
│   └── take.rs     - Complete swap logic
├── errors.rs       - Custom error definitions
├── lib.rs         - Program entrypoint
└── state.rs       - Escrow state management
```

## Installation

1. Ensure you have Rust installed with the 2021 edition
2. Clone this repository
3. Install dependencies:

```bash
cargo add pinocchio pinocchio-system pinocchio-token pinocchio-associated-token-account
```

## Building

```bash
cargo build-sbf
```

The build artifacts will be generated in `target/deploy/`

## State Management

The escrow state contains:
- Random seed (for multiple escrows per maker)
- Maker's public key
- Token A mint address (deposited token)
- Token B mint address (requested token)
- Requested amount of Token B
- PDA bump seed

## Security Features

- Trustless atomic swaps
- Secure PDA (Program Derived Address) derivation
- Protected token vaults
- Safe state management
- Proper validation checks

## Development Notes

- Uses `#[repr(C)]` for predictable memory layout
- Implements performance optimizations
- Includes proper error handling
- Follows Pinocchio program best practices

## License

This project is part of the Blueshift learning platform's challenges.

## Credits

Based on the [Pinocchio Escrow Challenge](https://learn.blueshift.gg/en/challenges/pinocchio-escrow) from Blueshift.
