use ethers::types::{Address, U256};
use ethers::{prelude::*, signers::coins_bip39::English};
use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use anyhow::{anyhow, Result, format_err};
use std::sync::Arc;

#[derive(Debug, StructOpt)]
#[structopt(about = "Check on or claim your vest contract")]
pub enum Subcommands {
    Claim(ClaimOpts),
    Check(CheckOpts),
    Mkr(MkrCmd),
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(subcommand)]
    pub subcmd: Subcommands
}
#[derive(Debug, StructOpt)]
#[structopt(about = "Claim your vesting award")]
pub struct ClaimOpts {
    #[structopt(flatten)]
    pub inner: InnerOpts,
    #[structopt(help = "Send awarded tokens to a different address?")]
    pub dest: Option<Address>,
    #[structopt(long = "award-id", short, help = "The id of the award you want to claim")]
    pub id: Option<U256>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Check on award contracts")]
pub struct CheckOpts {
    #[structopt(flatten)]
    pub inner: InnerOpts,
    #[structopt(long = "award-id", short, help = "Check on a specific award ID")]
    pub id: Option<U256>,
    #[structopt(help = "Check for award owned by address `usr`")]
    pub usr: Option<Address>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Interact with MKR token")]
pub enum MkrCmd {
    Transfer(MkrTransferOpts),
    Balance(InnerOpts)
}


#[derive(Debug, StructOpt)]
pub struct MkrTransferOpts {
    #[structopt(flatten)]
    pub inner: InnerOpts,
    #[structopt(help = "Destination address for transfer")]
    pub dest: Address,
    #[structopt(help = "Amount of MKR to transfer")]
    pub amt: U256
}



#[derive(Debug, StructOpt)]
pub struct InnerOpts {
    #[structopt(flatten)]
    pub eth: EthInfoOpts,
    #[structopt(flatten)]
    pub contract: ContractOpts
}


#[derive(Debug, StructOpt)]
pub struct ContractOpts {
    #[structopt(long = "contract-address", short, help = "The address of the DssVest contract")]
    pub addr: Address,
  
}
#[derive(Debug, StructOpt)]
pub struct EthInfoOpts {
    #[structopt(long = "eth-api-url", short, help = "Eth node url")]
    pub url: String,
    #[structopt(long = "eth-priv-key", short, help = "Your private key string")]
    pub private_key: Option<String>,
}

impl EthInfoOpts {
    pub fn get_client(&self) ->  Result<Arc<Provider<Http>>> {
        Ok(Arc::new(Provider::try_from(self.url.as_str())?))
    }

    pub fn get_wallet(&self) -> Result<LocalWallet> {
         if let Some(ref priv_key) = self.private_key {
            Ok(LocalWallet::from_str(priv_key)?)
         } else {
             Err(anyhow!("Please provide private key at this time, as paths are not yet supported"))
         }
    }
}