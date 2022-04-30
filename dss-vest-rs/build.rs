use std::{env, error::Error, fs::File, io::Write, path::PathBuf};
use ethers::solc::Solc;
use ethers::contract::Abigen;
fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let contract = "deps/dss-vest/src/DssVest.sol".to_string();
    let contracts = Solc::default().compile_source(&contract)?;
    println!("Contracts: {:?}", contracts);
    let abi = contracts.get(&contract, "DssVestTransferrable").unwrap().abi.unwrap();
    let abi = serde_json::to_string(abi).unwrap();

    let bindings = Abigen::new("DssVestTransferrable", &abi)?.generate()?;
    File::create(out_dir.join("dss-vest.rs"))?
        .write_all(bindings.into_tokens().to_string().as_bytes())?;
        
    println!("Bindings written to {:?}", out_dir.join("dss-vest.rs").as_os_str());

    Ok(())
}