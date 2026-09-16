<p align="center">
  <img src="https://raw.githubusercontent.com/Lifelyn456/lifelyn-web/main/public/logo.png" alt="Lifelyn" width="120" />
</p>

<h1 align="center">Lifelyn Contracts</h1>
<p align="center"><strong>Opaque proof on-chain, readable evidence nowhere but where it's authorized.</strong></p>

<p align="center">
  <a href="https://github.com/Lifelyn456/Lifelyn-contracts/actions/workflows/ci.yml"><img src="https://github.com/Lifelyn456/Lifelyn-contracts/actions/workflows/ci.yml/badge.svg" alt="Contract checks" /></a>
  <img src="https://img.shields.io/badge/stack-Rust%20%2F%20Soroban-DE7A22" alt="Rust / Soroban" />
  <img src="https://img.shields.io/badge/network-Stellar%20Testnet-08B5E5" alt="Stellar Testnet" />
  <img src="https://img.shields.io/badge/license-unlicensed-lightgrey" alt="Unlicensed" />
</p>

<p align="center">📖 <a href="https://cjay-1.gitbook.io/lifelyn-docs/">Documentation</a></p>

One Rust/Soroban workspace holding the four contracts that anchor [Lifelyn](https://github.com/Lifelyn456/lifelyn-web)'s trust layer: consent, provider status, immutable record attestation, and access receipts. State and typed events contain **only** opaque 32-byte references, addresses, timestamps, booleans, and hashes — never medical content, readable consent categories, names, document locations, or queries.

Every state-changing method requires explicit Soroban authorization (`require_auth()`). Consent validates its window and uses **ledger time** for `is_active`, so a caller can never forge `now`. Access receipts likewise record ledger time rather than a caller-controlled timestamp. Provider registration is restricted to the constructor-set authority and verifies the authority argument. Record attestations are immutable per opaque record/version reference.

## Table of contents

- [Maintainers](#maintainers)
- [Contracts](#contracts)
- [Deployed addresses (Testnet)](#deployed-addresses-testnet)
- [Build and bindings](#build-and-bindings)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [Contributors](#contributors)

## Maintainers

| | Name | Role | Contact |
| --- | --- | --- | --- |
| 🧑‍💻 | Chijioke | Maintainer | [@precious1joe](https://t.me/precious1joe) on Telegram · [@Cjay-Cyber-2](https://github.com/Cjay-Cyber-2) on GitHub |

## Contracts

| Contract | Responsibility |
| --- | --- |
| `ConsentRegistry` | Scoped, time-windowed patient consent; grant/revoke; `is_active` uses ledger time |
| `ProviderRegistry` | Provider verification status, gated by a constructor-set authority address |
| `RecordAttestationRegistry` | Immutable content-hash attestation per opaque record/version reference |
| `AccessReceiptRegistry` | Append-only access receipts, recorded by the authorized recorder address |

## Deployed addresses (Testnet)

| Contract | Address |
| --- | --- |
| `ConsentRegistry` | `CCLKF6V5NV5KWIZMIYN4YSJXWCUK7PFYIRXSRVO7OM2AO2HDVCFXFCED` |
| `ProviderRegistry` | `CBSULDGXAYGMMME5X47TIAJZ3IKTTRNR2Y4KSCZKKOPY4E37BY6IM57U` |
| `RecordAttestationRegistry` | `CDDZTXZBRZIC6BAWOZO4MADR7MG7WO7CJFYS2M44C2426A34FQKEZ52Y` |
| `AccessReceiptRegistry` | `CBXGHZKUKRPRKYWN2TNFOPC7XVGF2IMOVZFATYR4JABYUPSXW4SEJHHJ` |

View any of these on [Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet). These are Testnet-only; Mainnet deployment requires an independent Soroban/security review first.

## Build and bindings

Install Rust stable, the `wasm32v1-none` target, and Stellar CLI 27. Then:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
stellar contract build
pwsh ./scripts/generate-bindings.ps1
```

Generated TypeScript packages are committed under `bindings/`; CI regenerates them and rejects a diff. Host tests require a working native linker and run authoritatively on Linux CI — on some Windows hosts, native `cargo test` can fail to *link* (`export ordinal too large`, a MinGW limit on this dependency tree); that's an environment limitation, not a code defect, and Linux CI is authoritative.

Linux CI also starts the official Stellar Quickstart local network, deploys all four built contracts with a generated test-only identity, grants and revokes consent through the deployed contract, and asserts the live on-chain state transition. The smoke script is `scripts/local-network-smoke.ps1` and requires Docker plus Stellar CLI 27.

## Deployment

Copy `.env.example` to a local environment source and configure named Stellar CLI identities/addresses. `scripts/deploy.ps1` rejects raw seed-like identity input, builds and deploys all four contracts, writes an ignored `contract-ids.<network>.json`, and regenerates bindings. It never writes or requests a secret seed in source control.

Do not deploy to Mainnet or connect real medical workloads before an independent Soroban/security review. Testnet deployment and transaction confirmations are operational acceptance steps and are not claimed merely because the Wasm builds.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). Found a security issue? See [`SECURITY.md`](SECURITY.md) instead of opening a public issue.

## Contributors

<a href="https://github.com/Lifelyn456/Lifelyn-contracts/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Lifelyn456/Lifelyn-contracts" alt="Contributors" />
</a>
