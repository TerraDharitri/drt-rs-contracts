use dharitri_sc::derive_imports::*;
use dharitri_sc::imports::*;

pub const MAX_DISTRIBUTION_PERCENTAGE: u64 = 100_000; // 100%

#[type_abi]
#[derive(ManagedVecItem, NestedEncode, NestedDecode)]
pub struct Distribution<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percentage: u64,
    pub endpoint: ManagedBuffer<M>,
    pub gas_limit: u64,
}

#[dharitri_sc::module]
pub trait DistributionModule {
    fn init_distribution(&self, distribution: ManagedVec<Distribution<Self::Api>>) {
        self.validate_distribution(&distribution);
        self.distribution_rules().set(distribution);
    }

    fn distribute_funds(
        &self,
        token_id: &RewaOrDcdtTokenIdentifier,
        token_nonce: u64,
        total_amount: BigUint,
    ) {
        if total_amount == 0 {
            return;
        }
        for distribution in self.distribution_rules().get() {
            let payment_amount =
                &total_amount * distribution.percentage / MAX_DISTRIBUTION_PERCENTAGE;
            if payment_amount == 0 {
                continue;
            }

            self.tx()
                .to(distribution.address)
                .raw_call(distribution.endpoint)
                .payment(RewaOrDcdtTokenPayment::new(
                    token_id.clone(),
                    token_nonce,
                    payment_amount,
                ))
                .gas(distribution.gas_limit)
                .transfer_execute();
        }
    }

    fn validate_distribution(&self, distribution: &ManagedVec<Distribution<Self::Api>>) {
        let index_total: u64 = distribution
            .iter()
            .map(|distribution| distribution.percentage)
            .sum();
        require!(
            index_total == MAX_DISTRIBUTION_PERCENTAGE,
            "Distribution percent total must be 100%"
        );
    }

    #[view(getDistributionRules)]
    #[storage_mapper("distributionRules")]
    fn distribution_rules(&self) -> SingleValueMapper<ManagedVec<Distribution<Self::Api>>>;
}
