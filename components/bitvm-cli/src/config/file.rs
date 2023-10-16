use crate::config::{Config, LogConfig, StorageConfig};
use chainhook_sdk::indexer::IndexerConfig;
use chainhook_sdk::types::{BitcoinBlockSignaling, BitcoinNetwork};
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub storage: StorageConfigFile,
    pub network: NetworkConfigFile,
    pub logs: Option<LogConfigFile>,
}

impl ConfigFile {
    pub fn from_file_path(file_path: &str) -> Result<Config, String> {
        let mut file = File::open(file_path)
            .map_err(|e| format!("unable to read file {}\n{:?}", file_path, e))?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .map_err(|e| format!("unable to read file {}\n{:?}", file_path, e))?;
        let config_file: ConfigFile = match toml::from_str(&file_content) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Config file malformatted {}", e.to_string()));
            }
        };
        ConfigFile::from_config_file(config_file)
    }

    pub fn from_config_file(config_file: ConfigFile) -> Result<Config, String> {
        let bitcoin_network = match config_file.network.mode.as_str() {
            "regtest" => BitcoinNetwork::Regtest,
            "testnet" => BitcoinNetwork::Testnet,
            "mainnet" => BitcoinNetwork::Mainnet,
            _ => return Err("network.mode not supported".to_string()),
        };

        let config = Config {
            storage: StorageConfig {
                working_dir: config_file.storage.working_dir.unwrap_or("bitvm".into()),
            },
            network: IndexerConfig {
                bitcoind_rpc_url: config_file.network.bitcoind_rpc_url.to_string(),
                bitcoind_rpc_username: config_file.network.bitcoind_rpc_username.to_string(),
                bitcoind_rpc_password: config_file.network.bitcoind_rpc_password.to_string(),
                bitcoin_block_signaling: match config_file.network.bitcoind_zmq_url {
                    Some(ref zmq_url) => BitcoinBlockSignaling::ZeroMQ(zmq_url.clone()),
                    None => unreachable!(),
                },
                bitcoin_network,
                stacks_network: chainhook_sdk::types::StacksNetwork::Devnet, // TODO: clean
            },
            logs: LogConfig {
                bitvm_internals: config_file
                    .logs
                    .as_ref()
                    .and_then(|l| l.bitvm_internals)
                    .unwrap_or(true),
                chainhook_internals: config_file
                    .logs
                    .as_ref()
                    .and_then(|l| l.chainhook_internals)
                    .unwrap_or(true),
            },
        };
        Ok(config)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LogConfigFile {
    pub bitvm_internals: Option<bool>,
    pub chainhook_internals: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StorageConfigFile {
    pub working_dir: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NetworkConfigFile {
    pub mode: String,
    pub bitcoind_rpc_url: String,
    pub bitcoind_rpc_username: String,
    pub bitcoind_rpc_password: String,
    pub bitcoind_zmq_url: Option<String>,
}
