# Account Validation & Rent Safety Challenge

Build a Solana program with bulletproof account validation and rent-exempt handling.

## 🎯 Objective

Create a Solana program that demonstrates:
1. Comprehensive account validation
2. Proper rent-exemption handling
3. Safe account closure
4. Defense against common attacks

## 📋 Requirements

### Core Features
- [ ] Create user profile accounts
- [ ] Validate all account constraints
- [ ] Handle rent-exemption properly
- [ ] Safe account closure with lamport return

### Security Requirements
- [ ] Validate account ownership
- [ ] Check account discriminators
- [ ] Verify PDAs on-chain
- [ ] Prevent account substitution attacks
- [ ] Handle closing account race conditions

## 🏗️ Project Structure

```
account-validation-starter/
├── programs/
│   └── account-validation/
│       └── src/
│           └── lib.rs          # Your program logic
├── tests/
│   └── account-validation.ts   # Test file
├── Anchor.toml
├── Cargo.toml
└── package.json
```

## 🚀 Getting Started

### Prerequisites
- Rust & Cargo
- Solana CLI
- Anchor Framework

### Installation

```bash
npm install
anchor build
anchor test
```

## 📝 Evaluation Criteria

| Criteria | Weight |
|----------|--------|
| **Correctness** | Pass/Fail |
| **Validation** | 35% |
| **Rent Handling** | 25% |
| **Security** | 40% |

### What We Check
- **Validation:** All account constraints properly checked
- **Rent Handling:** Proper rent-exempt calculations, safe closures
- **Security:** No account substitution, proper ownership checks

## 📚 Resources

- [Solana Account Model](https://docs.solana.com/developing/programming-model/accounts)
- [Rent on Solana](https://docs.solana.com/developing/intro/rent)
- [Anchor Constraints](https://www.anchor-lang.com/docs/account-constraints)

## 🔗 Submission

1. Fork this repository
2. Implement the program
3. Ensure all tests pass
4. Submit your repo URL on the platform

Good luck! 🎉
