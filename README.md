# ✈️ TravelFund — Shared Travel Fund on Stellar

> A trustless, transparent smart contract on the Stellar network that lets a group of friends pool money for a shared trip — every contribution, refund, and payout recorded permanently on-chain.

---

## 📖 Project Description

**TravelFund** is a [Soroban](https://soroban.stellar.org) smart contract built on the Stellar blockchain. It solves a classic group-travel problem: _who has paid what, and can we trust the running total?_

Instead of relying on a spreadsheet, a group chat, or one person collecting cash, TravelFund puts the ledger on-chain. Every member's contribution is publicly verifiable, the goal progress is visible to everyone in real time, and no single person can silently withdraw funds — all movements are permanently recorded.

---

## 🔍 What It Does

1. **Admin creates a fund** — sets the trip name (e.g. `"Paris 2025"`) and a target goal in stroops (Stellar's smallest unit, 1 XLM = 10 000 000 stroops).
2. **Friends contribute** — each member calls `contribute()` with their share. Their balance is tracked individually on-chain.
3. **Anyone can check progress** — `get_status()` returns the trip name, goal, total raised, lock state, and number of contributors at any time.
4. **Members can request refunds** — before the fund is locked, any contributor can reclaim part or all of their contribution via `refund()`.
5. **Admin locks the fund** — once the group is committed to the trip, the admin calls `lock_fund()`. No further deposits or refunds are accepted.
6. **Payout is recorded** — when the trip expenses are settled, the admin calls `record_payout()`, which writes the final disbursement event to the on-chain ledger and zeroes the balance.

Every step — deposit, refund, payout — is appended to an **immutable transaction log** stored in contract storage.

---

## ✨ Features

| Feature | Description |
|---|---|

. |
| 🎯 **Goal tracking** | The contract stores a target amount; `remaining()` tells you exactly how many stroops are still needed. |
| 🔒 **Fund locking** | The admin can lock the fund to freeze deposits and refunds once the group is ready to commit to the trip. |

| 👁️ **Public fund status** | Anyone can call `get_status()` to see trip name, goal, total raised, lock state, and member count — no wallet required to read. |
| 🔄 **Refunds before lock** | Contributors can reclaim funds at any time while the fund is unlocked, preventing anyone from being trapped. |
| 🛡️ **Admin-only controls** | Lock, unlock, and payout operations are gated behind admin `require_auth()` — the admin address is set once at initialisation and cannot be changed. |
| 🧪 **Full test suite** | Five unit tests cover the happy path, refunds, locking, the TX log, and the full lifecycle using `soroban-sdk/testutils`. |

---

## 🗂️ Project Structure

```
travel-fund/
├── Cargo.toml          # Rust workspace & Soroban SDK dependency
└── src/
    └── lib.rs          # Smart contract (all logic, types, and tests)
```

---

## 🚀 Getting Started

### Prerequisites

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add the WASM target
rustup target add wasm32-unknown-unknown

# 3. Install the Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cd travel-fund
stellar contract build
# Output: target/wasm32-unknown-unknown/release/travel_fund.wasm
```

### Test

```bash
cargo test --features testutils
```

### Deploy to Testnet

```bash
# 1. Generate a keypair and fund it from Friendbot
stellar keys generate alice --network testnet
stellar keys fund alice --network testnet

# 2. Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/travel_fund.wasm \
  --source alice \
  --network testnet

# 3. Initialise the fund (replace CONTRACT_ID and ADMIN_ADDRESS)
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- initialize \
  --admin ADMIN_ADDRESS \
  --trip_name "Paris 2025" \
  --goal 5000000000
```

### Interact

```bash
# Contribute 100 XLM
stellar contract invoke --id CONTRACT_ID --source alice --network testnet \
  -- contribute --from ALICE_ADDRESS --amount 1000000000

# Check fund status
stellar contract invoke --id CONTRACT_ID --source alice --network testnet \
  -- get_status

# Get transaction log
stellar contract invoke --id CONTRACT_ID --source alice --network testnet \
  -- get_tx_log
```

---

## 🔗 Data Types

### `TxRecord`
```rust
pub struct TxRecord {
    pub contributor: Address,
    pub amount:      i128,    // positive = deposit, negative = refund/payout
    pub kind:        String,  // "deposit" | "refund" | "payout"
    pub timestamp:   u64,     // ledger timestamp
}
```

### `FundStatus`
```rust
pub struct FundStatus {
    pub trip_name:    String,
    pub goal_amount:  i128,
    pub total_raised: i128,
    pub is_locked:    bool,
    pub member_count: u32,
}
```

---

## 📋 Contract Interface

| Function | Access | Description |
|---|---|---|
| `initialize(admin, trip_name, goal)` | One-time | Deploy and configure the fund |
| `contribute(from, amount)` | Any member | Add funds to the pool |
| `refund(to, amount)` | Any member | Withdraw while unlocked |
| `lock_fund()` | Admin | Freeze deposits & refunds |
| `unlock_fund()` | Admin | Re-open the fund |
| `record_payout(destination)` | Admin | Record final disbursement |
| `get_status()` | Public | Fund snapshot |
| `get_balance(member)` | Public | Balance for one address |
| `get_members()` | Public | All contributor addresses |
| `get_tx_log()` | Public | Full transaction history |
| `remaining()` | Public | Stroops left to reach goal |

---

## 🧱 Built With

- [Soroban SDK](https://soroban.stellar.org) `v21`
- [Stellar CLI](https://github.com/stellar/stellar-cli)
- Rust `edition = "2021"`, targeting `wasm32-unknown-unknown`

---




wallet address:# ✈️ TravelFund — Shared Travel Fund on Stellar

> A trustless, transparent smart contract on the Stellar network that lets a group of friends pool money for a shared trip — every contribution, refund, and payout recorded permanently on-chain.

---

## 📖 Project Description

**TravelFund** is a [Soroban](https://soroban.stellar.org) smart contract built on the Stellar blockchain. It solves a classic group-travel problem: _who has paid what, and can we trust the running total?_

Instead of relying on a spreadsheet, a group chat, or one person collecting cash, TravelFund puts the ledger on-chain. Every member's contribution is publicly verifiable, the goal progress is visible to everyone in real time, and no single person can silently withdraw funds — all movements are permanently recorded.

---

## 🔍 What It Does

1. **Admin creates a fund** — sets the trip name (e.g. `"Paris 2025"`) and a target goal in stroops (Stellar's smallest unit, 1 XLM = 10 000 000 stroops).
2. **Friends contribute** — each member calls `contribute()` with their share. Their balance is tracked individually on-chain.
3. **Anyone can check progress** — `get_status()` returns the trip name, goal, total raised, lock state, and number of contributors at any time.
4. **Members can request refunds** — before the fund is locked, any contributor can reclaim part or all of their contribution via `refund()`.
5. **Admin locks the fund** — once the group is committed to the trip, the admin calls `lock_fund()`. No further deposits or refunds are accepted.
6. **Payout is recorded** — when the trip expenses are settled, the admin calls `record_payout()`, which writes the final disbursement event to the on-chain ledger and zeroes the balance.

Every step — deposit, refund, payout — is appended to an **immutable transaction log** stored in contract storage.

---

## ✨ Features

| Feature | Description |
|---|---|
. |
| 🎯 **Goal tracking** | The contract stores a target amount; `remaining()` tells you exactly how many stroops are still needed. |
| 🔒 **Fund locking** | The admin can lock the fund to freeze deposits and refunds once the group is ready to commit to the trip. |

| 👁️ **Public fund status** | Anyone can call `get_status()` to see trip name, goal, total raised, lock state, and member count — no wallet required to read. |
| 🔄 **Refunds before lock** | Contributors can reclaim funds at any time while the fund is unlocked, preventing anyone from being trapped. |
| 🛡️ **Admin-only controls** | Lock, unlock, and payout operations are gated behind admin `require_auth()` — the admin address is set once at initialisation and cannot be changed. |
| 🧪 **Full test suite** | Five unit tests cover the happy path, refunds, locking, the TX log, and the full lifecycle using `soroban-sdk/testutils`. |

---

## 🗂️ Project Structure

```
travel-fund/
├── Cargo.toml          # Rust workspace & Soroban SDK dependency
└── src/
    └── lib.rs          # Smart contract (all logic, types, and tests)
```

---

## 🚀 Getting Started

### Prerequisites

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add the WASM target
rustup target add wasm32-unknown-unknown

# 3. Install the Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cd travel-fund
stellar contract build
# Output: target/wasm32-unknown-unknown/release/travel_fund.wasm
```

### Test

```bash
cargo test --features testutils
```

### Deploy to Testnet

```bash
# 1. Generate a keypair and fund it from Friendbot
stellar keys generate alice --network testnet
stellar keys fund alice --network testnet

# 2. Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/travel_fund.wasm \
  --source alice \
  --network testnet

# 3. Initialise the fund (replace CONTRACT_ID and ADMIN_ADDRESS)
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- initialize \
  --admin ADMIN_ADDRESS \
  --trip_name "Paris 2025" \
  --goal 5000000000
```

### Interact

```bash
# Contribute 100 XLM
stellar contract invoke --id CONTRACT_ID --source alice --network testnet \
  -- contribute --from ALICE_ADDRESS --amount 1000000000

# Check fund status
stellar contract invoke --id CONTRACT_ID --source alice --network testnet \
  -- get_status

# Get transaction log
stellar contract invoke --id CONTRACT_ID --source alice --network testnet \
  -- get_tx_log
```

---

## 🔗 Data Types

### `TxRecord`
```rust
pub struct TxRecord {
    pub contributor: Address,
    pub amount:      i128,    // positive = deposit, negative = refund/payout
    pub kind:        String,  // "deposit" | "refund" | "payout"
    pub timestamp:   u64,     // ledger timestamp
}
```

### `FundStatus`
```rust
pub struct FundStatus {
    pub trip_name:    String,
    pub goal_amount:  i128,
    pub total_raised: i128,
    pub is_locked:    bool,
    pub member_count: u32,
}
```

---

## 📋 Contract Interface

| Function | Access | Description |
|---|---|---|
| `initialize(admin, trip_name, goal)` | One-time | Deploy and configure the fund |
| `contribute(from, amount)` | Any member | Add funds to the pool |
| `refund(to, amount)` | Any member | Withdraw while unlocked |
| `lock_fund()` | Admin | Freeze deposits & refunds |
| `unlock_fund()` | Admin | Re-open the fund |
| `record_payout(destination)` | Admin | Record final disbursement |
| `get_status()` | Public | Fund snapshot |
| `get_balance(member)` | Public | Balance for one address |
| `get_members()` | Public | All contributor addresses |
| `get_tx_log()` | Public | Full transaction history |
| `remaining()` | Public | Stroops left to reach goal |

---

## 🧱 Built With

- [Soroban SDK](https://soroban.stellar.org) `v21`
- [Stellar CLI](https://github.com/stellar/stellar-cli)
- Rust `edition = "2021"`, targeting `wasm32-unknown-unknown`

---




wallet address:GA243UV4KSLAQHDQZNAWNFXEUM3K7MZSTR5F7V7CJEK3XO5ODAWJ266B
                                                                                                     
contract address:CABMQEID6B4XGYTUPBAC6NAEHSXDQE5NHK77O6CPYE2KYSFSR3GVH5AN

https://stellar.expert/explorer/testnet/contract/CABMQEID6B4XGYTUPBAC6NAEHSXDQE5NHK77O6CPYE2KYSFSR3GVH5AN

<img width="1918" height="911" alt="image" src="https://github.com/user-attachments/assets/64e13dee-e8da-438a-b736-044b12ad1bb2" />



