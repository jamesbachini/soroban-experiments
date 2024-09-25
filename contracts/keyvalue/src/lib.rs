#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

mod keyvalue;
use keyvalue::KeyValueStorage;

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn set(env: Env, key: Symbol, value: Symbol) {
        KeyValueStorage::set(&env, key, value);
    }

    pub fn get(env: Env, key: Symbol) -> Option<Symbol> {
        KeyValueStorage::get(&env, &key)
    }

    pub fn remove(env: Env, key: Symbol) {
        KeyValueStorage::remove(&env, &key);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_key_value_operations() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ExampleContract);
        let client = ExampleContractClient::new(&env, &contract_id);

        let key = symbol_short!("test");
        let mut value = symbol_short!("hello");
        
        // Test set and get
        client.set(&key, &value);
        let value2 = client.get(&key);
        assert_eq!(value, value2.expect("t1"));

        // Test update
        value = symbol_short!("goodbye");
        client.set(&key, &value);
        let value3 = client.get(&key);
        assert_eq!(value, value3.expect("t2"));

        // Test remove
        client.remove(&key);
        let removed_value = client.get(&key);
        assert_eq!(removed_value, None);
    }
}