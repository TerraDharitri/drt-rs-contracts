use adder::adder_proxy;
use imports::{
    RewaOrDcdtTokenIdentifier, DcdtTokenPayment, MultiDcdtPayment, DrtscPath, TestAddress,
    TestSCAddress, TestTokenIdentifier, TokenIdentifier,
};
use dharitri_sc::{
    codec::{multi_types::MultiValueVec, top_encode_to_vec_u8_or_panic},
    types::{BigUint, MultiValueEncoded},
};
use dharitri_sc_scenario::{api::StaticApi, *};
use dharitri_sc_wrewa_swap::wrewa_proxy;
use paymaster::paymaster_proxy;

const PAYMASTER_ADDRESS_EXPR: TestSCAddress = TestSCAddress::new("paymaster");
const RELAYER_ADDRESS_EXPR: TestAddress = TestAddress::new("relayer");
const CALLEE_SC_ADDER_ADDRESS_EXPR: TestSCAddress = TestSCAddress::new("adder");
const CALLEE_SC_WREWA_ADDRESS_EXPR: TestSCAddress = TestSCAddress::new("sc:wrewa");
const PAYMASTER_PATH_EXPR: DrtscPath = DrtscPath::new("../output/paymaster.drtsc.json");
const ADDER_PATH_EXPR: DrtscPath = DrtscPath::new("../adder/output/adder.drtsc.json");
const WREWA_PATH_EXPR: DrtscPath =
    DrtscPath::new("../wrewa-swap/output/dharitri-sc-wrewa-swap.drtsc.json");
const CALLER_ADDRESS_EXPR: TestAddress = TestAddress::new("caller");
const CALLEE_USER_ADDRESS_EXPR: TestAddress = TestAddress::new("callee_user");
const OWNER_ADDRESS_EXPR: TestAddress = TestAddress::new("owner");
const BALANCE: u64 = 100_000_000;
const PAYMASTER_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("PAYMSTR-123456");
const WREWA_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("WREWA-123456");
const WREWA_TOKEN_ID: &[u8] = b"WREWA-123456";
const FEE_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("FEE-123456");
const ADDITIONAL_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("ADDIT-123456");
const FEE_AMOUNT: u64 = 20_000;
const INITIAL_ADD_VALUE: u64 = 5;
const ADDITIONAL_ADD_VALUE: u64 = 5;
const UNWRAP_ENDPOINT_NAME: &[u8] = b"unwrap";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(PAYMASTER_PATH_EXPR, paymaster::ContractBuilder);
    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain.register_contract(WREWA_PATH_EXPR, dharitri_sc_wrewa_swap::ContractBuilder);

    blockchain
}

struct PaymasterTestState {
    world: ScenarioWorld,
}

impl PaymasterTestState {
    fn new() -> Self {
        let mut world = world();
        world.start_trace();
        world.account(OWNER_ADDRESS_EXPR).nonce(1);
        world
            .account(CALLER_ADDRESS_EXPR)
            .nonce(1)
            .balance(BALANCE)
            .dcdt_balance(PAYMASTER_TOKEN_ID_EXPR, BALANCE)
            .dcdt_balance(WREWA_TOKEN_ID_EXPR, BALANCE)
            .dcdt_balance(FEE_TOKEN_ID_EXPR, BALANCE)
            .dcdt_balance(ADDITIONAL_TOKEN_ID_EXPR, BALANCE);

        world
            .account(CALLEE_USER_ADDRESS_EXPR)
            .nonce(1)
            .balance(BALANCE);
        world.account(RELAYER_ADDRESS_EXPR).nonce(1).balance(0);

        Self { world }
    }

    fn deploy_paymaster_contract(&mut self) -> &mut Self {
        self.world
            .new_address(OWNER_ADDRESS_EXPR, 1, PAYMASTER_ADDRESS_EXPR);

        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .typed(paymaster_proxy::PaymasterContractProxy)
            .init()
            .code(PAYMASTER_PATH_EXPR)
            .new_address(PAYMASTER_ADDRESS_EXPR)
            .run();

        self
    }

