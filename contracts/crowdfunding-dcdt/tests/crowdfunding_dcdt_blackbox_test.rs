use crowdfunding_dcdt::crowdfunding_dcdt_proxy;

use dharitri_sc_scenario::imports::*;

const CF_DEADLINE: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
const CF_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("CROWD-123456");
const FIRST_USER_ADDRESS: TestAddress = TestAddress::new("first-user");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SECOND_USER_ADDRESS: TestAddress = TestAddress::new("second-user");
const CODE_PATH: DrtscPath = DrtscPath::new("output/crowdfunding-dcdt.drtsc.json");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding-dcdt");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/crowdfunding-dcdt");
    blockchain.register_contract(CODE_PATH, crowdfunding_dcdt::ContractBuilder);
    blockchain
}

struct CrowdfundingDCDTTestState {
    world: ScenarioWorld,
}

impl CrowdfundingDCDTTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);

        world
            .account(FIRST_USER_ADDRESS)
            .nonce(1)
            .balance(1000)
            .dcdt_balance(CF_TOKEN_ID, 1000);

        world
            .account(SECOND_USER_ADDRESS)
            .nonce(1)
            .dcdt_balance(CF_TOKEN_ID, 1000);

        Self { world }
    }

    fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
            .init(
                2_000u32,
                CF_DEADLINE,
                RewaOrDcdtTokenIdentifier::dcdt(CF_TOKEN_ID),
            )
            .code(CODE_PATH)
            .new_address(CROWDFUNDING_ADDRESS)
            .run();
    }

    fn fund(&mut self, address: TestAddress, amount: u64) {
        self.world
            .tx()
            .from(address)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
            .fund()
            .rewa_or_single_dcdt(
                &RewaOrDcdtTokenIdentifier::dcdt(CF_TOKEN_ID),
                0u64,
                &dharitri_sc::proxy_imports::BigUint::from(amount),
            )
            .run();
    }

    fn check_deposit(&mut self, donor: TestAddress, amount: u64) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
            .deposit(donor)
            .returns(ExpectValue(amount))
            .run();
    }

    fn check_status(&mut self, expected_value: crowdfunding_dcdt_proxy::Status) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
            .status()
            .returns(ExpectValue(expected_value))
            .run();
    }

    fn claim(&mut self, address: TestAddress) {
        self.world
            .tx()
            .from(address)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
            .claim()
            .run();
    }

    fn check_dcdt_balance(&mut self, address: TestAddress, balance: u64) {
        self.world
            .check_account(address)
            .dcdt_balance(CF_TOKEN_ID, balance);
    }

    fn set_block_timestamp(&mut self, block_timestamp_expr: u64) {
        self.world
            .current_block()
            .block_timestamp(block_timestamp_expr);
    }
}

#[test]
fn test_fund() {
    let mut state = CrowdfundingDCDTTestState::new();
    state.deploy();

    state.fund(FIRST_USER_ADDRESS, 1_000u64);
    state.check_deposit(FIRST_USER_ADDRESS, 1_000u64);
}

#[test]
fn test_status() {
    let mut state = CrowdfundingDCDTTestState::new();
    state.deploy();

    state.check_status(crowdfunding_dcdt_proxy::Status::FundingPeriod);
}

#[test]
fn test_sc_error() {
    let mut state = CrowdfundingDCDTTestState::new();
    state.deploy();

    state
        .world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
        .fund()
        .rewa(1000)
        .with_result(ExpectError(4, "wrong token"))
        .run();

    state.check_deposit(FIRST_USER_ADDRESS, 0);
}

#[test]
fn test_successful_cf() {
    let mut state = CrowdfundingDCDTTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS, 1_000u64);
    state.check_deposit(FIRST_USER_ADDRESS, 1_000);

    // second user fund
    state.fund(SECOND_USER_ADDRESS, 1000);
    state.check_deposit(SECOND_USER_ADDRESS, 1_000);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + 1);

    // check status successful
    state.check_status(crowdfunding_dcdt_proxy::Status::Successful);

    state
        .world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_dcdt_proxy::CrowdfundingProxy)
        .claim()
        .with_result(ExpectError(4, "only owner can claim successful funding"))
        .run();

    // owner claim
    state.claim(OWNER_ADDRESS);

    state.check_dcdt_balance(OWNER_ADDRESS, 2000);
    state.check_dcdt_balance(FIRST_USER_ADDRESS, 0);
    state.check_dcdt_balance(SECOND_USER_ADDRESS, 0);
}

#[test]
fn test_failed_cf() {
    let mut state = CrowdfundingDCDTTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS, 300);
    state.check_deposit(FIRST_USER_ADDRESS, 300u64);

    // second user fund
    state.fund(SECOND_USER_ADDRESS, 600);
    state.check_deposit(SECOND_USER_ADDRESS, 600u64);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + 1);

    // check status failed
    state.check_status(crowdfunding_dcdt_proxy::Status::Failed);

    // first user claim
    state.claim(FIRST_USER_ADDRESS);

    // second user claim
    state.claim(SECOND_USER_ADDRESS);

    state.check_dcdt_balance(OWNER_ADDRESS, 0);
    state.check_dcdt_balance(FIRST_USER_ADDRESS, 1000);
    state.check_dcdt_balance(SECOND_USER_ADDRESS, 1000);
}
