#![no_std]
use soroban_sdk::{
    contract, contractevent, contractimpl, contracttype, symbol_short, Address, BytesN, Env,
};
// Soroban archives a persistent entry when its TTL runs out, and reads of an archived entry fail
// until someone restores it. A new entry lives only min_persistent_ttl ledgers (about 7 days on
// Testnet), so every write renews the TTL. Renew once under ~30 days remain and extend to ~150
// days (17,280 ledgers a day), below the network's 3,110,400-ledger ceiling.
const TTL_THRESHOLD: u32 = 518_400;
const TTL_EXTEND_TO: u32 = 2_592_000;
#[contractevent(topics = ["provider_status"])]
pub struct ProviderStatus {
    #[topic]
    pub provider_ref: BytesN<32>,
    pub verified: bool,
}
#[contracttype]
#[derive(Clone)]
pub struct Provider {
    pub metadata_hash: BytesN<32>,
    pub verified: bool,
}
#[contract]
pub struct ProviderRegistry;
#[contractimpl]
impl ProviderRegistry {
    pub fn __constructor(env: Env, authority: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("authority"), &authority);
    }
    pub fn register(
        env: Env,
        provider_ref: BytesN<32>,
        authority: Address,
        metadata_hash: BytesN<32>,
    ) {
        let configured_authority: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("authority"))
            .unwrap();
        assert!(authority == configured_authority, "invalid authority");
        authority.require_auth();
        assert!(
            !env.storage().persistent().has(&provider_ref),
            "provider exists"
        );
        env.storage().persistent().set(
            &provider_ref,
            &Provider {
                metadata_hash,
                verified: false,
            },
        );
        Self::renew(&env, &provider_ref);
    }
    pub fn set_status(env: Env, provider_ref: BytesN<32>, verified: bool) {
        let authority: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("authority"))
            .unwrap();
        authority.require_auth();
        let mut provider: Provider = env
            .storage()
            .persistent()
            .get(&provider_ref)
            .expect("missing provider");
        provider.verified = verified;
        env.storage().persistent().set(&provider_ref, &provider);
        Self::renew(&env, &provider_ref);
        ProviderStatus {
            provider_ref,
            verified,
        }
        .publish(&env);
    }
    // Keeps both the provider record and this contract's instance storage (which holds the
    // authority) alive. A plain `//` comment is used deliberately: `///` on a
    // `#[contractimpl]` method would be captured into the contract spec.
    fn renew(env: &Env, provider_ref: &BytesN<32>) {
        env.storage()
            .persistent()
            .extend_ttl(provider_ref, TTL_THRESHOLD, TTL_EXTEND_TO);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
    }
    pub fn get_status(env: Env, provider_ref: BytesN<32>) -> bool {
        env.storage()
            .persistent()
            .get::<_, Provider>(&provider_ref)
            .map(|p| p.verified)
            .unwrap_or(false)
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
    fn provider_starts_unverified() {
        let env = Env::default();
        env.mock_all_auths();
        let authority = Address::generate(&env);
        let id = env.register(ProviderRegistry, (authority.clone(),));
        let client = ProviderRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.register(&reference, &authority, &BytesN::from_array(&env, &[2; 32]));
        assert!(!client.get_status(&reference));
        client.set_status(&reference, &true);
        // `env.events().all()` reflects only the most recent top-level invocation, so this
        // must be asserted before the read-only `get_status` call below (which is itself a
        // separate invocation with no events) replaces it with an empty list.
        assert_eq!(
            env.events().all(),
            std::vec![ProviderStatus {
                provider_ref: reference.clone(),
                verified: true
            }
            .to_xdr(&env, &id)]
        );
        assert!(client.get_status(&reference));
    }
    #[test]
    #[should_panic]
    fn unauthorized_registration() {
        let env = Env::default();
        let authority = Address::generate(&env);
        let id = env.register(ProviderRegistry, (authority.clone(),));
        ProviderRegistryClient::new(&env, &id).register(
            &BytesN::from_array(&env, &[1; 32]),
            &authority,
            &BytesN::from_array(&env, &[2; 32]),
        );
    }
    #[test]
    #[should_panic]
    fn unauthorized_status_change() {
        let env = Env::default();
        env.mock_all_auths();
        let authority = Address::generate(&env);
        let id = env.register(ProviderRegistry, (authority.clone(),));
        let client = ProviderRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.register(&reference, &authority, &BytesN::from_array(&env, &[2; 32]));
        env.mock_auths(&[]);
        client.set_status(&reference, &true);
    }
    #[test]
    fn registration_and_status_changes_extend_storage_ttl() {
        let env = Env::default();
        env.mock_all_auths();
        let authority = Address::generate(&env);
        let id = env.register(ProviderRegistry, (authority.clone(),));
        let client = ProviderRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        let entry_ttl = || env.as_contract(&id, || env.storage().persistent().get_ttl(&reference));
        let instance_ttl = || env.as_contract(&id, || env.storage().instance().get_ttl());
        client.register(&reference, &authority, &BytesN::from_array(&env, &[2; 32]));
        assert_eq!(entry_ttl(), TTL_EXTEND_TO);
        assert_eq!(instance_ttl(), TTL_EXTEND_TO);
        client.set_status(&reference, &true);
        assert!(entry_ttl() >= TTL_THRESHOLD);
        assert!(instance_ttl() >= TTL_THRESHOLD);
    }
}
