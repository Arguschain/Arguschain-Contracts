#![no_std]

use soroban_sdk::{contract, contractimpl};
use soroban_sdk::{Address, Env, Symbol};
use soroban_sdk::symbol_short;

#[derive(Clone)]
pub struct RiskScore {
    pub score: u32,
    pub benford_flag: bool,
}

#[contract]
pub struct ArgusChainContract;
