use chainhook_sdk::indexer::IndexerConfig;

pub mod file;
pub mod generator;

#[derive(Clone, Debug)]
pub struct Config {
    pub storage: StorageConfig,
    pub network: IndexerConfig,
    pub logs: LogConfig,
}

#[derive(Clone, Debug)]
pub struct LogConfig {
    pub bitvm_internals: bool,
    pub chainhook_internals: bool,
}

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub working_dir: String,
}
