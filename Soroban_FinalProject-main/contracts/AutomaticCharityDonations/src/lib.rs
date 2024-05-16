#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

#[derive(Clone)]
#[contracttype]
pub struct Account {
    balance: u64,
}

#[contract]
pub struct MicroPaymentContract;


#[contractimpl]
impl MicroPaymentContract {
    pub fn deposit(env: Env, from: Address, token: Address, amount: i128) {
        // Ensure that the transaction sender is the contract owner
        from.require_auth();

        // Transfer tokens from sender to the contract
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);

        // Update the sender's balance in the contract
        let mut account: Account = env.storage().instance().get(&from).unwrap();
        account.balance += amount as u64;
        env.storage().instance().set(&from, &account);
    }

    pub fn withdraw(env: Env, from: Address, token: Address, amount: i128) {
        // Ensure that the transaction sender is the owner of the account
        from.require_auth();

        // Check if the sender has sufficient balance
        let account: Account = env.storage().instance().get(&from).unwrap();
        if account.balance < amount as u64 {
            panic!("Insufficient balance");
        }

        // Transfer tokens from the contract to the sender
        token::Client::new(&env, &token).transfer(
            &env.current_contract_address(),
            &from,
            &amount,
        );

        // Update the sender's balance in the contract
        let mut updated_account = account;
        updated_account.balance -= amount as u64;
        env.storage().instance().set(&from, &updated_account);
    }
}
