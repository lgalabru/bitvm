use bitvm_types::Circuit;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bristol/bristol.gram"]
struct BristolParser;

pub fn read_circuit_bristol_format(circuit_source: &str) {
    let pairs = match BristolParser::parse(Rule::file, circuit_source) {
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
        Ok(res) => res
    };
    // pest

    let mut circuit = Circuit::new();
    for pair in pairs {
        println!("-> {:?}", pair);
    }
}
