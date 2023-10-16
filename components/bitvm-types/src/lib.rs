use std::collections::BTreeMap;

type CircuitId = u64;

type GateId = u64;

#[derive(Debug, PartialEq)]
pub struct Circuit {
    pub circuit_id: CircuitId,
    pub gates: BTreeMap<GateId, Gate>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit { 
            circuit_id: 0, 
            gates: BTreeMap::new()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Gate {
    Nand(GateId, GateId),
    Inv(GateId),
    And(GateId, GateId),
}
