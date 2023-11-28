use crate::vrf::compute_vrf;
use p256::ecdsa::SigningKey;
use serde::{Deserialize, Serialize};
use std::fs;
// ... other imports ...

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub address: String,  // Ethereum address as a hex string
    pub power: u32,       // Renamed from 'weight' to 'power'
}

// Read candidates from json file
pub fn read_candidates_from_file(file_path: &str) -> Result<Vec<Candidate>, serde_json::Error> {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&file_content)
}

// Existing VRF functions here
pub fn generate_random_value_vrf(sk: &SigningKey, input: &[u8]) -> Vec<u8> {
    compute_vrf(sk, input)
}

use std::convert::TryInto;

// Convert hash to number
pub fn hash_to_number(hash: &[u8]) -> u32 {
    let (int_bytes, _) = hash.split_at(std::mem::size_of::<u32>());
    u32::from_ne_bytes(int_bytes.try_into().unwrap())
}

// Choose candidate base on random number and weights of candidates
pub fn choose_candidate_vrf(candidates: &[Candidate], random_number: u32) -> Option<&Candidate> {

    // Get total weight of all the candidates
    let total_weight: u32 = candidates.iter().map(|c| c.power).sum();
    let mut weighted_sum = 0;

    // Get a target number that can pick from 0 ~ total_weight
    let target = random_number % total_weight;

    // Line candidate * power and get the candidate by target number
    for candidate in candidates {
        weighted_sum += candidate.power;
        if target < weighted_sum {
            return Some(candidate);
        }
    }

    None
}
