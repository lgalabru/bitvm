use crate::config::generator::generate_config;
use crate::config::Config;
use bitvm::SerializedCircuit;
use chainhook_sdk::bitcoincore_rpc::{Auth, Client, RpcApi};
use chainhook_sdk::types::BitcoinNetwork;
use chainhook_sdk::utils::Context;
use clap::{Parser, Subcommand};
use hiro_system_kit;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, PartialEq, Clone, Debug)]
enum Command {
    /// Generate a new configuration file
    #[clap(subcommand)]
    Config(ConfigCommand),
    /// Circuits management
    #[clap(subcommand)]
    Circuits(CircuitsCommand),
}

#[derive(Subcommand, PartialEq, Clone, Debug)]
#[clap(bin_name = "config")]
enum ConfigCommand {
    /// Generate new config
    #[clap(name = "new", bin_name = "new", aliases = &["generate"])]
    New(NewConfig),
}

#[derive(Parser, PartialEq, Clone, Debug)]
struct NewConfig {
    /// Target Regtest network
    #[clap(
        long = "regtest",
        conflicts_with = "testnet",
        conflicts_with = "mainnet"
    )]
    pub regtest: bool,
    /// Target Testnet network
    #[clap(
        long = "testnet",
        conflicts_with = "regtest",
        conflicts_with = "mainnet"
    )]
    pub testnet: bool,
    /// Target Mainnet network
    #[clap(
        long = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "regtest"
    )]
    pub mainnet: bool,
}

#[derive(Subcommand, PartialEq, Clone, Debug)]
#[clap(bin_name = "circuits", aliases = &["circuit"])]
enum CircuitsCommand {
    /// Generate new Bristol file
    #[clap(name = "new", bin_name = "new", aliases = &["generate"])]
    New(NewCircuit),
    /// Check Bristol file
    #[clap(name = "check", bin_name = "check")]
    Check(CheckCircuit),
    /// Simulate Bristol file
    #[clap(name = "simulate", bin_name = "simulate")]
    Simulate(SimulateCircuit),    
}

#[derive(Parser, PartialEq, Clone, Debug)]
struct NewCircuit {
    /// Bristol file path
    pub bristol_file_path: String,
}

#[derive(Parser, PartialEq, Clone, Debug)]
struct CheckCircuit {
    /// Bristol file path
    pub bristol_file_path: String,
}

#[derive(Parser, PartialEq, Clone, Debug)]
struct SimulateCircuit {
    /// Bristol file path
    pub bristol_file_path: String,
    /// Simulate as prover
    #[clap(
        long = "prover",
        conflicts_with = "verifier",
    )]
    pub prover: bool,
    /// Simulate as verifier
    #[clap(
        long = "verifier",
        conflicts_with = "prover",
    )]
    pub verifier: bool,   
}


pub fn main() {
    let logger = hiro_system_kit::log::setup_logger();
    let _guard = hiro_system_kit::log::setup_global_logger(logger.clone());
    let ctx = Context {
        logger: Some(logger),
        tracer: false,
    };

    let opts: Opts = match Opts::try_parse() {
        Ok(opts) => opts,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    match hiro_system_kit::nestable_block_on(handle_command(opts, &ctx)) {
        Err(e) => {
            error!(ctx.expect_logger(), "{e}");
            std::thread::sleep(std::time::Duration::from_millis(500));
            process::exit(1);
        }
        Ok(_) => {}
    }
}

async fn handle_command(opts: Opts, _ctx: &Context) -> Result<(), String> {
    match opts.command {
        Command::Config(subcmd) => match subcmd {
            ConfigCommand::New(cmd) => {
                let bitcoin_network = match (cmd.mainnet, cmd.testnet, cmd.regtest) {
                    (_, _, true) => BitcoinNetwork::Regtest,
                    (_, true, _) => BitcoinNetwork::Testnet,
                    (true, _, _) => BitcoinNetwork::Mainnet,
                    _ => return Err("network.mode not supported".to_string()),
                };

                let config_content = generate_config(&bitcoin_network);
                let mut file_path = PathBuf::new();
                file_path.push("BitVM.toml");
                let mut file = File::create(&file_path)
                    .map_err(|e| format!("unable to open config {}\n{}", file_path.display(), e))?;
                file.write_all(config_content.as_bytes()).map_err(|e| {
                    format!("unable to write config {}\n{}", file_path.display(), e)
                })?;
                println!("Created file BitVM.toml");
            }
        },
        Command::Circuits(subcmd) => match subcmd {
            CircuitsCommand::New(cmd) => {
                let circuit_content = bitvm::bristol::generator::create_template();
                let mut file_path = PathBuf::new();
                file_path.push(format!("{}.bristol", cmd.bristol_file_path));
                let mut file = File::create(&file_path).map_err(|e| {
                    format!("unable to create circuit {}\n{}", file_path.display(), e)
                })?;
                file.write_all(circuit_content.as_bytes()).map_err(|e| {
                    format!("unable to write config {}\n{}", file_path.display(), e)
                })?;
                println!("Created circuit {}.bristol", cmd.bristol_file_path);
            }
            CircuitsCommand::Check(cmd) => {
                let mut circuit_file: File = File::open(&cmd.bristol_file_path).map_err(|e| {
                    format!("unable to open circuit {}\n{}", cmd.bristol_file_path, e)
                })?;

                let mut circuit_content = String::new();
                circuit_file
                    .read_to_string(&mut circuit_content)
                    .map_err(|e| {
                        format!("unable to read circuit {}\n{}", cmd.bristol_file_path, e)
                    })?;
                let circuit =
                    bitvm::read_and_check_circuit(&SerializedCircuit::Bristol(&circuit_content))?;
                println!("{}", circuit);
            }
            CircuitsCommand::Simulate(cmd) => {
                let mut circuit_file: File = File::open(&cmd.bristol_file_path).map_err(|e| {
                    format!("unable to open circuit {}\n{}", cmd.bristol_file_path, e)
                })?;

                let mut circuit_content = String::new();
                circuit_file
                    .read_to_string(&mut circuit_content)
                    .map_err(|e| {
                        format!("unable to read circuit {}\n{}", cmd.bristol_file_path, e)
                    })?;
                let circuit =
                    bitvm::read_and_check_circuit(&SerializedCircuit::Bristol(&circuit_content))?;
                println!("{}", circuit);

            }
        },
    }
    Ok(())
}

pub async fn check_bitcoind_connection(config: &Config) -> Result<u64, String> {
    let auth = Auth::UserPass(
        config.network.bitcoind_rpc_username.clone(),
        config.network.bitcoind_rpc_password.clone(),
    );

    let bitcoin_rpc = match Client::new(&config.network.bitcoind_rpc_url, auth) {
        Ok(con) => con,
        Err(message) => {
            return Err(format!(
                "unable to connect to bitcoind: {}",
                message.to_string()
            ));
        }
    };

    let end_block = match bitcoin_rpc.get_blockchain_info() {
        Ok(result) => result.blocks,
        Err(e) => {
            return Err(format!("unable to connect to bitcoind: {}", e.to_string()));
        }
    };

    Ok(end_block)
}
