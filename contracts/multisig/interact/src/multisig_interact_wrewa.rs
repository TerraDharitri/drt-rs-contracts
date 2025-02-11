use std::time::Duration;

use multisig::action::GasLimit;
use dharitri_sc_snippets::imports::*;

use super::*;

const WRAP_AMOUNT: u64 = 50000000000000000; // 0.05 REWA
const UNWRAP_AMOUNT: u64 = 25000000000000000; // 0.025 WREWA

impl MultisigInteract {
    pub async fn wrewa_swap_full(&mut self) {
        self.deploy().await;
        self.feed_contract_rewa().await;
        self.wrap_rewa().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.unwrap_rewa().await;
    }

    pub async fn wrap_rewa(&mut self) {
        println!("proposing wrap rewa...");
        let action_id = self.propose_wrap_rewa().await;

        println!("perfoming wrap rewa action `{action_id}`...");
        self.perform_action(action_id, 15_000_000u64).await;
    }

    pub async fn unwrap_rewa(&mut self) {
        println!("proposing unwrap rewa...");
        let action_id = self.propose_unwrap_rewa().await;

        println!("perfoming unwrap rewa action `{action_id}`...");
        self.perform_action(action_id, 15_000_000u64).await;
    }

    pub async fn wrewa_swap_set_state(&mut self) {
        self.interactor
            .retrieve_account(&self.config.wrewa_address)
            .await;
    }

    async fn propose_wrap_rewa(&mut self) -> usize {
        let function_call = self
            .interactor
            .tx()
            .to(&self.config.wrewa_address)
            .typed(wrewa_proxy::RewaDcdtSwapProxy)
            .wrap_rewa()
            .into_function_call();

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(
                &self.config.wrewa_address,
                WRAP_AMOUNT,
                Option::<GasLimit>::None,
                function_call,
            )
            .returns(ReturnsResult)
            .run()
            .await;

        println!("successfully proposed wrap rewa action `{action_id}`");
        action_id
    }

    pub async fn query_wrewa_token_identifier(&mut self) -> TokenIdentifier<StaticApi> {
        let wrewa_token_id = self
            .interactor
            .query()
            .to(&self.config.wrewa_address)
            .typed(wrewa_proxy::RewaDcdtSwapProxy)
            .wrapped_rewa_token_id()
            .returns(ReturnsResult)
            .run()
            .await;

        println!("WREWA token identifier: {wrewa_token_id}");

        wrewa_token_id
    }

    async fn propose_unwrap_rewa(&mut self) -> usize {
        let wrewa_token_id = self.query_wrewa_token_identifier().await;

        let normalized_tx = self
            .interactor
            .tx()
            .to(&self.config.wrewa_address)
            .typed(wrewa_proxy::RewaDcdtSwapProxy)
            .unwrap_rewa()
            .single_dcdt(&wrewa_token_id, 0u64, &UNWRAP_AMOUNT.into())
            .normalize();
        let normalized_to = normalized_tx.to;
        let normalized_data = normalized_tx.data;

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(
                normalized_to,
                0u64,
                Option::<GasLimit>::None,
                normalized_data,
            )
            .returns(ReturnsResult)
            .run()
            .await;

        println!("successfully proposed unwrap rewa action `{action_id}`");
        action_id
    }
}
