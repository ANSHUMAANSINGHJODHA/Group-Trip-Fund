#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, contractclient,
    Address, Env, Map, String, Symbol, Vec, symbol_short,
    log,
};

// ─────────────────────────────────────────────
//  Storage keys
// ─────────────────────────────────────────────
const ADMIN_KEY:       Symbol = symbol_short!("ADMIN");
const GOAL_KEY:        Symbol = symbol_short!("GOAL");
const TRIP_NAME_KEY:   Symbol = symbol_short!("TRIPNAME");
const LOCKED_KEY:      Symbol = symbol_short!("LOCKED");
const TOTAL_KEY:       Symbol = symbol_short!("TOTAL");
const MEMBERS_KEY:     Symbol = symbol_short!("MEMBERS");
const BALANCES_KEY:    Symbol = symbol_short!("BALANCES");
const TX_LOG_KEY:      Symbol = symbol_short!("TXLOG");

// ─────────────────────────────────────────────
//  Data types
// ─────────────────────────────────────────────

/// A single contribution or withdrawal recorded on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct TxRecord {
    pub contributor: Address,
    pub amount:      i128,       // positive = deposit, negative = refund
    pub kind:        String,     // "deposit" | "refund" | "payout"
    pub timestamp:   u64,
}

/// Public summary of the fund's state.
#[contracttype]
#[derive(Clone, Debug)]
pub struct FundStatus {
    pub trip_name:    String,
    pub goal_amount:  i128,
    pub total_raised: i128,
    pub is_locked:    bool,
    pub member_count: u32,
}

// ─────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────
#[contract]
pub struct TravelFundContract;

#[contractimpl]
impl TravelFundContract {

    // ── Initialisation ──────────────────────────────────────────────────

    /// Deploy and configure the travel fund.
    ///
    /// * `admin`     – address that can lock/unlock the fund and trigger payouts
    /// * `trip_name` – human-readable trip label stored on-chain
    /// * `goal`      – target amount in stroops (1 XLM = 10 000 000 stroops)
    pub fn initialize(
        env:       Env,
        admin:     Address,
        trip_name: String,
        goal:      i128,
    ) {
        // Guard: can only be called once
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("already initialised");
        }
        if goal <= 0 {
            panic!("goal must be positive");
        }

        admin.require_auth();

        env.storage().instance().set(&ADMIN_KEY,     &admin);
        env.storage().instance().set(&TRIP_NAME_KEY, &trip_name);
        env.storage().instance().set(&GOAL_KEY,      &goal);
        env.storage().instance().set(&LOCKED_KEY,    &false);
        env.storage().instance().set(&TOTAL_KEY,     &0_i128);

        // Empty collections
        let members:  Vec<Address>      = Vec::new(&env);
        let balances: Map<Address, i128> = Map::new(&env);
        let tx_log:   Vec<TxRecord>     = Vec::new(&env);

        env.storage().instance().set(&MEMBERS_KEY,  &members);
        env.storage().instance().set(&BALANCES_KEY, &balances);
        env.storage().instance().set(&TX_LOG_KEY,   &tx_log);

