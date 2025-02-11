#![no_std]

use dharitri_sc::imports::*;

pub mod available_tokens;
pub mod dn404_proxy;
pub mod fee;
pub mod price;

pub type Nonce = u64;
pub type Percentage = u32;

pub const MAX_PERCENTAGE: Percentage = 10_000;
pub const NFT_AMOUNT: u32 = 1;

#[dharitri_sc::contract]
pub trait Dn404:
    available_tokens::AvailableTokensModule
    + price::PriceModule
    + fee::FeeModule
    + dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + dharitri_sc_modules::pause::PauseModule
    + dharitri_sc_modules::only_admin::OnlyAdminModule
{
    /// Needs mint and burn roles for fractal_token
    #[init]
    fn init(&self, fractal_token_id: TokenIdentifier, admins: MultiValueEncoded<ManagedAddress>) {
        require!(
            fractal_token_id.is_valid_dcdt_identifier(),
            "Invalid token ID"
        );

        self.fractal_token().set_token_id(fractal_token_id);

        let caller = self.blockchain().get_caller();
        let _ = self.admins().insert(caller);
        self.admins().extend(admins);
        self.set_paused(true);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
