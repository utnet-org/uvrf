use vrf::generate_key_pair;
use weighted_selection::{read_candidates_from_file, generate_random_value_vrf, hash_to_number, choose_candidate_vrf};

mod vrf;
mod weighted_selection;

fn main() {
    // Example integration
    let candidates = read_candidates_from_file("Candidates.json")
        .expect("Failed to read candidates");

    let (sk, _) = generate_key_pair();
    let vrf_input = b"some input";
    let vrf_output = generate_random_value_vrf(&sk, vrf_input);
    let random_number = hash_to_number(&vrf_output);

    if let Some(selected_candidate) = choose_candidate_vrf(&candidates, random_number) {
        println!("Selected candidate: {:?}", selected_candidate);
    } else {
        println!("No candidate was selected");
    }
}
