# Stellar integration boundary

Responsibilities:
- Resolve network config (Testnet/other) explicitly.
- Connect to a Stellar wallet and request signatures.
- Build transactions from generated contract bindings/spec.
- Submit through supported RPC/client libraries.
- Poll transaction status and read contract state/events.
- Render verified transaction receipts.

Do not:
- Sign with a secret key embedded in browser code.
- Treat a submitted transaction as successful before confirmation.
- hardcode contract IDs or asset addresses before deployment.
- use Horizon as a substitute for Soroban RPC contract invocation.
- report a mock response as Testnet evidence.

Pin SDK versions after selecting the frontend/backend stack and test against the target Stellar network.
