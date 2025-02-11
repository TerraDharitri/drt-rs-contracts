use dharitri_sc::{api::ManagedTypeApi, types::DcdtTokenPayment};

use dharitri_sc::derive_imports::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem)]
pub struct UnlockedToken<M: ManagedTypeApi> {
    pub token: DcdtTokenPayment<M>,
    pub unbond_epoch: u64,
}
