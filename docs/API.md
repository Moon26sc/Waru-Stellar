# Contract API (proposed)

The initial Rust scaffold exposes a minimal sponsor vault and oracle settlement flow. This API is a design boundary; inspect generated contract spec after building before integrating clients.

## `initialize(admin, oracle, token, reward_per_liter, max_reward_per_claim)`
One-time initialization. Admin authorizes. Sets roles, asset and reward policy.

## `fund(sponsor, amount)`
Sponsor authorizes a token transfer into the contract vault. Emits funding event.

## `settle(oracle, claim_id, farmer, evidence_hash, saved_liters, expires_at)`
Oracle authorizes. Contract rejects:
- uninitialized contract;
- wrong oracle;
- empty/duplicate claim ID;
- evidence hash not 32 bytes;
- zero/negative saving;
- expired claim;
- reward <= 0 or above cap.

On success, contract transfers configured token from vault to farmer and records the claim atomically.

## Read methods
- `get_config()`
- `is_settled(claim_id)`
- `get_claim(claim_id)`

## Events
- `funded(sponsor, amount)`
- `settled(claim_id, farmer, evidence_hash, saved_liters, reward_amount)`

## Important
For a production protocol, bind the signed oracle attestation to the full payload (including wallet, period, method version, amount/rate, and expiry), not merely a free-form hash. Add signature verification or a rigorously specified Soroban authorization flow before relying on the oracle interface. The first scaffold is intentionally not a production oracle implementation.
