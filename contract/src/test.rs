#![cfg(test)]
use soroban_sdk::{Env, Address};
use crate::{BaonLockContract, BaonLockContractClient};

#[test]
fn happy_path() {
    let env = Env::default();
    let id = env.register_contract(None, BaonLockContract);
    let c = BaonLockContractClient::new(&env, &id);

    let parent = Address::random(&env);
    let student = Address::random(&env);

    c.deposit(&parent, &student, &1000, &200);
    c.claim(&student);

    assert_eq!(c.get(&student).remaining, 800);
}

#[test]
#[should_panic]
fn over_claim() {
    let env = Env::default();
    let id = env.register_contract(None, BaonLockContract);
    let c = BaonLockContractClient::new(&env, &id);

    let parent = Address::random(&env);
    let student = Address::random(&env);

    c.deposit(&parent, &student, &100, &200);
    c.claim(&student);
}

#[test]
fn state_check() {
    let env = Env::default();
    let id = env.register_contract(None, BaonLockContract);
    let c = BaonLockContractClient::new(&env, &id);

    let parent = Address::random(&env);
    let student = Address::random(&env);

    c.deposit(&parent, &student, &1000, &200);

    assert_eq!(c.get(&student).total, 1000);
}

#[test]
fn multiple_claims() {
    let env = Env::default();
    let id = env.register_contract(None, BaonLockContract);
    let c = BaonLockContractClient::new(&env, &id);

    let parent = Address::random(&env);
    let student = Address::random(&env);

    c.deposit(&parent, &student, &1000, &200);
    c.claim(&student);
    c.claim(&student);

    assert_eq!(c.get(&student).remaining, 600);
}

#[test]
fn full_spend() {
    let env = Env::default();
    let id = env.register_contract(None, BaonLockContract);
    let c = BaonLockContractClient::new(&env, &id);

    let parent = Address::random(&env);
    let student = Address::random(&env);

    c.deposit(&parent, &student, &400, &200);
    c.claim(&student);
    c.claim(&student);

    assert_eq!(c.get(&student).remaining, 0);
}