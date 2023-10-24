use bitcoin::secp256k1::SecretKey;
use bitcoin::secp256k1::{self, Secp256k1};
use bitvm_types::Circuit;
use tapleaf::commitment_address::compute_commitment_address;
use sha2::{Digest, Sha256};


extern crate pest;

pub mod bristol;
pub mod circuit;
pub mod tapleaf;

pub enum SerializedCircuit<'a> {
    Bristol(&'a str),
}

pub fn read_and_check_circuit(serialized_circuit: &SerializedCircuit) -> Result<Circuit, String> {
    let circuit = match serialized_circuit {
        SerializedCircuit::Bristol(src) => bristol::parser::read_circuit(src)?,
    };

    let secp: Secp256k1<secp256k1::All> = Secp256k1::new();

    let paul_secret = {
        let seed = Sha256::digest(&[0]);
        SecretKey::from_slice(&seed).unwrap()
    };
    let vicky_secret = {
        let seed = Sha256::digest(&[0]);
        SecretKey::from_slice(&seed).unwrap()
    };

    compute_commitment_address(&circuit, &secp, &paul_secret.public_key(&secp), &vicky_secret.public_key(&secp));
    Ok(circuit)
}
