#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn ping(_env: Env) -> Symbol {
        Symbol::short("OK")
    }
}