    fn deploy_adder_contract(&mut self) -> &mut Self {
        self.world
            .new_address(OWNER_ADDRESS_EXPR, 2, CALLEE_SC_ADDER_ADDRESS_EXPR);

        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .typed(adder_proxy::AdderProxy)
            .init(INITIAL_ADD_VALUE)
            .code(ADDER_PATH_EXPR)
            .new_address(CALLEE_SC_ADDER_ADDRESS_EXPR)
            .run();

        self
    }

    fn deploy_wrewa_contract(&mut self) -> &mut Self {
        self.world
            .new_address(OWNER_ADDRESS_EXPR, 3, CALLEE_SC_WREWA_ADDRESS_EXPR);

        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .typed(wrewa_proxy::RewaDcdtSwapProxy)
            .init(WREWA_TOKEN_ID)
            .code(WREWA_PATH_EXPR)
            .new_address(CALLEE_SC_WREWA_ADDRESS_EXPR)
            .run();

        self
    }

    fn check_dcdt_balance(
        &mut self,
        address: TestAddress,
        token: TestTokenIdentifier,
        balance: u64,
    ) -> &mut Self {
        self.world
            .check_account(address)
            .dcdt_balance(token, balance);

        self
    }

    fn check_rewa_balance(&mut self, address: TestAddress, balance: u64) -> &mut Self {
        self.world.check_account(address).balance(balance);

        self
    }
}

#[test]
fn test_deploy_paymasters() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();
    state.deploy_wrewa_contract();
}

#[test]
fn test_forward_call_no_fee_payment() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();

    state
        .world
        .tx()
        .from(CALLER_ADDRESS_EXPR)
        .to(PAYMASTER_ADDRESS_EXPR)
        .typed(paymaster_proxy::PaymasterContractProxy)
        .forward_execution(
            RELAYER_ADDRESS_EXPR,
            CALLEE_USER_ADDRESS_EXPR,
            b"add",
            MultiValueVec::<Vec<u8>>::new(),
        )
        .with_result(ExpectError(4, "There is no fee for payment!"))
        .run();
}

#[test]
fn test_forward_call_user() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();

    state
        .world
        .tx()
        .from(CALLER_ADDRESS_EXPR)
        .to(PAYMASTER_ADDRESS_EXPR)
        .typed(paymaster_proxy::PaymasterContractProxy)
        .forward_execution(
            RELAYER_ADDRESS_EXPR,
            CALLEE_USER_ADDRESS_EXPR,
            b"add",
            MultiValueVec::<Vec<u8>>::new(),
        )
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(FEE_TOKEN_ID_EXPR),
            0u64,
            &BigUint::from(FEE_AMOUNT),
        )
        .run();

    state
        .world
        .check_account(RELAYER_ADDRESS_EXPR)
        .dcdt_balance(FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
}

#[test]
fn test_forward_call_sc_adder() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, PAYMASTER_TOKEN_ID_EXPR, BALANCE);

    state
        .world
        .tx()
        .from(CALLER_ADDRESS_EXPR)
        .to(PAYMASTER_ADDRESS_EXPR)
        .typed(paymaster_proxy::PaymasterContractProxy)
        .forward_execution(
            RELAYER_ADDRESS_EXPR,
            CALLEE_SC_ADDER_ADDRESS_EXPR,
            b"add",
            MultiValueVec::from([top_encode_to_vec_u8_or_panic(&ADDITIONAL_ADD_VALUE)]),
        )
        .single_dcdt(
            &TokenIdentifier::from(FEE_TOKEN_ID_EXPR),
            0,
            &BigUint::from(FEE_AMOUNT),
        )
        .run();

    let expected_adder_sum = INITIAL_ADD_VALUE + ADDITIONAL_ADD_VALUE;

    state
        .world
        .query()
        .to(CALLEE_SC_ADDER_ADDRESS_EXPR)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .with_result(ExpectValue(expected_adder_sum))
        .run();
    state.check_dcdt_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
}

