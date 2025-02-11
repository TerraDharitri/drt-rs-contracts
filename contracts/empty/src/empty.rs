#![no_std]

#[allow(unused_imports)]
use dharitri_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[dharitri_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
