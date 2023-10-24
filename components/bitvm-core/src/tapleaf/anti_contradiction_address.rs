use std::str::FromStr;

use bitcoin::{script::Builder, opcodes, ScriptBuf, taproot::{TaprootBuilder, LeafVersion}, bip32::DerivationPath};
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::secp256k1::PublicKey;
use bitcoin::bip32::ExtendedPrivKey;
use bitvm_types::{Circuit, BitCommitmentHashes};

pub fn compute_anti_contradiction_address(circuit: &Circuit, secp: &Secp256k1<secp256k1::All>, public_key: &PublicKey, other_public_key: &PublicKey) {

    let input_wires_ids = circuit.collect_input_wires_ids();

    let bit_commitment_preimages =
        circuit.collect_gates_bit_commitments_preimages(&input_wires_ids);

    let subsequent_commitment_preimages =
        circuit.collect_subsequent_gates_bit_commitments_preimages(&input_wires_ids);

    let bit_commitments_hashes = circuit.compute_bit_commitments_hashes(&bit_commitment_preimages);
    let mut anti_contradiction_branches = vec![];
    for (_, bit_commitment_hashes) in bit_commitments_hashes.iter() {
        let script = build_anti_contradiciton_leaf(public_key, bit_commitment_hashes);
        anti_contradiction_branches.push(script);
    }
    
    let subsequent_bit_commitments_hashes = circuit.compute_bit_commitments_hashes(&subsequent_commitment_preimages);
    for (_, bit_commitment_hashes) in subsequent_bit_commitments_hashes.iter() {
        let script = build_anti_contradiciton_leaf(public_key, bit_commitment_hashes);
        anti_contradiction_branches.push(script);
    }

    // Leaf 1: Allows Vicky to spend the inputs of the bit commitment address after 10 blocks have passed.
    let slashing_script = build_slashing_leave(public_key);



    let xpriv= ExtendedPrivKey::new_master(bitcoin::Network::Regtest, &[0]).unwrap();
    let derivation_path = DerivationPath::from_str(&format!("m/101/1/0/0/1")).unwrap();
    let internal_keypair =
        xpriv.derive_priv(&secp, &derivation_path).unwrap().to_keypair(&secp);

    let mut depth_max = 1;
    let mut breadth = anti_contradiction_branches.len();
    while breadth > 1 {
        breadth >>= 1;
        depth_max += 1;
    }

    let mut taproot_builder = TaprootBuilder::new();
    for script in anti_contradiction_branches.into_iter() {
        taproot_builder = taproot_builder.add_leaf(depth_max, script).unwrap();
    }
    let tree_info = taproot_builder.add_leaf(depth_max, slashing_script).unwrap()
        .finalize(&secp, internal_keypair.x_only_public_key().0).unwrap();

    let output_key = tree_info.output_key();

    // for script in vec![root_script, script_1, script_2] {
    //     let ver_script = (script, LeafVersion::TapScript);
    //     let ctrl_block = tree_info.control_block(&ver_script).unwrap();
    //     assert!(ctrl_block.verify_taproot_commitment(
    //         &secp,
    //         output_key.to_inner(),
    //         &ver_script.0
    //     ))
    // }

    println!("-> {:?}", output_key);

}


pub fn build_anti_contradiciton_leaf(public_key: &PublicKey, bit_commitment: &BitCommitmentHashes) -> ScriptBuf {
    let script = Builder::new()
        .push_opcode(opcodes::all::OP_SHA256) 
        .push_slice(&bit_commitment.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(&bit_commitment.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHNUM_1)
        .into_script();
    //todo: replace the last OP_1 with these two lines:
    //<Vicky’s key>
    //OP_CHECKSIG
    return script
}

pub fn build_slashing_leave(public_key: &PublicKey) -> ScriptBuf {
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
