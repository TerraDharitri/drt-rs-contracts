use dharitri_sc::imports::*;

#[dharitri_sc::module]
pub trait TokenModule:
    dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[payable("REWA")]
    #[endpoint(registerToken)]
    fn register_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().rewa().clone();
        self.token().issue_and_set_all_roles(
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[endpoint(setTransferRole)]
    fn set_transfer_role(&self, opt_address: OptionalValue<ManagedAddress>) {
        let address = match opt_address {
            OptionalValue::Some(addr) => addr,
            OptionalValue::None => self.blockchain().get_sc_address(),
        };

        self.token()
            .set_local_roles_for_address(&address, &[DcdtLocalRole::Transfer], None);
    }

    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token(&self) -> FungibleTokenMapper;
}
