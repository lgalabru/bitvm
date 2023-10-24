use std::str::FromStr;

use bitcoin::{script::Builder, opcodes, ScriptBuf, taproot::{TaprootBuilder, LeafVersion}, bip32::DerivationPath};
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::secp256k1::PublicKey;
use bitcoin::bip32::ExtendedPrivKey;
use bitvm_types::{Circuit, BitCommitmentHashes};

pub fn compute_commitment_address(circuit: &Circuit, secp: &Secp256k1<secp256k1::All>, public_key: &PublicKey, other_public_key: &PublicKey) {
    let input_wires_ids = circuit.collect_input_wires_ids();
    let bit_commitment_preimages =
        circuit.collect_gates_bit_commitments_preimages(&input_wires_ids);

    let subsequent_commitment_preimages =
        circuit.collect_subsequent_gates_bit_commitments_preimages(&input_wires_ids);

    let mut tap_script_builder = Builder::new();
    let bit_commitments_hashes = circuit.compute_bit_commitments_hashes(&bit_commitment_preimages);
    for (_, bit_commitment_hash) in bit_commitments_hashes.iter() {
        tap_script_builder = augment_with_bit_commitment_leaf(tap_script_builder, bit_commitment_hash);
    }
    
    let subsequent_bit_commitments_hashes = circuit.compute_bit_commitments_hashes(&subsequent_commitment_preimages);
    for (_, bit_commitment_hash) in subsequent_bit_commitments_hashes.iter() {
        tap_script_builder = augment_with_bit_commitment_leaf(tap_script_builder, bit_commitment_hash);
    }
    let root_script = tap_script_builder
        .push_opcode(opcodes::all::OP_PUSHBYTES_1) // TODO: incomplete
        .into_script();

    // Leaf 1: Allows Vicky to spend the inputs of the bit commitment address after 10 blocks have passed.
    let script_1 = build_leaf_1(public_key);
    // Leaf 2: Allows Paul and Vicky to cooperatively sign a 2/2 multisig to spend the inputs at any time.
    let script_2 = build_leaf_2(public_key, other_public_key);
    // Leaf 3: Contains the actual bit commitment. Allows Paul to spend the inputs if he provides a valid execution trace.


    let xpriv= ExtendedPrivKey::new_master(bitcoin::Network::Regtest, &[0]).unwrap();
    let derivation_path = DerivationPath::from_str(&format!("m/101/1/0/0/1")).unwrap();
    let internal_keypair =
        xpriv.derive_priv(&secp, &derivation_path).unwrap().to_keypair(&secp);

    let taproot_builder = TaprootBuilder::new();
    let tree_info = taproot_builder
        .add_leaf(1, root_script.clone()).unwrap()
        .add_leaf(2, script_1.clone()).unwrap()
        .add_leaf(2, script_2.clone()).unwrap()
        .finalize(&secp, internal_keypair.x_only_public_key().0).unwrap();

    let output_key = tree_info.output_key();

    for script in vec![root_script, script_1, script_2] {
        let ver_script = (script, LeafVersion::TapScript);
        let ctrl_block = tree_info.control_block(&ver_script).unwrap();
        assert!(ctrl_block.verify_taproot_commitment(
            &secp,
            output_key.to_inner(),
            &ver_script.0
        ))
    }

    println!("-> {:?}", output_key);

}


pub fn build_leaf_1(public_key: &PublicKey) -> ScriptBuf {
    let script = Builder::new()
        .push_opcode(opcodes::all::OP_PUSHNUM_10) 
        .push_opcode(opcodes::all::OP_CSV)
        .push_slice(public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script();

        // if key.compressed {
        //     self.push_slice(key.inner.serialize())
        // } else {
        //     self.push_slice(key.inner.serialize_uncompressed())
        // }
    return script
}

pub fn build_leaf_2(public_key: &PublicKey, other_public_key: &PublicKey) -> ScriptBuf {
    let script: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_slice(public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_slice(other_public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_opcode(opcodes::all::OP_PUSHNUM_2)
        .push_opcode(opcodes::all::OP_EQUAL)
        .into_script();
    return script
}

pub fn augment_with_bit_commitment_leaf(mut builder: Builder, bit_commitment: &BitCommitmentHashes) -> Builder {
    builder
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(&bit_commitment.0)
        .push_opcode(opcodes::all::OP_EQUAL)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(&bit_commitment.1)
        .push_opcode(opcodes::all::OP_EQUAL)
        .push_opcode(opcodes::all::OP_BOOLOR)
        .push_opcode(opcodes::all::OP_VERIFY)
}
