#![no_std]
use soroban_sdk::{Env, Symbol};

pub struct KeyValueStorage;

impl KeyValueStorage {
    pub fn set(env: &Env, key: Symbol, value: Symbol) {
        env.storage().instance().set(&key, &value);
    }

    pub fn get(env: &Env, key: &Symbol) -> Option<Symbol> {
        env.storage().instance().get(key)
    }

    pub fn remove(env: &Env, key: &Symbol) {
        env.storage().instance().remove(key);
    }
}