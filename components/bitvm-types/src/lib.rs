use core::fmt;
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
            gates: BTreeMap::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Gate {
    Nand(GateId, GateId),
    Inv(GateId),
    And(GateId, GateId),
    Xor(GateId, GateId),
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stats_nand = 0;
        let mut stats_and = 0;
        let mut stats_xor = 0;
        let mut stats_inv = 0;
        for (_gate_id, gate) in self.gates.iter() {
            match gate {
                Gate::And(_, _) => stats_and += 1,
                Gate::Nand(_, _) => stats_nand += 1,
                Gate::Inv(_) => stats_inv += 1,
                Gate::Xor(_, _) => stats_xor += 1,
            }
        }
        write!(
            f,
            "<Circuit id={} and={stats_and} inv={stats_inv} nand={stats_nand} xor={stats_xor}>",
            self.circuit_id
        )
    }
}
