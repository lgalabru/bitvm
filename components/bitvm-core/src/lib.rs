use bitvm_types::Circuit;

extern crate pest;

pub mod bristol;
pub mod circuit;

pub enum SerializedCircuit<'a> {
    Bristol(&'a str),
}

pub fn read_and_check_circuit(serialized_circuit: &SerializedCircuit) -> Result<Circuit, String> {
    let circuit = match serialized_circuit {
        SerializedCircuit::Bristol(src) => bristol::parser::read_circuit(src)?,
    };

    let top_level_gates_ids = circuit.collect_top_level_gates_ids();
    let bit_commitment_preimages =
        circuit.collect_gates_bit_commitments_preimages(&top_level_gates_ids);
    let subsequent_commitment_preimages =
        circuit.collect_subsequent_gates_bit_commitments_preimages(&top_level_gates_ids);

    Ok(circuit)
}
