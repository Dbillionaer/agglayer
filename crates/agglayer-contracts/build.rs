use std::path::Path;

fn main() {
    let contract_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/contracts");

    build_contract(
        "PolygonZkEVM.json",
        "polygon_zk_evm.rs",
        "PolygonZkEvm",
        &contract_dir,
    );
    build_contract(
        "PolygonRollupManager.json",
        "polygon_rollup_manager.rs",
        "PolygonRollupManager",
        &contract_dir,
    );
    build_contract(
        "PolygonZkEVMGlobalExitRootV2.json",
        "polygon_zkevm_global_exit_root_v2.rs",
        "PolygonZkEVMGlobalExitRootV2",
        &contract_dir,
    );
    build_contract(
        "AggchainBase.json",
        "aggchain_base.rs",
        "AggchainBase",
        &contract_dir,
    );
    build_contract(
        "AggLayerGateway.json",
        "agglayer_gateway.rs",
        "AggLayerGateway",
        &contract_dir,
    );
}

fn build_contract(source: &str, destination: &str, struct_name: &str, contract_dir: &Path) {
    let abi_source = contract_dir.join(source);
    let out_file = contract_dir.join(destination);

    if out_file.exists() {
        std::fs::remove_file(&out_file).unwrap();
    }

    ethers_contract::Abigen::new(struct_name, abi_source.display().to_string())
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(out_file)
        .unwrap();
}