#[test]
fn test_forward_call_sc_adder_with_relayer_address() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, PAYMASTER_TOKEN_ID_EXPR, BALANCE);

    state
        .world
        .tx()
        .from(CALLER_ADDRESS_EXPR)
        .to(PAYMASTER_ADDRESS_EXPR)
        .typed(paymaster_proxy::PaymasterContractProxy)
        .forward_execution(
            RELAYER_ADDRESS_EXPR,
            CALLEE_SC_ADDER_ADDRESS_EXPR,
            b"add",
            MultiValueVec::from([top_encode_to_vec_u8_or_panic(&ADDITIONAL_ADD_VALUE)]),
        )
        .single_dcdt(
            &TokenIdentifier::from(FEE_TOKEN_ID_EXPR),
            0,
            &BigUint::from(FEE_AMOUNT),
        )
        .run();

    let expected_adder_sum = INITIAL_ADD_VALUE + ADDITIONAL_ADD_VALUE;
    state
        .world
        .query()
        .to(CALLEE_SC_ADDER_ADDRESS_EXPR)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .with_result(ExpectValue(expected_adder_sum))
        .run();

    state.check_dcdt_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
}

#[test]
fn test_forward_call_wrewa() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, WREWA_TOKEN_ID_EXPR, BALANCE);

    let mut payments: MultiDcdtPayment<StaticApi> = MultiDcdtPayment::new();
    payments.push(DcdtTokenPayment::new(
        TokenIdentifier::from(FEE_TOKEN_ID_EXPR),
        0,
        BigUint::from(FEE_AMOUNT),
    ));
    payments.push(DcdtTokenPayment::new(
        TokenIdentifier::from(WREWA_TOKEN_ID_EXPR),
        0,
        BigUint::from(FEE_AMOUNT),
    ));

    // Call fails because unwrap amount is 0
    state
        .world
        .tx()
        .from(CALLER_ADDRESS_EXPR)
        .to(PAYMASTER_ADDRESS_EXPR)
        .typed(paymaster_proxy::PaymasterContractProxy)
        .forward_execution(
            RELAYER_ADDRESS_EXPR,
            CALLEE_SC_WREWA_ADDRESS_EXPR,
            UNWRAP_ENDPOINT_NAME,
            MultiValueEncoded::new(),
        )
        .multi_dcdt(payments)
        .run();

    // Fee is kept by the relayer
    let new_fee_amount: u64 = 99_980_000;
    state.check_dcdt_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, new_fee_amount);

    // Caller has the original balance
    state.check_rewa_balance(CALLER_ADDRESS_EXPR, BALANCE);
}

#[test]
fn test_forward_call_fails_wrewa_0_amount() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, WREWA_TOKEN_ID_EXPR, BALANCE);

    let failling_amount = 0u64;

    let mut payments: MultiDcdtPayment<StaticApi> = MultiDcdtPayment::new();
    payments.push(DcdtTokenPayment::new(
        TokenIdentifier::from(FEE_TOKEN_ID_EXPR),
        0,
        BigUint::from(FEE_AMOUNT),
    ));
    payments.push(DcdtTokenPayment::new(
        TokenIdentifier::from(WREWA_TOKEN_ID_EXPR),
        0,
        BigUint::from(failling_amount),
    ));

    // Call fails because unwrap amount is 0
    state
        .world
        .tx()
        .from(CALLER_ADDRESS_EXPR)
        .to(PAYMASTER_ADDRESS_EXPR)
        .typed(paymaster_proxy::PaymasterContractProxy)
        .forward_execution(
            RELAYER_ADDRESS_EXPR,
            CALLEE_SC_WREWA_ADDRESS_EXPR,
            UNWRAP_ENDPOINT_NAME,
            MultiValueEncoded::new(),
        )
        .multi_dcdt(payments)
        .run();

    // Fee is kept by the relayer
    let new_fee_amount: u64 = 99_980_000;
    state.check_dcdt_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, new_fee_amount);

    // Caller has the original balance
    state.check_dcdt_balance(CALLER_ADDRESS_EXPR, WREWA_TOKEN_ID_EXPR, BALANCE);
}
