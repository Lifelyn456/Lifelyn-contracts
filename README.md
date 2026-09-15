# Lifelyn Soroban contracts

One Rust/Soroban workspace contains the four PRD contracts: consent, provider status, immutable record attestation, and access receipts. State and typed events contain only opaque 32-byte references, addresses, timestamps, booleans, and hashes—never medical content, readable consent categories, names, document locations, or queries.

Every state-changing method requires explicit Soroban authorization. Consent validates its window and uses ledger time for `is_active` so a caller cannot forge `now`. Access receipts likewise record ledger time rather than accepting a caller-controlled timestamp. Provider registration is restricted to the constructor-set authority and verifies the authority argument. Record attestations are immutable per opaque record/version reference.

## Build and bindings

Install Rust stable, `wasm32v1-none`, and Stellar CLI 27. Then run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
stellar contract build
pwsh ./scripts/generate-bindings.ps1
```

Generated TypeScript packages are committed under `bindings/`; CI regenerates them and rejects a diff. On this machine all four Wasm contracts compile. Host tests require a working native linker and run authoritatively on Linux CI.

Linux CI also starts the official Stellar Quickstart local network, deploys all four built contracts with a generated test-only identity, grants and revokes consent through the deployed contract, and asserts the live on-chain state transition. The smoke script is `scripts/local-network-smoke.ps1` and requires Docker plus Stellar CLI 27.

## Deployment

Copy `.env.example` to a local environment source and configure named Stellar CLI identities/addresses. `scripts/deploy.ps1` rejects raw seed-like identity input, builds and deploys all four contracts, writes an ignored `contract-ids.<network>.json`, and regenerates bindings. It never writes or requests a secret seed in source control.

Do not deploy to Mainnet or connect real medical workloads before an independent Soroban/security review. Testnet deployment and transaction confirmations are operational acceptance steps and are not claimed merely because the Wasm builds.
