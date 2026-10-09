#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Address, BytesN, Env};
// Soroban archives a persistent entry when its TTL runs out, and reads of an archived entry fail
// until someone restores it. A new entry lives only min_persistent_ttl ledgers (about 7 days on
// Testnet), so every write renews the TTL. Renew once under ~30 days remain and extend to ~150
// days (17,280 ledgers a day), below the network's 3,110,400-ledger ceiling.
const TTL_THRESHOLD: u32 = 518_400;
const TTL_EXTEND_TO: u32 = 2_592_000;
#[contractevent(topics = ["record_attested"])]
pub struct RecordAttested {
    #[topic]
    pub record_ref: BytesN<32>,
    #[topic]
    pub issuer: Address,
    pub content_hash: BytesN<32>,
}
#[contracttype]
#[derive(Clone)]
pub struct Attestation {
    pub content_hash: BytesN<32>,
    pub issuer: Address,
}
#[contract]
pub struct RecordAttestationRegistry;
#[contractimpl]
impl RecordAttestationRegistry {
    pub fn attest(env: Env, record_ref: BytesN<32>, content_hash: BytesN<32>, issuer: Address) {
        issuer.require_auth();
        assert!(
            !env.storage().persistent().has(&record_ref),
            "immutable attestation"
        );
        env.storage().persistent().set(
            &record_ref,
            &Attestation {
                content_hash: content_hash.clone(),
                issuer: issuer.clone(),
            },
        );
        env.storage()
            .persistent()
            .extend_ttl(&record_ref, TTL_THRESHOLD, TTL_EXTEND_TO);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        RecordAttested {
            record_ref,
            issuer,
            content_hash,
        }
        .publish(&env);
    }
    pub fn get(env: Env, record_ref: BytesN<32>) -> Option<Attestation> {
        env.storage().persistent().get(&record_ref)
    }
}
#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use soroban_sdk::testutils::storage::{Instance as _, Persistent as _};
    use soroban_sdk::{
        testutils::{Address as _, Events as _},
        Event,
    };
    #[test]
    fn attestation_round_trip() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(RecordAttestationRegistry, ());
        let client = RecordAttestationRegistryClient::new(&env, &id);
        let issuer = Address::generate(&env);
        let reference = BytesN::from_array(&env, &[1; 32]);
        let hash = BytesN::from_array(&env, &[2; 32]);
        client.attest(&reference, &hash, &issuer);
        // `env.events().all()` reflects only the most recent top-level invocation, so this
        // must be asserted before the read-only `get` call below (which is itself a separate
        // invocation with no events) replaces it with an empty list.
        assert_eq!(
            env.events().all(),
            std::vec![RecordAttested {
                record_ref: reference.clone(),
                issuer: issuer.clone(),
                content_hash: hash.clone()
            }
            .to_xdr(&env, &id)]
        );
        assert_eq!(client.get(&reference).unwrap().content_hash, hash);
    }
    #[test]
    #[should_panic]
    fn duplicate_cannot_replace_hash() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(RecordAttestationRegistry, ());
        let client = RecordAttestationRegistryClient::new(&env, &id);
        let issuer = Address::generate(&env);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.attest(&reference, &BytesN::from_array(&env, &[2; 32]), &issuer);
        client.attest(&reference, &BytesN::from_array(&env, &[3; 32]), &issuer);
    }
    #[test]
    #[should_panic]
    fn unauthorized_attestation() {
        let env = Env::default();
        let id = env.register(RecordAttestationRegistry, ());
        RecordAttestationRegistryClient::new(&env, &id).attest(
            &BytesN::from_array(&env, &[1; 32]),
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
        );
    }
    #[test]
    fn attestation_extends_storage_ttl() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(RecordAttestationRegistry, ());
        let client = RecordAttestationRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.attest(
            &reference,
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
        );
        let ttl = env.as_contract(&id, || env.storage().persistent().get_ttl(&reference));
        let instance_ttl = env.as_contract(&id, || env.storage().instance().get_ttl());
        assert_eq!(ttl, TTL_EXTEND_TO);
        assert_eq!(instance_ttl, TTL_EXTEND_TO);
    }
}
