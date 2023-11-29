// src/api.rs
use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use crate::weighted_selection::{read_candidates_from_file, generate_random_value_vrf, hash_to_number, choose_candidate_vrf};
use crate::vrf::generate_key_pair;
use crate::Candidate;  // Assuming Candidate struct is defined in this module

#[derive(Serialize)]
struct ApiResponse<'a> {
    public_key: String,
    selected_candidate: &'a Candidate,
}

pub async fn get_candidates() -> impl Responder {
    let (sk, pk) = generate_key_pair();
    // Read candidates from json file
    let candidates = read_candidates_from_file("Candidates.json")
        .expect("Failed to read candidates");

    // Generate vrf output by input with special string and sign key that generated before
    let vrf_input = b"some input";
    let vrf_output = generate_random_value_vrf(&sk, vrf_input);

    // Get number by vrf output
    let random_number = hash_to_number(&vrf_output);

    if let Some(selected_candidate) = choose_candidate_vrf(&candidates, random_number) {
        HttpResponse::Ok().json(ApiResponse {
            public_key: format!("{:?}", pk),
            selected_candidate, // No clone needed
        })
    } else {
        HttpResponse::InternalServerError().body("No candidate was selected")
    }
}
