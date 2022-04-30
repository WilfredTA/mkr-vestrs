use dss_vest_rs::{DssVestTransferrable, DSSVESTTRANSFERRABLE_ABI};
use ethers::prelude::*;
use ethers::abi::AbiDecode;
mod opts;
use opts::{Opts, Subcommands, ClaimOpts, CheckOpts, InnerOpts};
use structopt::StructOpt;
use anyhow::Result;
#[tokio::main]
async fn main() {
    let opts = Opts::from_args();

    match opts.subcmd {
        Subcommands::Claim(claim_opts) => {

        },
        Subcommands::Check(check_opts) => {
            check(check_opts).await;
        },
        Subcommands::Mkr(mkr_cmd) => {
            
        },
    }
}

async fn claim_token(opts: ClaimOpts) {
    
}

async fn check(opts: CheckOpts) -> Result<()>{
    let CheckOpts { id, inner, usr } = opts;
    
    let InnerOpts {contract, eth } = inner;

    let provider = eth.get_client()?;
    let contract = DssVestTransferrable::new(contract.addr, provider.clone());
    if let Some(id) = id {
        let award = contract.awards(id).call().await?;
        println!("Award for id {:#?} is {:#?}", id, award);
    }

   
    Ok(())
}