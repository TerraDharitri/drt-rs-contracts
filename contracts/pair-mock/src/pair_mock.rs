#![no_std]

use dharitri_sc::imports::*;

#[dharitri_sc::contract]
pub trait PairMock {
    #[init]
    fn init(&self, first_token_id: TokenIdentifier, second_token_id: TokenIdentifier) {
        self.first_token_id().set(first_token_id);
        self.second_token_id().set(second_token_id);
    }

    #[payable]
    #[endpoint(swapTokensFixedInput)]
    fn swap_tokens_fixed_input(
        &self,
        _token_out: TokenIdentifier,
        _amount_out_min: BigUint,
    ) -> DcdtTokenPayment {
        let payment = self.call_value().single_dcdt();
        let first_token_id = self.first_token_id().get();
        let second_token_id = self.second_token_id().get();
        let output = if payment.token_identifier == first_token_id {
            DcdtTokenPayment::new(second_token_id, 0, &payment.amount * 2u32)
        } else {
            DcdtTokenPayment::new(first_token_id, 0, &payment.amount / 2u32)
        };

        self.tx()
            .to(ToCaller)
            .payment(DcdtTokenPayment::new(
                output.token_identifier.clone(),
                0,
                output.amount.clone(),
            ))
            .transfer();

        output
    }

    #[storage_mapper("firstTokenId")]
    fn first_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("secondTokenId")]
    fn second_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
