#![no_std]

use dharitri_sc::imports::*;

use dharitri_sc_modules::pause;

pub mod config;
pub mod contract_interactions;
pub mod events;
pub mod proxy_deployer_proxy;

#[dharitri_sc::contract]
pub trait ProxyDeployer:
    contract_interactions::ContractInteractionsModule
    + config::ConfigModule
    + events::EventsModule
    + pause::PauseModule
{
    #[init]
    fn init(&self, default_gas_for_save: u64) {
        self.default_gas_for_save_operation()
            .set(default_gas_for_save);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
