use p256::NistP256;
use p256::ecdsa::{SigningKey, Signature, signature::Signer};
use elliptic_curve::PublicKey;
use sha2::{Sha256, Digest};
use rand::thread_rng;
use generic_array::GenericArray;
use generic_array::typenum::U64;

pub fn generate_key_pair() -> (SigningKey, PublicKey<NistP256>) {
    let signing_key = SigningKey::random(&mut thread_rng());
    let verifying_key = signing_key.verifying_key();
    let public_key = PublicKey::from(verifying_key);
    (signing_key, public_key)
}

pub fn compute_vrf(signing_key: &SigningKey, input: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(input);
    let signature: Signature = signing_key.sign(&hash);

    // Serialize the signature into bytes. Adjust according to the crate's API.
    let signature_bytes = signature.to_der().as_bytes().to_vec();
    signature_bytes
}


use p256::ecdsa::{VerifyingKey, signature::Verifier};

pub fn verify_vrf(public_key: &PublicKey<NistP256>, input: &[u8], proof: &[u8]) -> bool {
    let hash = Sha256::digest(input);
    let verifying_key = VerifyingKey::from(public_key.clone());

    // Check if the proof length matches the expected length
    if proof.len() == 64 {
        let mut array = GenericArray::<u8, U64>::default();
        array.copy_from_slice(proof);

        if let Ok(signature) = Signature::from_bytes(&array) {
            verifying_key.verify(&hash, &signature).is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::{Signer, Verifier}};
    use sha2::{Sha256, Digest};

    #[test]
    // fn test_key_generation() {
    //     let (sk, pk) = generate_key_pair();
    //     // Perform assertions to check key generation
    //     assert!(!sk.to_bytes().is_empty());  // Adjust the test according to your key structure
    //     assert!(!pk.to_encoded_point(false).is_identity());
    // }
    fn test_key_generation() {
        let (sk, pk) = generate_key_pair();
        let signing_key = SigningKey::from(sk);
        let verifying_key: VerifyingKey = VerifyingKey::from(pk);

        let message = b"Test message";
        let hash = Sha256::digest(message);

        // Explicitly specify the type of `signature`
        let signature: Signature = signing_key.sign(&hash);

        // Use the verifying_key to verify the signature
        assert!(verifying_key.verify(&hash, &signature).is_ok());
    }

    #[test]
    fn test_vrf_computation() {
        let (sk, pk) = generate_key_pair();
        let input = b"Hello, VRF!";
        let proof = compute_vrf(&sk, input);

        // Check if the proof is not empty
        assert!(!proof.is_empty());
    }

    #[test]
    fn test_vrf_verification() {
        let (sk, pk) = generate_key_pair();
        let input = b"Hello, VRF!";
        let proof = compute_vrf(&sk, input);

        // Verify the proof and assert the result
        assert!(verify_vrf(&pk, input, &proof));
    }

    // Add more tests as needed
}


