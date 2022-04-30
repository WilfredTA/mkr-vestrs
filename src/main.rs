use dss_vest_rs::{DssVestTransferrable, DSSVESTTRANSFERRABLE_ABI};
use ethers::prelude::*;
mod opts;
use opts::{Opts, Subcommands};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();

    match opts.subcmd {
        Subcommands::Claim(claim_opts) => {

        },
        Subcommands::Check(check_opts) => {

        },
        Subcommands::Mkr(mkr_cmd) => {
            
        },
    }
}