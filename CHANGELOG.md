# Changelog

## Unreleased

### Fixed
- **Storage expiry.** No contract extended any TTL, so on Testnet every grant, provider registration, attestation and receipt (and the contract instance and code) was archived after about 7 days. Every state-changing call now renews the entries it touches to about 150 days. See `docs/STORAGE_TTL.md`. Contracts deployed before this change are not affected by it and must be redeployed.
- CI: the local-network smoke test waits for the friendbot and retries funding, which fixes the intermittent `Account not found` failure.

### Added
- MIT license.
- Storage-TTL unit tests in all four contracts.

## v0.1.0

- Four Soroban contracts (`ConsentRegistry`, `ProviderRegistry`, `RecordAttestationRegistry`, `AccessReceiptRegistry`), generated TypeScript bindings, deployment scripts and CI.
