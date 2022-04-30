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
mkr-vestrs 0.1.0
Utilities for working with dss-vest contracts and the MKR token

USAGE:
    mkr-vest <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check    Check on award contracts
    claim    Claim your vesting award
    help     Prints this message or the help of the given subcommand(s)
    mkr      Interact with MKR token

```