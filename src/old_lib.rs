#[macro_use]
extern crate log;

pub mod burn;
pub mod constants;
pub mod data;
pub mod decode;
pub mod derive;
pub mod errors;
pub mod limiter;
pub mod mint;
pub mod opt;
pub mod parse;
//pub mod process_subcommands;
pub mod sign;
pub mod snapshot;
pub mod spinner;
pub mod update_metadata;
pub mod withdraw;

use anyhow::Result;
use env_logger::{Builder, Target};
use log::LevelFilter;
use constants::PUBLIC_RPC_URLS;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;

use constants::*;
use opt::*;
use parse::parse_solana_config;
//use process_subcommands::*;


extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

use crate::decode::decode_metadata_lib;
//use crate::snapshot::{snapshot_cm_accounts, snapshot_holders, snapshot_mints};


fn setup_logging(log_level: String) -> Result<()> {
    let level = LevelFilter::from_str(log_level.as_str())?;
    Builder::new()
        .filter_level(level)
        .target(Target::Stdout)
        .init();
    Ok(())
}


py_module_initializer!(mylib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    Ok(())
});


fn get_result(_py: Python, val: &str) -> PyResult<String> {

//    let options = Opt::from_args();

//    setup_logging(options.log_level);
    let mut log_level = String::new();
    log_level.push_str("Info");
    setup_logging(log_level);

    let sol_config = parse_solana_config();

    let mut arg_rpc = String::new();
    arg_rpc.push_str("https://api.devnet.solana.com");

    let timeout : u64 = 60;

//    let (rpc, commitment) = if let Some(cli_rpc) = options.rpc {
//        (cli_rpc.clone(), String::from("confirmed"))
//    } else {
//        if let Some(config) = sol_config {
//            (config.json_rpc_url, config.commitment)
//        } else {
//            info!(
//            "Could not find a valid Solana-CLI config file. Defaulting to https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899/ devenet node."
//        );
//            (
//                String::from("https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899/"),
//                String::from("confirmed"),
//            )
//        }
//    };

    let rpc = arg_rpc;
    let commitment = String::from("confirmed");




    // Set rate limiting if the user specified a public RPC.
    if PUBLIC_RPC_URLS.contains(&rpc.as_str()) {
        warn!(
            "Using a public RPC URL is not recommended for heavy tasks as you will be rate-limited and suffer a performance hit.
        Please use a private RPC endpoint for best performance results."
        );
        *USE_RATE_LIMIT.write().unwrap() = true;
    }

    let commitment = CommitmentConfig::from_str(&commitment);
    let timeout = Duration::from_secs(timeout);

    let client = RpcClient::new_with_timeout_and_commitment(rpc.clone(), timeout, CommitmentConfig::confirmed());
    let full : bool = false;

    let mut list_file = String::new();
    list_file.push_str("");

    let mut account = String::new();
    account.push_str("AU7JnJ7wjmLchvy5q9j65sjSmqFSxwLyNYcGGwJnJyDQ");

    let mut output = String::new();
    output.push_str("/Users/omerduskin/tmp/tmp/");

    let result = decode_metadata_lib(&client, Some(&account), full, None, &output);
    match result {
        Ok(v) => return Ok(v.to_string()),
        Err(e) => return Ok(e.to_string()),
    }

//        let tmp = format!("{:?}", &result);
//    Ok("Rust says: ".to_owned() + val)
}
