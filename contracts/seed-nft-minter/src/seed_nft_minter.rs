#![no_std]

use dharitri_sc::derive_imports::*;
use dharitri_sc::imports::*;

mod distribution_module;
pub mod nft_marketplace_proxy;
mod nft_module;

use distribution_module::Distribution;
use dharitri_sc_modules::default_issue_callbacks;

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[dharitri_sc::contract]
pub trait SeedNftMinter:
    distribution_module::DistributionModule
    + nft_module::NftModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(
        &self,
        marketplaces: ManagedVec<ManagedAddress>,
        distribution: ManagedVec<Distribution<Self::Api>>,
    ) {
        self.marketplaces().extend(marketplaces);
        self.init_distribution(distribution);
    }

    #[allow_multiple_var_args]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        uri: ManagedBuffer,
        selling_price: BigUint,
        opt_token_used_as_payment: OptionalValue<TokenIdentifier>,
        opt_token_used_as_payment_nonce: OptionalValue<u64>,
    ) {
        let token_used_as_payment = match opt_token_used_as_payment {
            OptionalValue::Some(token) => RewaOrDcdtTokenIdentifier::dcdt(token),
            OptionalValue::None => RewaOrDcdtTokenIdentifier::rewa(),
        };
        require!(
            token_used_as_payment.is_valid(),
            "Invalid token_used_as_payment arg, not a valid token ID"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_rewa() {
            0
        } else {
            match opt_token_used_as_payment_nonce {
                OptionalValue::Some(nonce) => nonce,
                OptionalValue::None => 0,
            }
        };

        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let nft_nonce = self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );

        self.nft_count().set(nft_nonce);
    }

    #[only_owner]
    #[endpoint(claimAndDistribute)]
    fn claim_and_distribute(&self, token_id: RewaOrDcdtTokenIdentifier, token_nonce: u64) {
        let total_amount = self.claim_royalties(&token_id, token_nonce);
        self.distribute_funds(&token_id, token_nonce, total_amount);
    }

    fn claim_royalties(&self, token_id: &RewaOrDcdtTokenIdentifier, token_nonce: u64) -> BigUint {
        let claim_destination = self.blockchain().get_sc_address();
        let mut total_amount = BigUint::zero();
        for address in self.marketplaces().iter() {
            let results: MultiValue2<BigUint, ManagedVec<DcdtTokenPayment>> = self
                .tx()
                .to(&address)
                .typed(nft_marketplace_proxy::NftMarketplaceProxy)
                .claim_tokens(&claim_destination, token_id, token_nonce)
                .returns(ReturnsResult)
                .sync_call();

            let (rewa_amount, dcdt_payments) = results.into_tuple();
            let amount = if token_id.is_rewa() {
                rewa_amount
            } else {
                dcdt_payments
                    .try_get(0)
                    .map(|dcdt_payment| dcdt_payment.amount.clone())
                    .unwrap_or_default()
            };
            total_amount += amount;
        }

        total_amount
    }

    #[view(getMarketplaces)]
    #[storage_mapper("marketplaces")]
    fn marketplaces(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNftCount)]
    #[storage_mapper("nftCount")]
    fn nft_count(&self) -> SingleValueMapper<u64>;
}
