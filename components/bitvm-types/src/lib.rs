use core::fmt;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};

type CircuitId = u64;
type GateId = u64;

#[derive(Debug, PartialEq)]
pub struct BitCommitmentPreimages([u8; 32], [u8; 32]);

impl BitCommitmentPreimages {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let rng_0: u32 = rng.gen();
        let preimage_0 = Sha256::digest(rng_0.to_be_bytes());

        let rng_1: u32 = rng.gen();
        let preimage_1 = Sha256::digest(rng_1.to_be_bytes());

        Self(preimage_0.into(), preimage_1.into())
    }

    pub fn compute_bit_commitment_hashes(&self) -> BitCommitmentHashes {
        let hash_0 = Sha256::digest(&self.0);
        let hash_1 = Sha256::digest(&self.1);
        BitCommitmentHashes(hash_0.into(), hash_1.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct BitCommitmentHashes([u8; 32], [u8; 32]);

#[derive(Debug, PartialEq)]
pub struct Circuit {
    pub circuit_id: CircuitId,
    pub gates: HashMap<GateId, Gate>,
    pub gates_bit_commitments_preimages: HashMap<GateId, BitCommitmentPreimages>,
    pub reverse_lookup: HashMap<GateId, HashSet<GateId>>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            circuit_id: 0,
            gates: HashMap::new(),
            gates_bit_commitments_preimages: HashMap::new(),
            reverse_lookup: HashMap::new(),
        }
    }

    pub fn insert_gate(&mut self, gate_id: GateId, gate: Gate) {
        match gate {
            Gate::Nand(input_1, input_2)
            | Gate::And(input_1, input_2)
            | Gate::Xor(input_1, input_2) => {
                if !self.gates_bit_commitments_preimages.contains_key(&input_1) {
                    self.gates_bit_commitments_preimages
                        .insert(input_1, BitCommitmentPreimages::new());
                }
                if !self.gates_bit_commitments_preimages.contains_key(&input_2) {
                    self.gates_bit_commitments_preimages
                        .insert(input_2, BitCommitmentPreimages::new());
                }

                self.reverse_lookup
                    .entry(input_1)
                    .or_insert_with(HashSet::new)
                    .insert(gate_id);

                self.reverse_lookup
                    .entry(input_2)
                    .or_insert_with(HashSet::new)
                    .insert(gate_id);
            }
            Gate::Inv(input) => {
                if !self.gates_bit_commitments_preimages.contains_key(&input) {
                    self.gates_bit_commitments_preimages
                        .insert(input, BitCommitmentPreimages::new());
                }

                self.reverse_lookup
                    .entry(input)
                    .or_insert_with(HashSet::new)
                    .insert(gate_id);
            }
        }
        if !self.gates_bit_commitments_preimages.contains_key(&gate_id) {
            self.gates_bit_commitments_preimages
                .insert(gate_id, BitCommitmentPreimages::new());
        }
        self.gates.insert(gate_id, gate);
    }

    pub fn collect_top_level_gates_ids(&self) -> Vec<&GateId> {
        let mut hash_set = HashSet::new();
        for (_gate_id, gate) in self.gates.iter() {
            match gate {
                Gate::Nand(input_1, input_2)
                | Gate::And(input_1, input_2)
                | Gate::Xor(input_1, input_2) => {
                    if !self.gates.contains_key(input_1) {
                        hash_set.insert(input_1);
                    }
                    if !self.gates.contains_key(input_2) {
                        hash_set.insert(input_2);
                    }
                }
                Gate::Inv(input) => {
                    if !self.gates.contains_key(input) {
                        hash_set.insert(input);
                    }
                }
            }
        }
        let mut top_level_gates_ids = hash_set.into_iter().collect::<Vec<_>>();
        top_level_gates_ids.sort();
        top_level_gates_ids
    }

    pub fn collect_gates_bit_commitments_preimages<'a>(
        &'a self,
        gates_ids: &'a Vec<&'a GateId>,
    ) -> BTreeMap<&'a u64, &'a BitCommitmentPreimages> {
        let mut collected = BTreeMap::new();
        for gate_id in gates_ids.into_iter() {
            let Some(preimage) = self.gates_bit_commitments_preimages.get(gate_id) else {
                continue;
            };
            collected.insert(*gate_id, preimage);
        }
        collected
    }

    pub fn collect_subsequent_gates_bit_commitments_preimages<'a>(
        &'a self,
        gates_ids: &'a Vec<&'a GateId>,
    ) -> BTreeMap<&'a u64, &'a BitCommitmentPreimages> {
        let mut collected = BTreeMap::new();
        for gate_id in gates_ids.into_iter() {
            let Some(subsequent_gates) = self.reverse_lookup.get(gate_id) else {
                continue;
            };

            for gate in subsequent_gates.iter() {
                let Some(preimage) = self.gates_bit_commitments_preimages.get(gate) else {
                    continue;
                };
                collected.insert(*gate_id, preimage);
            }
        }
        collected
    }

    pub fn compute_bit_commitments_hashes<'a>(
        &'a self,
        preimages: &'a BTreeMap<&'a u64, &'a BitCommitmentPreimages>,
    ) -> BTreeMap<u64, BitCommitmentHashes> {
        let mut hashes = BTreeMap::new();
        for (gate_id, preimage) in preimages.into_iter() {
            hashes.insert(**gate_id, preimage.compute_bit_commitment_hashes());
        }
        hashes
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
