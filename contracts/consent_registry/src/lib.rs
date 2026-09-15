#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Address, BytesN, Env};
#[contractevent(topics = ["consent_granted"])]
pub struct ConsentGranted {
    #[topic]
    pub grant_ref: BytesN<32>,
}
#[contractevent(topics = ["consent_revoked"])]
pub struct ConsentRevoked {
    #[topic]
    pub grant_ref: BytesN<32>,
}
#[contracttype]
#[derive(Clone)]
pub struct Grant {
    pub grantor: Address,
    pub subject: BytesN<32>,
    pub recipient: Address,
    pub scope_hash: BytesN<32>,
    pub starts_at: u64,
    pub expires_at: u64,
    pub revoked: bool,
}
#[contract]
pub struct ConsentRegistry;
#[contractimpl]
impl ConsentRegistry {
    // Soroban's calling convention requires each scalar field individually (the generated
    // TypeScript/CLI bindings still present this as a single named-args object to callers),
    // so the extra parameter count here does not leak into ergonomics for `lifelyn-api` or
    // any other consumer. A plain `//` comment is used deliberately: `///` doc comments on a
    // `#[contractimpl]` method are captured into the on-chain contract spec and would leak
    // this internal implementation note into every generated binding.
    #[allow(clippy::too_many_arguments)]
    pub fn grant(
        env: Env,
        grant_ref: BytesN<32>,
        grantor: Address,
        subject: BytesN<32>,
        recipient: Address,
        scope_hash: BytesN<32>,
        starts_at: u64,
        expires_at: u64,
    ) {
        grantor.require_auth();
        assert!(
            expires_at > starts_at && expires_at > env.ledger().timestamp(),
            "invalid expiry"
        );
        assert!(!env.storage().persistent().has(&grant_ref), "grant exists");
        env.storage().persistent().set(
            &grant_ref,
            &Grant {
                grantor,
                subject,
                recipient,
                scope_hash,
                starts_at,
                expires_at,
                revoked: false,
            },
        );
        ConsentGranted { grant_ref }.publish(&env);
    }
    pub fn revoke(env: Env, grant_ref: BytesN<32>) {
        let mut grant: Grant = env
            .storage()
            .persistent()
            .get(&grant_ref)
            .expect("missing grant");
        grant.grantor.require_auth();
        grant.revoked = true;
        env.storage().persistent().set(&grant_ref, &grant);
        ConsentRevoked { grant_ref }.publish(&env);
    }
    pub fn get(env: Env, grant_ref: BytesN<32>) -> Option<Grant> {
        env.storage().persistent().get(&grant_ref)
    }
    pub fn is_active(env: Env, grant_ref: BytesN<32>) -> bool {
        let now = env.ledger().timestamp();
        Self::get(env, grant_ref)
            .map(|g| !g.revoked && g.starts_at <= now && g.expires_at > now)
            .unwrap_or(false)
    }
}
#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Events as _, Ledger},
        Event,
    };
    #[test]
    fn active_and_revoked() {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().with_mut(|l| l.timestamp = 100);
        let id = env.register(ConsentRegistry, ());
        let client = ConsentRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.grant(
            &reference,
            &Address::generate(&env),
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[3; 32]),
            &100,
            &200,
        );
        assert_eq!(
            env.events().all(),
            std::vec![ConsentGranted {
                grant_ref: reference.clone()
            }
            .to_xdr(&env, &id)]
        );
        assert!(client.is_active(&reference));
        client.revoke(&reference);
        // `env.events().all()` reflects only the most recent top-level contract invocation
        // (each `client.*()` call is its own invocation), not a cumulative log across every
        // call made so far in the test — so this only asserts the event from `revoke`, not
        // the earlier `grant` event which was already asserted above.
        assert_eq!(
            env.events().all(),
            std::vec![ConsentRevoked {
                grant_ref: reference.clone()
            }
            .to_xdr(&env, &id)]
        );
        assert!(!client.is_active(&reference));
    }
    #[test]
    #[should_panic]
    fn unauthorized_grant_rejected() {
        let env = Env::default();
        let id = env.register(ConsentRegistry, ());
        ConsentRegistryClient::new(&env, &id).grant(
            &BytesN::from_array(&env, &[1; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[3; 32]),
            &0,
            &200,
        );
    }
    #[test]
    fn exact_expiry_is_inactive() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(ConsentRegistry, ());
        let client = ConsentRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.grant(
            &reference,
            &Address::generate(&env),
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[3; 32]),
            &0,
            &200,
        );
        env.ledger().with_mut(|l| l.timestamp = 200);
        assert!(!client.is_active(&reference));
    }
    #[test]
    #[should_panic]
    fn unauthorized_revoke_rejected() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(ConsentRegistry, ());
        let client = ConsentRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.grant(
            &reference,
            &Address::generate(&env),
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[3; 32]),
            &0,
            &200,
        );
        env.mock_auths(&[]);
        client.revoke(&reference);
    }
    #[test]
    #[should_panic]
    fn invalid_expiry_rejected() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(ConsentRegistry, ());
        ConsentRegistryClient::new(&env, &id).grant(
            &BytesN::from_array(&env, &[1; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[2; 32]),
            &Address::generate(&env),
            &BytesN::from_array(&env, &[3; 32]),
            &200,
            &100,
        );
    }
}
