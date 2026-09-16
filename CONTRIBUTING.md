# Contributing to Lifelyn Contracts

Thanks for looking at this. A few ground rules before you send a PR.

## Ground rules

- State and events carry **opaque references, hashes, addresses, timestamps, and booleans only** — never readable medical content, names, or document locations.
- Every state-changing method needs an explicit `require_auth()`, and — where applicable — expiry/revocation checks and immutability guarantees. Add a negative test for each.
- Testnet only. Never target Mainnet from a PR in this repo.

## Getting set up

Install Rust stable, the `wasm32v1-none` target, and Stellar CLI 27:

```powershell
rustup update stable
rustup target add wasm32v1-none
cargo install stellar-cli --version 27.1.0 --locked
```

## Before you open a PR

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
stellar contract build
pwsh ./scripts/generate-bindings.ps1
git diff --exit-code -- bindings   # bindings must be regenerated, not hand-edited
```

CI additionally runs a local-network smoke test (`scripts/local-network-smoke.ps1`) that deploys all four contracts and exercises a real grant/revoke cycle. All checks must be green before merge.

> **Windows note**: native `cargo test` can fail to link on some Windows hosts with `x86_64-pc-windows-gnu` (`export ordinal too large`, a MinGW linker limit on this dependency tree). If you hit that, run tests from WSL 2 or rely on CI — it's a host toolchain limitation, not a code issue.

## Commit style

Conventional commits: `type(scope): description`. One logical change per commit.

## Deployment

Never commit a deployer seed. `scripts/deploy.ps1` accepts only a named Stellar CLI identity (rejects raw seed input) and writes contract IDs to a gitignored `contract-ids.<network>.json`.

## Reporting bugs vs. security issues

Regular bugs: open a GitHub issue. Security vulnerabilities: see `SECURITY.md` — do not file those as public issues.
