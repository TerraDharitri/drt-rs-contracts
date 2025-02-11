#![no_std]

use dharitri_sc::imports::*;

use dharitri_sc_modules::pause;

#[dharitri_sc::contract]
pub trait CheckPauseContract: pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[endpoint(checkPause)]
    fn check_pause(&self) -> bool {
        self.is_paused()
    }
}
