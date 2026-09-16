# Security policy

This repository holds the four Soroban contracts backing Lifelyn's consent, provider-verification, record-attestation, and access-receipt state. It is **unaudited**. Contracts are deployed to **Stellar Testnet only** — do not deploy to Mainnet or connect a Mainnet contract to real medical workloads before an independent Soroban/security audit.

## Scope

`ConsentRegistry`, `ProviderRegistry`, `RecordAttestationRegistry`, `AccessReceiptRegistry`, and the generated TypeScript bindings in `bindings/`. State and events must never contain readable medical content, names, document locations, or queries — only opaque 32-byte references, addresses, timestamps, booleans, and hashes. Any finding where a contract leaks or could be made to leak more than that is high severity.

## Reporting a vulnerability

Please **do not** open a public GitHub issue for a security finding.

- Preferred: use this repository's [GitHub Security Advisories](https://github.com/Lifelyn456/Lifelyn-contracts/security/advisories/new) ("Report a vulnerability" under the Security tab).
- Alternative: contact **@precious1joe** on Telegram with a clear description, reproduction steps, and impact.

We aim to acknowledge reports within 5 business days.

## What's in scope

- Missing or incorrect `require_auth()` on any state-changing method
- Consent expiry/revocation not enforced by `is_active`
- Non-immutability of record attestations
- Provider authority check bypass
- Ledger-time manipulation (a caller forging `now`)

## What's out of scope

- Findings that assume a compromised deployer or signer identity
- Gas/fee optimization suggestions with no security impact (file those as a regular issue)
