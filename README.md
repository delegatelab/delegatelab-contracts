# DelegateLab — Disposable CAP-71 fixture contracts

Open-source adversarial testing for Stellar delegated-account authorization. This repository owns the contracts component.

Run `cargo test --locked`, `cargo test --locked --features vulnerable` and `node build.mjs`. Strict and vulnerable builds share one source; sink is a separate feature. See TESTNET.md in the backend for live evidence. The unit tests exercise context policy; signed enforce-mode testnet checks exercise actual host delegation.

Read [INTEGRATION.md](INTEGRATION.md) to run the connected workspace and [ROADMAP.md](ROADMAP.md) for the exact remaining scope. All five repositories: [engine](https://github.com/delegatelab/delegatelab-engine), [contracts](https://github.com/delegatelab/delegatelab-contracts), [backend](https://github.com/delegatelab/delegatelab-backend), [frontend](https://github.com/delegatelab/delegatelab-frontend), [fixtures](https://github.com/delegatelab/delegatelab-fixtures).

MIT licensed. Early fixture-focused testnet MVP, not a production wallet or security certification.
