use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("drtsc:output/empty.drtsc.json", empty::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/empty.scen.json");
}
