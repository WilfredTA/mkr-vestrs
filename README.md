# Mkr Vest CLI
Simply cli to check on the status of dss-vest awards and to claim your awards.

Motivation for building this is simple: I want to interact with MKR token easily, but then [this issue arose in MetaMask](https://github.com/MetaMask/metamask-extension/issues/14187). 

# Installation
```bash
git clone --recurse-submodules https://github.com/WilfredTA/mkr-vestrs
cargo install --path ./mkr-vestrs

```

# Usage

```bash
mkr-vest help

mkr-vestrs 0.1.0
Check on or claim your vest contract

USAGE:
    mkr-vest <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check    Claim your vesting award
    claim    Claim your vesting award
    help     Prints this message or the help of the given subcommand(s)
```