# Security and trust boundaries

Fixtures are intentionally unsuitable for production. The vulnerable build deliberately accepts amounts above its configured cap; neither fixture should ever hold assets or authorize asset contracts. Use only disposable testnet identities. The HTTP service binds to loopback with Host/Origin gates and optional API_TOKEN. It does not hold a signer or submit transactions. Signing is confined to explicit CLI pilot commands with fresh testnet-only keys in ignored .private/ storage.

Report vulnerabilities privately to the organization owner through their GitHub profile contact; do not publish secrets in issues. No security audit or formal proof is claimed. RPC responses, imported observations and synthetic models have different provenance; inspect the mode label. Testnet resets, TTL expiry and RPC retention can invalidate live reproduction. General wallet policies, nested delegation and replay analysis are not yet supported.
