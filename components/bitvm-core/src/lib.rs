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

    Ok(circuit)
}
