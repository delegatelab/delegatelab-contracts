# DelegateLab connected workspace

Clone all five repositories as siblings. Use Node.js 24+ (built-in SQLite is required), Rust stable and official Stellar CLI 28.1.0 for contract work.

```bash
mkdir delegatelab-workspace
cd delegatelab-workspace
for repo in engine contracts backend frontend fixtures; do
  git clone https://github.com/delegatelab/delegatelab-$repo.git
done
cd delegatelab-backend
npm ci
cd ../delegatelab-frontend
npm run dev
```

Open http://127.0.0.1:4330. API: http://127.0.0.1:4331. Run the local model with the cap check enabled and then omitted; inspect saved history and captured testnet evidence. Model runs never sign or contact Stellar. Ctrl-C stops both development processes. For customized ports, run services separately with their .env.example settings; dev.mjs uses fixed ports and does not load .env files.

## Architecture

Frontend → loopback proxy → backend → engine. Backend uses SQLite for atomic report persistence and the official Stellar SDK for signed testnet runner/diagnostic decoding. Fixtures import the engine. Contracts provide two builds of a custom account and one intent-recording sink. They transfer no assets.

Sibling imports are explicit and CI checks out required siblings. A release must pin sibling commit SHAs; current development CI follows main. No production deployment or multi-tenant authentication is provided. Optional API_TOKEN is injected by the frontend proxy, never browser JavaScript. Keep data/, .env and .private/ out of Git.

## What is actually supported

Six deterministic cases: at-cap, over-cap, wrong recipient, wrong contract, zero and negative amounts. The strict account has one configured delegate and checks all authorized contexts; the vulnerable build intentionally omits only the amount-cap check. CAP-71 host delegation authenticates the delegate. Reports distinguish unexpected acceptance, unexpected rejection and inconclusive infrastructure failures. Diagnostic decoding requires the expected custom account and exact contract error code.

This is a fixture-focused MVP, not an arbitrary-wallet analyzer. Function mutation, expiration/nonce replay cases, nested delegation, arbitrary ABI adapters and additional wallet integrations remain open roadmap work. Test results are finite evidence, not a proof of security. Simulations and finalized transactions are labeled separately. RPC data is trusted through TLS, not independently verified consensus.
