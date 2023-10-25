
use bitcoin::{script::Builder, opcodes, ScriptBuf, taproot::{TaprootBuilder, LeafVersion}, bip32::DerivationPath};
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::secp256k1::PublicKey;
use bitcoin::bip32::ExtendedPrivKey;
use bitvm_types::{Circuit, BitCommitmentHashes};

pub fn compute_challenge_address(circuit: &Circuit, secp: &Secp256k1<secp256k1::All>, public_key: &PublicKey, other_public_key: &PublicKey) {

    

}

pub fn build_tap_scripts_for_defectuous_inv_gate(input: &BitCommitmentHashes, output: &BitCommitmentHashes) -> Vec<ScriptBuf> {
    let case_input_0_output_0: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NOT)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .into_script();

    let case_input_1_output_1: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NOT)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .into_script();

    return vec![case_input_0_output_0, case_input_1_output_1]
}

pub fn build_tap_scripts_for_defectuous_and_gate(input_a: &BitCommitmentHashes, input_b: &BitCommitmentHashes, output: &BitCommitmentHashes) -> Vec<ScriptBuf> {
    let case_input_a_1_input_b_1_output_0: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_BOOLAND)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    let case_input_a_0_input_b_1_output_1: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_BOOLAND)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    let case_input_a_1_input_b_0_output_1: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_BOOLAND)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    let case_input_a_0_input_b_0_output_1: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_BOOLAND)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    return vec![case_input_a_1_input_b_1_output_0, case_input_a_0_input_b_1_output_1, case_input_a_1_input_b_0_output_1, case_input_a_0_input_b_0_output_1]
}

pub fn build_tap_scripts_for_defectuous_xor_gate(input_a: &BitCommitmentHashes, input_b: &BitCommitmentHashes, output: &BitCommitmentHashes) -> Vec<ScriptBuf> {

    let case_input_a_1_input_b_0_output_0: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    let case_input_a_0_input_b_1_output_0: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    let case_input_a_1_input_b_1_output_1: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    let case_input_a_0_input_b_0_output_1: ScriptBuf = Builder::new()
        .push_opcode(opcodes::all::OP_TOALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_a.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_SWAP)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(input_b.0)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_0)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_FROMALTSTACK)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(output.1)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_PUSHBYTES_1)
        .push_opcode(opcodes::all::OP_NUMNOTEQUAL)
        .push_opcode(opcodes::all::OP_VERIFY)
        .into_script();

    return vec![case_input_a_1_input_b_0_output_0, case_input_a_0_input_b_1_output_0, case_input_a_1_input_b_1_output_1, case_input_a_0_input_b_0_output_1]
}
