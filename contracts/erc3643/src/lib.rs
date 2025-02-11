#![no_std]

use dharitri_sc::imports::*;

pub mod exchange_actions;
pub mod hooks;
pub mod token;
pub mod transfer;
pub mod users;

#[dharitri_sc::contract]
pub trait Erc3643:
    users::UsersModule
    + token::TokenModule
    + hooks::call_hook::CallHookModule
    + hooks::change_hooks::ChangeHooksModule
    + dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + dharitri_sc_modules::pause::PauseModule
{
    #[init]
    fn init(&self) {
        self.set_paused(true);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
