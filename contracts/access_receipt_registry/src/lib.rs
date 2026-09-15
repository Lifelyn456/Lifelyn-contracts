#![no_std]
use soroban_sdk::{
    contract, contractevent, contractimpl, contracttype, symbol_short, Address, BytesN, Env,
};
#[contractevent(topics = ["access_recorded"])]
pub struct AccessRecorded {
    #[topic]
    pub access_ref: BytesN<32>,
    pub timestamp: u64,
}
#[contracttype]
#[derive(Clone)]
pub struct Receipt {
    pub grant_ref: BytesN<32>,
    pub purpose_hash: BytesN<32>,
    pub timestamp: u64,
}
#[contract]
pub struct AccessReceiptRegistry;
#[contractimpl]
impl AccessReceiptRegistry {
    pub fn __constructor(env: Env, recorder: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("recorder"), &recorder);
    }
    pub fn record_receipt(
        env: Env,
        grant_ref: BytesN<32>,
        access_ref: BytesN<32>,
        purpose_hash: BytesN<32>,
    ) {
        let recorder: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("recorder"))
            .unwrap();
        recorder.require_auth();
        assert!(
            !env.storage().persistent().has(&access_ref),
            "receipt exists"
        );
        env.storage().persistent().set(
            &access_ref,
            &Receipt {
                grant_ref,
                purpose_hash,
                timestamp: env.ledger().timestamp(),
            },
        );
        AccessRecorded {
            access_ref,
            timestamp: env.ledger().timestamp(),
        }
        .publish(&env);
    }
    pub fn get(env: Env, access_ref: BytesN<32>) -> Option<Receipt> {
        env.storage().persistent().get(&access_ref)
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
    fn records_ledger_time() {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().with_mut(|l| l.timestamp = 123);
        let id = env.register(AccessReceiptRegistry, (Address::generate(&env),));
        let client = AccessReceiptRegistryClient::new(&env, &id);
        let reference = BytesN::from_array(&env, &[1; 32]);
        client.record_receipt(
            &BytesN::from_array(&env, &[2; 32]),
            &reference,
            &BytesN::from_array(&env, &[3; 32]),
        );
        // `env.events().all()` reflects only the most recent top-level invocation, so this
        // must be asserted before the read-only `get` call below (which is itself a separate
        // invocation with no events) replaces it with an empty list.
        assert_eq!(
            env.events().all(),
            std::vec![AccessRecorded {
                access_ref: reference.clone(),
                timestamp: 123
            }
            .to_xdr(&env, &id)]
        );
        assert_eq!(client.get(&reference).unwrap().timestamp, 123);
    }
    #[test]
    #[should_panic]
    fn unauthorized_receipt() {
        let env = Env::default();
        let id = env.register(AccessReceiptRegistry, (Address::generate(&env),));
        AccessReceiptRegistryClient::new(&env, &id).record_receipt(
            &BytesN::from_array(&env, &[1; 32]),
            &BytesN::from_array(&env, &[2; 32]),
            &BytesN::from_array(&env, &[3; 32]),
        );
    }
}
