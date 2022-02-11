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

use std::env;
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



use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::Serialize;
use serde_json::json;


use crate::decode::decode_metadata_lib;
use crate::mint::{mint_list, mint_one};
use crate::update_metadata::*;
//use crate::snapshot::{snapshot_cm_accounts, snapshot_holders, snapshot_mints};


fn setup_logging(log_level: String) -> Result<()> {
    let level = LevelFilter::from_str(log_level.as_str())?;
    Builder::new()
        .filter_level(level)
        .target(Target::Stdout)
        .init();
    Ok(())
}


#[derive(Debug, Serialize)]
pub struct JSONCreator {
    pub address: String,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

#[pyclass]
struct MonkeyBoss {
    client: RpcClient,
    keypair: String
}

#[pymethods]
impl MonkeyBoss {

    #[new]
    fn new(rpc: Option<String>, keypair: Option<String>, timeout: Option<u64>) -> Self {
        // set timeout
        let (timeout, ) = if let Some(timeout_sec) = timeout {
            (timeout_sec.clone(), )
        } else {
            (60,)
        };

        // set RPC url
        let (rpc, commitment) = if let Some(got_rpc) = rpc {
            (got_rpc.clone(), String::from("confirmed"))
        } else {
            let sol_config = parse_solana_config();
            if let Some(config) = sol_config {
                (config.json_rpc_url, config.commitment)
            } else {
                (
                    String::from("https://api.devnet.solana.com"),
                    String::from("confirmed"),
                )
            }
        };
        info!("using RPC {}", rpc);

        // set keypair url
        let (keypair, ) = if let Some(keypair) = keypair {
            (keypair.clone(), )
        } else {
            let sol_config = parse_solana_config();
            if let Some(config) = sol_config {
                (config.keypair_path, )
            } else {
                let cwd = env::home_dir().unwrap();
                let home_dir: String = cwd.as_os_str().to_str().unwrap().to_string();
                (String::from(format!("{}/.config/solana/id.json", home_dir)), )
            }
        };
        info!("using keypair {}", keypair);

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

        Self { client, keypair }
    }

    fn token_data(&self, account: String) -> PyResult<String> {
        let full : bool = false;

        let mut list_file = String::from("");
        let mut output = String::from("/Users/omerduskin/tmp/tmp/");

        let result = decode_metadata_lib(&self.client, Some(&account), full, None, &output);

        match result {
            Ok(v) => {
//                let s1: String = result.to_string();
//                let obj: serde_json::Value = serde_json::from_str(s1).unwrap();
                return Ok(v.to_string())
            },
            Err(e) => return Ok(e.to_string()),
        }
    }

    fn mint_one(&self) -> PyResult<()> {
        let mut nft_data_file = String::from("/Users/omerduskin/mint_data.json");
        let mut receiver = String::from("A16u3sNVzoYUWe46KyDc2YGdNqeHde66UmeSmb3Mxs52");
        let mut external_metadata_uri = None;

        let immutable : bool = false;
        let primary_sale_happened : bool = true;

        mint_one(&self.client,
            &self.keypair,
            &Some(receiver),
            Some(nft_data_file),
            external_metadata_uri.as_ref(),
            immutable,
            primary_sale_happened,
        );

        Ok(())
    }

    fn update_data(&self, account: String, new_data_file: String) -> PyResult<()> {
        update_data_one(&self.client, &self.keypair, &account, &new_data_file);

        Ok(())
    }

    fn update_uri(&self, account: String, new_uri: String) -> PyResult<()> {
        update_uri_one(&self.client, &self.keypair, &account, &new_uri);

        Ok(())
    }
}


#[pyfunction]
fn say_hello(wallet: String) -> PyResult<String> {
//    let decoded: Vec<u8> = bs58::decode(base58_string)
//        .into_vec()
//        .expect("Failed to decode base58 string");

//    let metadata: Metadata = try_from_slice_unchecked(&decoded).unwrap();
//
//    let creators = metadata
//        .data
//        .creators
//        .unwrap()
//        .iter()
//        .map(|c| JSONCreator {
//            address: c.address.to_string(),
//            verified: c.verified,
//            share: c.share,
//        })
//        .collect::<Vec<JSONCreator>>();
//
//    let nft_metadata = json!({
//        "name": metadata.data.name.to_string().trim_matches(char::from(0)),
//        "symbol": metadata.data.symbol.to_string().trim_matches(char::from(0)),
//        "seller_fee_basis_points": metadata.data.seller_fee_basis_points,
//        "uri": metadata.data.uri.to_string().trim_matches(char::from(0)),
//        "creators": [creators],
//    });

    Ok(wallet)
}

//#[pyfunction]
//fn mint_one(wallet: String) -> PyResult<String> {
//
//    mint_one(
//            &client,
//            &keypair,
//            &receiver,
//            nft_data_file,
//            external_metadata_uri.as_ref(),
//            immutable,
//            primary_sale_happened,
//        )
//
//    Ok(wallet)
//}



#[pyfunction]
fn token_data(account: String, rpc: Option<String>, timeout: Option<u64>) -> PyResult<String> {

    // set timeout
    let (timeout, ) = if let Some(timeout_sec) = timeout {
        (timeout_sec.clone(), )
    } else {
        (60,)
    };

    // read solana config
    let sol_config = parse_solana_config();

    // set RPC url
    let (rpc, commitment) = if let Some(got_rpc) = rpc {
        (got_rpc.clone(), String::from("confirmed"))
    } else {
        if let Some(config) = sol_config {
            (config.json_rpc_url, config.commitment)
        } else {
            (
                String::from("https://api.devnet.solana.com"),
                String::from("confirmed"),
            )
        }
    };
    info!("using RPC {}", rpc);

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

    let mut list_file = String::from("");
    let mut output = String::from("/Users/omerduskin/tmp/tmp/");

    let result = decode_metadata_lib(&client, Some(&account), full, None, &output);
    match result {
        Ok(v) => return Ok(v.to_string()),
        Err(e) => return Ok(e.to_string()),
    }

}

#[pymodule]
fn metaboss(_py: Python, m: &PyModule) -> PyResult<()> {
    setup_logging(String::from("Info"));

    m.add_class::<MonkeyBoss>()?;

    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(token_data, m)?)?;

    Ok(())
}
