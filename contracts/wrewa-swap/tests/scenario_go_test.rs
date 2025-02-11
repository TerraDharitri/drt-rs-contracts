use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn unwrap_rewa_go() {
    world().run("scenarios/unwrap_rewa.scen.json");
}

#[test]
fn wrap_rewa_go() {
    world().run("scenarios/wrap_rewa.scen.json");
}
