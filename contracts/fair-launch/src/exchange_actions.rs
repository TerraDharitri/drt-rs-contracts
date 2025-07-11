use crate::common::{Percentage, MAX_FEE_PERCENTAGE};

use dharitri_sc::derive_imports::*;
use dharitri_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone)]
pub struct EndpointInfo<M: ManagedTypeApi> {
    pub endpoint_name: ManagedBuffer<M>,
    pub input_fee_percentage: Percentage,
    pub burn_input: bool,
    pub output_fee_percentage: Percentage,
    pub burn_output: bool,
}

#[dharitri_sc::module]
pub trait ExchangeActionsModule:
    crate::common::CommonModule
    + crate::initial_launch::InitialLaunchModule
    + crate::token_info::TokenInfoModule
    + dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + dharitri_sc_modules::pause::PauseModule
{
    /// Arguments: endpoint_name,
    /// input_fee_percentage: between 0 and 10_000,
    /// burn_input: bool, burn input tokens taken as fee,
    /// output_fee_percentage: between 0 and 10_000,
    /// burn_output: bool, burn output taken as fee
    #[only_owner]
    #[endpoint(addExchangeEndpoint)]
    fn add_exchange_endpoint(
        &self,
        sc_addr: ManagedAddress,
        endpoint_name_fee_percentage_pairs: MultiValueEncoded<
            MultiValue5<ManagedBuffer, Percentage, bool, Percentage, bool>,
        >,
    ) {
        let mut new_endpoints = ManagedVec::<Self::Api, EndpointInfo<Self::Api>>::new();
        for pair in endpoint_name_fee_percentage_pairs {
            let (
                endpoint_name,
                input_fee_percentage,
                burn_input,
                output_fee_percentage,
                burn_output,
            ) = pair.into_tuple();

            require!(
                input_fee_percentage <= MAX_FEE_PERCENTAGE
                    && output_fee_percentage <= MAX_FEE_PERCENTAGE,
                "Invalid fee percentage"
            );

            new_endpoints.push(EndpointInfo {
                endpoint_name,
                input_fee_percentage,
                burn_input,
                output_fee_percentage,
                burn_output,
            });
        }

        let mapper = self.known_contracts(&sc_addr);
        let mut current_endpoints = mapper.get();
        for new_endpoint in new_endpoints {
            for current_endpoint in &current_endpoints {
                require!(
                    current_endpoint.endpoint_name != new_endpoint.endpoint_name,
                    "Endpoint already known"
                );
            }

            current_endpoints.push(new_endpoint);
        }

        mapper.set(current_endpoints);
    }

    #[only_owner]
    #[endpoint(removeExchangeEndpoint)]
    fn remove_exchange_endpoint(
        &self,
        sc_addr: ManagedAddress,
        endpoint_names: MultiValueEncoded<ManagedBuffer>,
    ) {
        let mapper = self.known_contracts(&sc_addr);
        let mut current_endpoints = mapper.get();

        for endpoint_to_remove in endpoint_names {
            let endpoint_to_remove_index = current_endpoints
                .iter()
                .position(|endpoint| endpoint.endpoint_name == endpoint_to_remove);

            require!(endpoint_to_remove_index.is_some(), "Unknown endpoint name");

            current_endpoints.remove(endpoint_to_remove_index.unwrap());
        }
    }

    /// forward an execute on dest context call on an exchange SC
    #[payable]
    #[endpoint(forwardExecuteOnDest)]
    fn forward_execute_on_dest(
        &self,
        dest: ManagedAddress,
        endpoint_name: ManagedBuffer,
        extra_args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.require_not_paused();
        self.require_not_initial_launch();

        // TODO: switch to rewa+dcdt multi transfer handling
        let rewa_value = self.call_value().rewa_direct_non_strict().clone();
        require!(rewa_value == 0, "Invalid payment");

        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_dcdt_transfers().clone();
        let endpoint_info = self.find_endpoint_info(&dest, &endpoint_name);

        let mut input_fees_percentage = ManagedVec::new();
        for _ in 0..payments.len() {
            input_fees_percentage.push(endpoint_info.input_fee_percentage);
        }

        let back_transfers = if !payments.is_empty() {
            let take_fees_result =
                self.take_fees(caller.clone(), payments.clone(), input_fees_percentage);

            if endpoint_info.burn_input {
                self.burn_all_tokens(&take_fees_result.fees);
            }

            self.tx()
                .to(dest)
                .raw_call(endpoint_name)
                .arguments_raw(ManagedArgBuffer::from(extra_args.into_vec_of_buffers()))
                .with_multi_token_transfer(take_fees_result.transfers.clone())
                .returns(ReturnsBackTransfers)
                .sync_call()
        } else {
            self.tx()
                .to(dest)
                .raw_call(endpoint_name)
                .arguments_raw(ManagedArgBuffer::from(extra_args.into_vec_of_buffers()))
                .returns(ReturnsBackTransfers)
                .sync_call()
        };

        if !back_transfers.dcdt_payments.is_empty() {
            let mut output_fees_percentage = ManagedVec::new();
            for _ in 0..back_transfers.dcdt_payments.len() {
                output_fees_percentage.push(endpoint_info.output_fee_percentage);
            }

            let take_fees_from_results =
                self.take_fees(caller, back_transfers.dcdt_payments, output_fees_percentage);

            if endpoint_info.burn_output {
                self.burn_all_tokens(&take_fees_from_results.fees);
            }

            self.tx()
                .to(&take_fees_from_results.original_caller)
                .payment(&take_fees_from_results.transfers)
                .transfer();
        }
    }

    fn find_endpoint_info(
        &self,
        dest: &ManagedAddress,
        endpoint_name: &ManagedBuffer,
    ) -> EndpointInfo<Self::Api> {
        let known_sc_mapper = self.known_contracts(dest);
        require!(
            !known_sc_mapper.is_empty(),
            "Unknown SC, use forwardTransfer endpoint"
        );

        let endpoints_info = known_sc_mapper.get();
        let mut opt_info = None;
        for info in &endpoints_info {
            if &info.endpoint_name == endpoint_name {
                opt_info = Some(info);
                break;
            }
        }

        match opt_info {
            Some(info) => info.clone(),
            None => sc_panic!("Unknown endpoint"),
        }
    }
}
