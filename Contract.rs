#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Map};

#[contracttype]
#[derive(Clone)]
pub struct VoteData {
    pub option: Symbol,
    pub count: u32,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Initialize voting options
    pub fn initialize(env: Env, options: Vec<Symbol>) {
        let mut votes: Map<Symbol, u32> = Map::new(&env);

        for option in options.iter() {
            votes.set(option.clone(), 0);
        }

        env.storage().instance().set(&Symbol::new(&env, "votes"), &votes);
    }

    // Cast a vote
    pub fn vote(env: Env, option: Symbol) {
        let key = Symbol::new(&env, "votes");
        let mut votes: Map<Symbol, u32> = env.storage().instance().get(&key).unwrap();

        let count = votes.get(option.clone()).unwrap_or(0);
        votes.set(option, count + 1);

        env.storage().instance().set(&key, &votes);
    }

    // Get results
    pub fn get_results(env: Env) -> Vec<VoteData> {
        let key = Symbol::new(&env, "votes");
        let votes: Map<Symbol, u32> = env.storage().instance().get(&key).unwrap();

        let mut results = Vec::new(&env);

        for (option, count) in votes.iter() {
            results.push_back(VoteData {
                option,
                count,
            });
        }

        results
    }
}
