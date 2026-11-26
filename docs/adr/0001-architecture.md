# ADR 0001 — Minimal oracle-attested Soroban settlement

- Status: Proposed
- Date: 2026-11-26

## Context
Waru needs to convert eligible agricultural observations into small, auditable payments. Physical telemetry and agronomic interpretation are not natively available to a smart contract.

## Decision
Keep detailed telemetry and baseline computation off-chain. A dedicated oracle account attests to a validated claim. Soroban enforces authorization, uniqueness, reward caps and token transfer. The frontend reads public receipts and uses a Stellar wallet for user-authorized operations.

## Consequences
Positive:
- Small on-chain footprint.
- Private farm data stays off-chain.
- Deterministic settlement rules.
- Easy to demonstrate on Testnet.

Trade-offs:
- Oracle is a trusted authority.
- Contract cannot prove sensor truth.
- Agronomic model requires independent validation.
- Production needs key governance, dispute handling and audit.

## Rejected for MVP
- Storing raw telemetry on-chain.
- Claiming decentralization of sensor data.
- Building custom IoT hardware before validating the incentive loop.
- Mainnet deployment before pilot and security review.
