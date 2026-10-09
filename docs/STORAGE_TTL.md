# Storage lifetime (TTL)

Soroban does not keep contract data forever. Every ledger entry has a time to live (TTL), counted in ledgers. When it runs out the entry is **archived**: it is not deleted, but reads fail until someone submits a restore transaction. This applies to each contract's persistent entries, to the contract instance, and to its Wasm code.

## What the network allows

Read from Stellar Testnet on 9 October 2026 (`ConfigSetting` `state_archival`):

| Setting | Ledgers | About (5 s per ledger) |
| --- | --- | --- |
| `min_persistent_ttl` (a new entry starts with this) | 120,960 | 7 days |
| `max_entry_ttl` (the ceiling for any extension) | 3,110,400 | 180 days |

## The problem this fixed

Before this change none of the four contracts extended anything. A grant, provider registration, attestation or receipt written on Testnet lived about **7 days**, and so did the contract instance itself. After that, `get` and `is_active` failed until a restore, and a consent that should still be active would read as unreadable.

This was observed, not just predicted. The contracts deployed on 16 September 2026 had expired by 9 October: calling `is_active` on the September `ConsentRegistry` made the CLI submit a restore transaction first, instead of running a read-only simulation.

## What the contracts do now

Every state-changing call renews the entries it touches:

```text
if remaining TTL < 518,400 ledgers (about 30 days)  ->  extend to 2,592,000 ledgers (about 150 days)
```

| Contract | Call | Renewed |
| --- | --- | --- |
| `ConsentRegistry` | `grant`, `revoke` | the grant entry, and the instance and code |
| `ProviderRegistry` | `register`, `set_status` | the provider entry, and the instance and code (which holds the authority) |
| `RecordAttestationRegistry` | `attest` | the attestation entry, and the instance and code |
| `AccessReceiptRegistry` | `record_receipt` | the receipt entry, and the instance and code (which holds the recorder) |

`instance().extend_ttl` in the Soroban SDK renews both the contract instance and its Wasm code. The 150-day target sits below the network's 180-day ceiling, so the extension is never refused.

### Verified on Testnet

Fresh copies of the contracts were deployed and written to on 9 October 2026, and the real expiry ledgers were read back from the network:

| | Lifetime after the write |
| --- | --- |
| Grant, attestation and provider entries | exactly 2,592,000 ledgers |
| Contract instances | about 2,592,000 ledgers |
| Before this change (contract instance) | 120,959 ledgers |

Unit tests assert the TTL after every write (`*_extends_storage_ttl` in each contract).

## Limits and operations

- **Only writes renew.** An entry that nothing writes to for 150 days will still be archived. A long-lived consent grant is renewed when it is revoked, but not while it sits unchanged. A permissionless "keep alive" call, or a scheduled `ExtendFootprintTTL` operation, would close this gap. It is tracked as an open issue.
- **Archived does not mean lost.** An archived entry can be brought back with a `RestoreFootprint` transaction (the Stellar CLI does this automatically when it detects one). Its data is unchanged.
- **Already-deployed contracts are not upgraded.** The contracts have no upgrade path, by design. To get this behaviour, deploy the new Wasm and update `STELLAR_*_CONTRACT_ID` in the API and the signer. Old entries stay on the old contracts.
- **Mainnet values differ.** Read the live `state_archival` settings before choosing different constants.
