use core::fmt;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};

type CircuitId = u64;
type GateId = u64;

#[derive(Debug, PartialEq)]
pub struct BitCommitmentPreimage([u8; 32], [u8; 32]);

#[derive(Debug, PartialEq)]
pub struct BitCommitmentHash([u8; 32], [u8; 32]);

#[derive(Debug, PartialEq)]
pub struct GateInput {
    preimage: BitCommitmentPreimage,
    hash: BitCommitmentHash,
}

#[derive(Debug, PartialEq)]
pub struct GateOutput {
    preimage: BitCommitmentPreimage,
    hash: BitCommitmentHash,
}

#[derive(Debug, PartialEq)]
pub struct Circuit {
    pub circuit_id: CircuitId,
    pub gates: BTreeMap<GateId, Gate>,
    pub bit_commitments_inputs: HashMap<GateId, Vec<GateInput>>,
    pub bit_commitments_outputs: HashMap<GateId, GateOutput>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            circuit_id: 0,
            gates: BTreeMap::new(),
            bit_commitments_inputs: HashMap::new(),
            bit_commitments_outputs: HashMap::new(),
        }
    }

    pub fn insert_gate(&mut self, gate_id: GateId, gate: Gate) {
        self.bit_commitments_inputs
            .insert(gate_id, gate.compute_inputs_preimages_bits());
        self.bit_commitments_outputs
            .insert(gate_id, gate.compute_output_preimage_bit());
        self.gates.insert(gate_id, gate);
    }
}

#[derive(Debug, PartialEq)]
pub enum Gate {
    Nand(GateId, GateId),
    Inv(GateId),
    And(GateId, GateId),
    Xor(GateId, GateId),
}

pub fn generate_bit_commitment() -> (BitCommitmentPreimage, BitCommitmentHash) {
    let mut rng = thread_rng();
    let rng_0: u32 = rng.gen();
    let preimage_0 = Sha256::digest(rng_0.to_be_bytes());
    let hash_0 = Sha256::digest(&preimage_0);

    let rng_1: u32 = rng.gen();
    let preimage_1 = Sha256::digest(rng_1.to_be_bytes());
    let hash_1 = Sha256::digest(&preimage_1);

    (
        BitCommitmentPreimage(preimage_0.into(), preimage_1.into()),
        BitCommitmentHash(hash_0.into(), hash_1.into()),
    )
}

impl Gate {
    pub fn compute_inputs_preimages_bits(&self) -> Vec<GateInput> {
        match &self {
            Gate::Nand(_, _) | Gate::And(_, _) | Gate::Xor(_, _) => {
                let (bcp_input_1, bch_input_1) = generate_bit_commitment();
                let (bcp_input_2, bch_input_2) = generate_bit_commitment();
                vec![
                    GateInput {
                        preimage: bcp_input_1,
                        hash: bch_input_1,
                    },
                    GateInput {
                        preimage: bcp_input_2,
                        hash: bch_input_2,
                    },
                ]
            }
            Gate::Inv(_) => {
                let (bcp_input_1, bch_input_1) = generate_bit_commitment();
                vec![GateInput {
                    preimage: bcp_input_1,
                    hash: bch_input_1,
                }]
            }
        }
    }

    pub fn compute_output_preimage_bit(&self) -> GateOutput {
        let (bcp_output, bch_output) = generate_bit_commitment();
        GateOutput {
            preimage: bcp_output,
            hash: bch_output,
        }
    }
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
