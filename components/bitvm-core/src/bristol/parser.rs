use bitvm_types::{Circuit, Gate};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bristol/bristol.gram"]
struct BristolParser;

pub fn read_circuit(circuit_source: &str) -> Result<Circuit, String> {
    let file = match BristolParser::parse(Rule::file, &circuit_source) {
        Ok(ref mut r) => r.next().unwrap(),
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };

    let mut circuit = Circuit::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::and => {
                let (gate_id, input_1_id, input_2_id) = parse_2_inputs_1_output(record);
                circuit
                    .gates
                    .insert(gate_id, Gate::And(input_1_id, input_2_id));
            }
            Rule::nand => {
                let (gate_id, input_1_id, input_2_id) = parse_2_inputs_1_output(record);
                circuit
                    .gates
                    .insert(gate_id, Gate::Nand(input_1_id, input_2_id));
            }
            Rule::xor => {
                let (gate_id, input_1_id, input_2_id) = parse_2_inputs_1_output(record);
                circuit
                    .gates
                    .insert(gate_id, Gate::Xor(input_1_id, input_2_id));
            }
            Rule::inv => {
                let (gate_id, input_id) = parse_1_input_1_output(record);
                circuit.gates.insert(gate_id, Gate::Inv(input_id));
            }
            Rule::EOI => (),
            _ => {}
        }
    }

    Ok(circuit)
}

pub fn parse_2_inputs_1_output(record: Pair<'_, Rule>) -> (u64, u64, u64) {
    let mut gate_id: u64 = 0;
    let mut input_1_id = 0;
    let mut input_2_id = 0;

    for (pos, field) in record.into_inner().enumerate() {
        if pos == 2 {
            gate_id = field
                .as_span()
                .as_str()
                .parse::<u64>()
                .expect("unable to parse");
        }
        if pos == 3 {
            input_1_id = field
                .as_span()
                .as_str()
                .parse::<u64>()
                .expect("unable to parse");
        }
        if pos == 4 {
            input_2_id = field
                .as_span()
                .as_str()
                .parse::<u64>()
                .expect("unable to parse");
        }
    }
    (gate_id, input_1_id, input_2_id)
}

pub fn parse_1_input_1_output(record: Pair<'_, Rule>) -> (u64, u64) {
    let mut gate_id: u64 = 0;
    let mut input_id = 0;

    for (pos, field) in record.into_inner().enumerate() {
        if pos == 2 {
            gate_id = field
                .as_span()
                .as_str()
                .parse::<u64>()
                .expect("unable to parse");
        }
        if pos == 3 {
            input_id = field
                .as_span()
                .as_str()
                .parse::<u64>()
                .expect("unable to parse");
        }
    }
    (gate_id, input_id)
}

#[test]
fn test_bristol_parse() {
    let circuit = read_circuit(include_str!("fixtures/test_vector_1.bristol"))
        .expect("unable to parse bristol");
    assert_eq!(
        format!("{}", circuit),
        "<Circuit id=0 and=5 inv=6 nand=1 xor=1>".to_string()
    )
}
