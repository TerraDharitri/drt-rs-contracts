use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/wrewa-swap");
    blockchain.register_contract(
        "drtsc:output/dharitri-sc-wrewa-swap.drtsc.json",
        dharitri_sc_wrewa_swap::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_rewa_rs() {
    world().run("scenarios/unwrap_rewa.scen.json");
}

#[test]
fn wrap_rewa_rs() {
    world().run("scenarios/wrap_rewa.scen.json");
}
