use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/my-neversea-2025.mxsc.json", my_neversea_2025::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/my_neversea_2025.scen.json");
}
