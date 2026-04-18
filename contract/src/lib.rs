#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contracttype]
pub struct Allowance {
    pub parent: Address,
    pub student: Address,
    pub total: i128,
    pub daily: i128,
    pub remaining: i128,
}

#[contract]
pub struct BaonLockContract;

#[contractimpl]
impl BaonLockContract {

    // Parent deposits allowance
    pub fn deposit(env: Env, parent: Address, student: Address, total: i128, daily: i128) {
        parent.require_auth();

        let data = Allowance {
            parent,
            student: student.clone(),
            total,
            daily,
            remaining: total,
        };

        env.storage().instance().set(&student, &data);
    }

    // Student claims daily allowance
    pub fn claim(env: Env, student: Address) {
        student.require_auth();

        let mut data: Allowance = env.storage().instance().get(&student).unwrap();
        assert!(data.remaining >= data.daily, "No funds");

        data.remaining -= data.daily;

        env.storage().instance().set(&student, &data);
    }

    pub fn get(env: Env, student: Address) -> Allowance {
        env.storage().instance().get(&student).unwrap()
    }
}