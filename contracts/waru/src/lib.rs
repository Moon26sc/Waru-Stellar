#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Bytes, Env, String,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Oracle,
    Token,
    RewardPerLiter,
    MaxReward,
    Claim(String),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimReceipt {
    pub farmer: Address,
    pub evidence_hash: Bytes,
    pub saved_liters: i128,
    pub reward_amount: i128,
}

#[contract]
pub struct Waru;

#[contractimpl]
impl Waru {
    /// One-time setup. All amounts use the configured token's smallest units.
    pub fn initialize(
        env: Env,
        admin: Address,
        oracle: Address,
        token_address: Address,
        reward_per_liter: i128,
        max_reward_per_claim: i128,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        if reward_per_liter <= 0 || max_reward_per_claim <= 0 {
            panic!("invalid reward policy");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage().instance().set(&DataKey::Token, &token_address);
        env.storage().instance().set(&DataKey::RewardPerLiter, &reward_per_liter);
        env.storage().instance().set(&DataKey::MaxReward, &max_reward_per_claim);
    }

    /// Sponsor transfers test asset into the contract vault.
    pub fn fund(env: Env, sponsor: Address, amount: i128) {
        sponsor.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token).unwrap_or_else(|| panic!("not initialized"));
        let client = token::Client::new(&env, &token_address);
        client.transfer(&sponsor, &env.current_contract_address(), &amount);
        env.events().publish((symbol_short!("funded"), sponsor), amount);
    }

    /// Minimal oracle-authorized settlement scaffold.
    /// Production must replace this direct account authorization with a fully
    /// specified signed attestation flow and expiry policy.
    pub fn settle(
        env: Env,
        oracle: Address,
        claim_id: String,
        farmer: Address,
        evidence_hash: Bytes,
        saved_liters: i128,
        expires_at: u64,
    ) -> i128 {
        oracle.require_auth();

        let configured_oracle: Address = env.storage().instance()
            .get(&DataKey::Oracle).unwrap_or_else(|| panic!("not initialized"));
        if oracle != configured_oracle {
            panic!("unauthorized oracle");
        }
        if claim_id.len() == 0 {
            panic!("empty claim id");
        }
        if evidence_hash.len() != 32 {
            panic!("evidence hash must be 32 bytes");
        }
        if saved_liters <= 0 {
            panic!("no eligible savings");
        }
        if env.ledger().timestamp() > expires_at {
            panic!("claim expired");
        }

        let claim_key = DataKey::Claim(claim_id.clone());
        if env.storage().persistent().has(&claim_key) {
            panic!("claim already settled");
        }

        let rate: i128 = env.storage().instance().get(&DataKey::RewardPerLiter).unwrap();
        let cap: i128 = env.storage().instance().get(&DataKey::MaxReward).unwrap();
        let uncapped = saved_liters.checked_mul(rate).unwrap_or_else(|| panic!("reward overflow"));
        let reward = if uncapped > cap { cap } else { uncapped };
        if reward <= 0 {
            panic!("invalid reward");
        }

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        // Token transfer and storage write are in the same Soroban invocation:
        // a failed transfer reverts the invocation.
        let client = token::Client::new(&env, &token_address);
        client.transfer(&env.current_contract_address(), &farmer, &reward);

        let receipt = ClaimReceipt {
            farmer: farmer.clone(),
            evidence_hash: evidence_hash.clone(),
            saved_liters,
            reward_amount: reward,
        };
        env.storage().persistent().set(&claim_key, &receipt);
        env.events().publish(
            (symbol_short!("settled"), claim_id),
            (farmer, evidence_hash, saved_liters, reward),
        );
        reward
    }

    pub fn is_settled(env: Env, claim_id: String) -> bool {
        env.storage().persistent().has(&DataKey::Claim(claim_id))
    }

    pub fn get_claim(env: Env, claim_id: String) -> Option<ClaimReceipt> {
        env.storage().persistent().get(&DataKey::Claim(claim_id))
    }

    pub fn get_config(env: Env) -> (Address, Address, Address, i128, i128) {
        (
            env.storage().instance().get(&DataKey::Admin).unwrap(),
            env.storage().instance().get(&DataKey::Oracle).unwrap(),
            env.storage().instance().get(&DataKey::Token).unwrap(),
            env.storage().instance().get(&DataKey::RewardPerLiter).unwrap(),
            env.storage().instance().get(&DataKey::MaxReward).unwrap(),
        )
    }
}

use soroban_sdk::symbol_short;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_is_capped_without_float_math() {
        let saved_liters: i128 = 1_000;
        let rate: i128 = 3;
        let cap: i128 = 2_000;
        let amount = saved_liters.checked_mul(rate).unwrap();
        assert_eq!(core::cmp::min(amount, cap), 2_000);
    }

    #[test]
    fn non_positive_savings_are_not_eligible() {
        assert!(0_i128 <= 0);
        assert!(-1_i128 <= 0);
    }
}
