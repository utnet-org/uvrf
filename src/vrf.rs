use p256::NistP256;
use p256::ecdsa::{SigningKey, Signature, signature::Signer};
use p256::ecdsa::{VerifyingKey, signature::Verifier};
use elliptic_curve::PublicKey;
use sha2::{Sha256, Digest};
use rand::thread_rng;

// Generate ECC key pair
pub fn generate_key_pair() -> (SigningKey, PublicKey<NistP256>) {
    let signing_key = SigningKey::random(&mut thread_rng());
    let verifying_key = signing_key.verifying_key();
    let public_key = PublicKey::from(verifying_key);
    (signing_key, public_key)
}

// Compute VRF with signing_key and input string, output verified random hash
pub fn compute_vrf(signing_key: &SigningKey, input: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(input);
    let signature: Signature = signing_key.sign(&hash);

    // Serialize the signature into bytes. Adjust according to the crate's API.
    let signature_bytes = signature.to_der().as_bytes().to_vec();
    signature_bytes
}


// Verify VRF result by public key and input string
pub fn verify_vrf(public_key: &PublicKey<NistP256>, input: &[u8], proof: &[u8]) -> bool {
    let hash = Sha256::digest(input);
    let verifying_key = VerifyingKey::from(public_key.clone());

    match Signature::from_der(proof) {
        Ok(signature) => verifying_key.verify(&hash, &signature).is_ok(),
        Err(_) => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::{Signer, Verifier}};
    use sha2::{Sha256, Digest};

    #[test]
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