        log!(&env, "TravelFund initialised | trip={} goal={}", trip_name, goal);
    }

    // ── Contributions ───────────────────────────────────────────────────

    /// Contribute `amount` stroops to the fund.
    /// The caller must have authorised the transaction.
    pub fn contribute(env: Env, from: Address, amount: i128) {
        from.require_auth();
        Self::assert_not_locked(&env);

        if amount <= 0 {
            panic!("amount must be positive");
        }

        // Update per-member balance
        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&BALANCES_KEY).unwrap();

        let prev = balances.get(from.clone()).unwrap_or(0);
        balances.set(from.clone(), prev + amount);
        env.storage().instance().set(&BALANCES_KEY, &balances);

        // Track unique members
        let mut members: Vec<Address> =
            env.storage().instance().get(&MEMBERS_KEY).unwrap();
        if !members.contains(&from) {
            members.push_back(from.clone());
            env.storage().instance().set(&MEMBERS_KEY, &members);
        }

        // Update running total
        let total: i128 = env.storage().instance().get(&TOTAL_KEY).unwrap();
        env.storage().instance().set(&TOTAL_KEY, &(total + amount));

        // Append to immutable ledger
        Self::record_tx(&env, from.clone(), amount, String::from_str(&env, "deposit"));

        log!(&env, "Contribution | from={} amount={}", from, amount);
    }

    // ── Refunds ─────────────────────────────────────────────────────────

    /// Request a refund of `amount` stroops (only while fund is NOT locked).
    pub fn refund(env: Env, to: Address, amount: i128) {
        to.require_auth();
        Self::assert_not_locked(&env);

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&BALANCES_KEY).unwrap();

        let balance = balances.get(to.clone()).unwrap_or(0);
        if balance < amount {
            panic!("insufficient balance");
        }

        balances.set(to.clone(), balance - amount);
        env.storage().instance().set(&BALANCES_KEY, &balances);

        let total: i128 = env.storage().instance().get(&TOTAL_KEY).unwrap();
        env.storage().instance().set(&TOTAL_KEY, &(total - amount));

        Self::record_tx(&env, to.clone(), -amount, String::from_str(&env, "refund"));

        log!(&env, "Refund | to={} amount={}", to, amount);
    }

    // ── Admin actions ────────────────────────────────────────────────────

    /// Lock the fund (no more deposits or refunds).
    /// Only the admin may call this.
    pub fn lock_fund(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&LOCKED_KEY, &true);
        log!(&env, "Fund LOCKED");
    }

    /// Unlock the fund.
    pub fn unlock_fund(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&LOCKED_KEY, &false);
        log!(&env, "Fund UNLOCKED");
    }

    /// Mark the full balance as paid out (records a payout event).
    /// Actual XLM transfer is handled by the calling transaction.
    pub fn record_payout(env: Env, destination: Address) {
        Self::require_admin(&env);
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();

        let total: i128 = env.storage().instance().get(&TOTAL_KEY).unwrap();
        if total <= 0 {
            panic!("nothing to pay out");
        }

        Self::record_tx(&env, destination.clone(), -total, String::from_str(&env, "payout"));
        env.storage().instance().set(&TOTAL_KEY, &0_i128);

        log!(&env, "Payout recorded | to={} amount={}", destination, total);
    }

    // ── Read-only views ──────────────────────────────────────────────────

    /// Returns a snapshot of the fund's current state.
    pub fn get_status(env: Env) -> FundStatus {
        let trip_name: String  = env.storage().instance().get(&TRIP_NAME_KEY).unwrap();
        let goal:      i128    = env.storage().instance().get(&GOAL_KEY).unwrap();
        let total:     i128    = env.storage().instance().get(&TOTAL_KEY).unwrap();
        let locked:    bool    = env.storage().instance().get(&LOCKED_KEY).unwrap();
        let members: Vec<Address> = env.storage().instance().get(&MEMBERS_KEY).unwrap();

        FundStatus {
            trip_name,
            goal_amount:  goal,
            total_raised: total,
            is_locked:    locked,
            member_count: members.len(),
        }
    }

    /// Returns the balance contributed by a specific member.
    pub fn get_balance(env: Env, member: Address) -> i128 {
        let balances: Map<Address, i128> =
            env.storage().instance().get(&BALANCES_KEY).unwrap();
        balances.get(member).unwrap_or(0)
    }

    /// Returns the full on-chain transaction log.
    pub fn get_tx_log(env: Env) -> Vec<TxRecord> {
        env.storage().instance().get(&TX_LOG_KEY).unwrap()
    }

    /// Returns all contributor addresses.
    pub fn get_members(env: Env) -> Vec<Address> {
        env.storage().instance().get(&MEMBERS_KEY).unwrap()
    }

    /// Returns how many stroops remain until the goal is reached.
    pub fn remaining(env: Env) -> i128 {
        let goal:  i128 = env.storage().instance().get(&GOAL_KEY).unwrap();
        let total: i128 = env.storage().instance().get(&TOTAL_KEY).unwrap();
        (goal - total).max(0)
    }

    // ── Private helpers ──────────────────────────────────────────────────

    fn assert_not_locked(env: &Env) {
        let locked: bool = env.storage().instance().get(&LOCKED_KEY).unwrap_or(false);
        if locked {
            panic!("fund is locked");
        }
    }

    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();
    }

    fn record_tx(env: &Env, contributor: Address, amount: i128, kind: String) {
        let mut log_vec: Vec<TxRecord> =
            env.storage().instance().get(&TX_LOG_KEY).unwrap();

        log_vec.push_back(TxRecord {
            contributor,
            amount,
            kind,
            timestamp: env.ledger().timestamp(),
        });

        env.storage().instance().set(&TX_LOG_KEY, &log_vec);
    }
}

// ─────────────────────────────────────────────
//  Tests
// ─────────────────────────────────────────────
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::Env;

    fn setup() -> (Env, TravelFundContractClient<'static>, Address, Address, Address) {
        let env     = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, TravelFundContract);
        let client      = TravelFundContractClient::new(&env, &contract_id);

        let admin   = Address::generate(&env);
        let alice   = Address::generate(&env);
        let bob     = Address::generate(&env);

        client.initialize(
            &admin,
            &String::from_str(&env, "Paris 2025"),
            &5_000_000_000_i128, // 500 XLM goal
        );

        (env, client, admin, alice, bob)
    }

    #[test]
    fn test_contribute_and_status() {
        let (_env, client, _admin, alice, bob) = setup();

        client.contribute(&alice, &1_000_000_000); // 100 XLM
        client.contribute(&bob,   &2_000_000_000); // 200 XLM

        let status = client.get_status();
        assert_eq!(status.total_raised, 3_000_000_000);
        assert_eq!(status.member_count, 2);
        assert!(!status.is_locked);

        assert_eq!(client.get_balance(&alice), 1_000_000_000);
        assert_eq!(client.remaining(),         2_000_000_000);
    }

    #[test]
    fn test_refund() {
        let (_env, client, _admin, alice, _bob) = setup();

        client.contribute(&alice, &2_000_000_000);
        client.refund(&alice, &500_000_000);

        assert_eq!(client.get_balance(&alice), 1_500_000_000);
    }

    #[test]
    #[should_panic(expected = "fund is locked")]
    fn test_locked_prevents_deposit() {
        let (_env, client, _admin, alice, _bob) = setup();
        client.lock_fund();
        client.contribute(&alice, &1_000_000_000);
    }

    #[test]
    fn test_tx_log_records_events() {
        let (_env, client, _admin, alice, bob) = setup();

        client.contribute(&alice, &1_000_000_000);
        client.contribute(&bob,   &1_000_000_000);
        client.refund(&alice,     &500_000_000);

        let log = client.get_tx_log();
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn test_full_lifecycle() {
        let (env, client, admin, alice, bob) = setup();

        client.contribute(&alice, &2_500_000_000);
        client.contribute(&bob,   &2_500_000_000);

        assert_eq!(client.remaining(), 0);

        client.lock_fund();
        assert!(client.get_status().is_locked);

        client.record_payout(&admin);
        assert_eq!(client.get_status().total_raised, 0);
    }
}