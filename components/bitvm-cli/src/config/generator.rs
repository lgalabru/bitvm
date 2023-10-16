use chainhook_sdk::types::BitcoinNetwork;

pub fn generate_config(network: &BitcoinNetwork) -> String {
    let network = format!("{:?}", network);
    let conf = format!(
        r#"[storage]
working_dir = "bitvm"

[network]
mode = "{network}"
bitcoind_rpc_url = "http://0.0.0.0:8332"
bitcoind_rpc_username = "devnet"
bitcoind_rpc_password = "devnet"
bitcoind_zmq_url = "tcp://0.0.0.0:18543"

[logs]
bitvm_internals = true
chainhook_internals = true
"#,
        network = network.to_lowercase(),
    );
    return conf;
}
