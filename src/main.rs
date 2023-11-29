use vrf::generate_key_pair;
use weighted_selection::{read_candidates_from_file, generate_random_value_vrf, hash_to_number, choose_candidate_vrf, Candidate};
mod vrf;
mod weighted_selection;

fn main() {
   let candidates = generate_mock_candidates(100);
    let selection_counts = run_selections(&candidates, 1_000_000);
    // let selection_counts_standard = run_selections_standard_random(&candidates, 10_000);

    // Export to CSV
    export_to_csv(&candidates, &selection_counts, "output.csv")
        .expect("Failed to export to CSV");

    // // Read candidates from json file
    // let candidates = read_candidates_from_file("/Library/WebServer/Documents/uvrf/src/Candidates.json")
    //     .expect("Failed to read candidates");

    // // Generate ECC key pairs
    // let (sk, _) = generate_key_pair();
    // let vrf_input = b"some input";

    // // Generate vrf output by input with special string and sign key that generated before
    // let vrf_output = generate_random_value_vrf(&sk, vrf_input);

    // // Get number by vrf output
    // let random_number = hash_to_number(&vrf_output);

    // if let Some(selected_candidate) = choose_candidate_vrf(&candidates, random_number) {
    //     println!("Selected candidate: {:?}", selected_candidate);
    // } else {
    //     println!("No candidate was selected");
    // }
}

pub fn generate_mock_candidates(num_candidates: usize) -> Vec<Candidate> {
    (0..num_candidates).map(|i| Candidate {
        address: format!("0x{:x}", i),
        power: rand::random::<u32>() % 100, // Random power up to 100
    }).collect()
}

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_selections(candidates: &[Candidate], num_iterations: usize) -> Vec<usize> {
    let (sk, _) = generate_key_pair();
    let mut selection_counts = vec![0; candidates.len()];
    let mut counter = 0u64;

    for _ in 0..num_iterations {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        counter = counter.wrapping_add(1);
        let combined_input = [timestamp.to_ne_bytes(), counter.to_ne_bytes()].concat();

        // Hash the combined input
        let mut hasher = Sha256::new();
        hasher.update(combined_input);
        let hashed_input = hasher.finalize();
        let vrf_output = generate_random_value_vrf(&sk, &hashed_input);
        let random_number = hash_to_number(&vrf_output);
        if let Some(selected) = choose_candidate_vrf(&candidates, random_number) {
            let index = candidates.iter().position(|c| c == selected).unwrap();
            selection_counts[index] += 1;
        }
    }

    selection_counts
}



use std::error::Error;
use csv::{Writer, WriterBuilder};

pub fn export_to_csv(candidates: &[Candidate], selection_counts: &[usize], file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().has_headers(true).from_path(file_path)?;

    // Manually write the headers
    wtr.write_record(&["address", "power", "count"])?;

    for (candidate, &count) in candidates.iter().zip(selection_counts.iter()) {
        wtr.serialize((candidate.address.clone(), candidate.power, count))?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrf_random_selection_from_file() {
        let path = "Candidates.json"; // Adjust the path to your test JSON file
        let candidates = read_candidates_from_file(path).expect("Failed to read candidates");

        // Here, use the actual VRF function to generate a random value
        let (sk, _) = generate_key_pair();
        let vrf_input = b"test input"; // Use a constant input for reproducibility
        let vrf_output = generate_random_value_vrf(&sk, vrf_input);
        let random_number = hash_to_number(&vrf_output);
        let selected_candidate = choose_candidate_vrf(&candidates, random_number);

        
        println!("Random number : {}", random_number);

        if let Some(candidate) = selected_candidate {
            println!("Selected candidate address: {}", candidate.address);
        } else {
            println!("No candidate was selected");
        }

        assert!(selected_candidate.is_some());
        // You can add more assertions here
    }

    // ... other tests ...
}

