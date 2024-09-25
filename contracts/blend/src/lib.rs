#![no_std]

use soroban_sdk::{contract, contractimpl, vec, Map, Address, Env, String};

mod ipool {
    use super::*;
    use soroban_sdk::{contractclient, contracttype, Vec};

    #[contracttype]
    pub struct Request { 
        pub address: Address, 
        pub amount: i128, 
        pub request_type: u32 
    }

    #[contracttype]
    pub struct Positions { 
        collateral: Map<u32, i128>, 
        liabilities: Map<u32, i128>, 
        supply: Map<u32, i128> 
    }

    #[contractclient(name = "IPoolClient")]
    #[allow(dead_code)]
    pub trait IPool {
        fn submit(
            from: Address,
            spender: Address,
            to: Address,
            requests: Vec<Request>,
        ) -> Positions;
    }
}

use ipool::IPoolClient;

#[contract]
pub struct Deposit;

#[contractimpl]
impl Deposit {
    pub fn run(env: Env, invoker: Address) -> i128 {
        let pool_address_str = String::from_str(&env,
            "CDWCIQMI3EHKGK32SVICLUBFVBT5VKNG72O3G7KNJOLSIVA4WDVQ2IYX");
        let token_address_str = String::from_str(&env,
            "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
        let pool_address = Address::from_string(&pool_address_str);
        let token_address = Address::from_string(&token_address_str);

        let amount = 10 * 10i128.pow(6);
        let pool = IPoolClient::new(&env, &pool_address);
        let requests = vec![
            &env,
            ipool::Request {
                address: token_address,
                amount,
                request_type: 2u32,
            },
        ];
        pool.submit(&invoker, &invoker, &invoker, &requests);
        amount
    }
}