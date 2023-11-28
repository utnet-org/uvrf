use crate::vrf::compute_vrf;
use p256::ecdsa::SigningKey;
use serde::{Deserialize, Serialize};
use std::fs;
// ... other imports ...

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    name: String,
    weight: u32,
}

pub fn read_candidates_from_file(file_path: &str) -> Result<Vec<Candidate>, serde_json::Error> {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&file_content)
}

// Existing VRF functions here...

pub fn generate_random_value_vrf(sk: &SigningKey, input: &[u8]) -> Vec<u8> {
    compute_vrf(sk, input)
}

use std::convert::TryInto;

pub fn hash_to_number(hash: &[u8]) -> u32 {
    let (int_bytes, _) = hash.split_at(std::mem::size_of::<u32>());
    u32::from_ne_bytes(int_bytes.try_into().unwrap())
}

pub fn choose_candidate_vrf(candidates: &[Candidate], random_number: u32) -> Option<&Candidate> {
    let total_weight: u32 = candidates.iter().map(|c| c.weight).sum();
    let mut weighted_sum = 0;
    let target = random_number % total_weight;

    for candidate in candidates {
        weighted_sum += candidate.weight;
        if target < weighted_sum {
            return Some(candidate);
        }
    }

    None
}
