use vrf::generate_key_pair;
use weighted_selection::{read_candidates_from_file, generate_random_value_vrf, hash_to_number, choose_candidate_vrf};

mod vrf;
mod weighted_selection;

fn main() {
    // Read candidates from json file
    let candidates = read_candidates_from_file("/Library/WebServer/Documents/uvrf/src/Candidates.json")
        .expect("Failed to read candidates");

    // Generate ECC key pairs
    let (sk, _) = generate_key_pair();
    let vrf_input = b"some input";

    // Generate vrf output by input with special string and sign key that generated before
    let vrf_output = generate_random_value_vrf(&sk, vrf_input);

    // Get number by vrf output
    let random_number = hash_to_number(&vrf_output);

    if let Some(selected_candidate) = choose_candidate_vrf(&candidates, random_number) {
        println!("Selected candidate: {:?}", selected_candidate);
    } else {
        println!("No candidate was selected");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrf_random_selection_from_file() {
        let path = "/Library/WebServer/Documents/uvrf/src/Candidates.json"; // Adjust the path to your test JSON file
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

