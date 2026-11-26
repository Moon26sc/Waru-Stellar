# Web application boundary

Frontend framework intentionally not selected in this foundation. Keep UI separate from contract logic.

Required screens:
1. Sponsor dashboard: vault balance, funding receipts, active program policy.
2. Farmer dashboard: claims, measurement periods, reward status, payout history.
3. Claim detail: source type, baseline, observed volume, units, method version, evidence hash.
4. Wallet: connect, network check, transaction signing.
5. Receipt: contract, asset, transaction hash, ledger and explorer link.

Rules:
- Simulator data must carry a visible `SIMULATED` label.
- Never display a hardcoded success state or invented balance.
- Handle pending, failed, rejected, expired and settled states distinctly.
- Do not place oracle secret keys in frontend environment variables.
