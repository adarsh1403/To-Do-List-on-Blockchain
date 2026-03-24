#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec, String};

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // Add a new task
    pub fn add_task(env: Env, task: String) {
        let key = Symbol::short("TASKS");

        let mut tasks: Vec<String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        tasks.push_back(task);

        env.storage().instance().set(&key, &tasks);
    }

    // Get all tasks
    pub fn get_tasks(env: Env) -> Vec<String> {
        let key = Symbol::short("TASKS");

        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }

    // Remove a task by index
    pub fn remove_task(env: Env, index: u32) {
        let key = Symbol::short("TASKS");

        let mut tasks: Vec<String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        if index < tasks.len() {
            tasks.remove(index);
        }

        env.storage().instance().set(&key, &tasks);
    }

    // Clear all tasks
    pub fn clear_tasks(env: Env) {
        let key = Symbol::short("TASKS");
        let empty: Vec<String> = Vec::new(&env);

        env.storage().instance().set(&key, &empty);
    }
}