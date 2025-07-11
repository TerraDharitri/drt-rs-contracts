use dharitri_sc_modules::transfer_role_proxy::PaymentsVec;

use super::hook_type::{ErcHookType, Hook};

use dharitri_sc::imports::*;

#[dharitri_sc::module]
pub trait CallHookModule {
    fn call_hook(
        &self,
        hook_type: ErcHookType,
        original_caller: ManagedAddress,
        input_payments: PaymentsVec<Self::Api>,
        args: ManagedVec<ManagedBuffer>,
    ) -> PaymentsVec<Self::Api> {
        let hooks = self.hooks(hook_type).get();
        if hooks.is_empty() {
            return input_payments;
        }

        let payments_len = input_payments.len();
        let mut call_args = ManagedArgBuffer::new();
        call_args.push_arg(original_caller);

        for arg in &args {
            call_args.push_arg(arg);
        }

        let mut output_payments = input_payments;
        for hook in hooks {
            let back_transfers = self
                .tx()
                .to(hook.dest_address)
                .raw_call(hook.endpoint_name)
                .arguments_raw(call_args.clone())
                .with_multi_token_transfer(output_payments.clone())
                .returns(ReturnsBackTransfers)
                .sync_call();

            require!(
                back_transfers.dcdt_payments.len() == payments_len,
                "Wrong payments received from SC"
            );

            for (payment_before, payment_after) in output_payments
                .iter()
                .zip(back_transfers.dcdt_payments.iter())
            {
                require!(
                    payment_before.token_identifier == payment_after.token_identifier
                        && payment_before.token_nonce == payment_after.token_nonce,
                    "Invalid payment received from SC"
                );
            }

            output_payments = back_transfers.dcdt_payments;
        }

        output_payments
    }

    fn encode_arg_to_vec<T: TopEncode>(&self, arg: &T, vec: &mut ManagedVec<ManagedBuffer>) {
        let mut encoded_value = ManagedBuffer::new();
        let _ = arg.top_encode(&mut encoded_value);
        vec.push(encoded_value);
    }

    #[storage_mapper("hooks")]
    fn hooks(&self, hook_type: ErcHookType) -> SingleValueMapper<ManagedVec<Hook<Self::Api>>>;
}
