use wasm_bindgen::prelude::*;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use serde_json::json;
use hex;

/// Generate a new Ed25519 keypair
/// Returns JSON with private_key (hex) and public_key (hex)
#[wasm_bindgen]
pub fn generate_keypair() -> String {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    json!({
        "private_key": hex::encode(signing_key.to_bytes()),
        "public_key": hex::encode(verifying_key.as_bytes())
    }).to_string()
}

/// Sign a message using a private key
/// Input: private_key_hex (32 bytes), message (string)
/// Returns: signature as hex string
#[wasm_bindgen]
pub fn sign_message(private_key_hex: &str, message: &str) -> String {
    let private_bytes = hex::decode(private_key_hex).unwrap();
    let signing_key = SigningKey::from_bytes(&private_bytes.try_into().unwrap());
    let signature = signing_key.sign(message.as_bytes());
    hex::encode(signature.to_bytes())
}

/// Verify a message signature
/// Input: public_key_hex (32 bytes), message (string), signature_hex (64 bytes)
/// Returns: true if signature is valid, false otherwise
#[wasm_bindgen]
pub fn verify_signature(public_key_hex: &str, message: &str, signature_hex: &str) -> bool {
    use ed25519_dalek::{Verifier, Signature};
    
    let pub_bytes = match hex::decode(public_key_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let verifying_key = match VerifyingKey::from_bytes(&pub_bytes.try_into().unwrap()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    let sig_bytes = match hex::decode(signature_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let signature = match Signature::from_bytes(&sig_bytes.try_into().unwrap()) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    verifying_key.verify(message.as_bytes(), &signature).is_ok()
}

/// Hash a message using BLAKE3 (domain separated)
#[wasm_bindgen]
pub fn hash_message(message: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"VERIFYKIT.v1.ATTESTATION");
    hasher.update(message.as_bytes());
    hex::encode(hasher.finalize().as_bytes())
}

/// Create a signed attestation bundle
/// Returns JSON with: message, signature, public_key
#[wasm_bindgen]
pub fn create_attestation(message: &str, private_key_hex: &str) -> String {
    let signature = sign_message(private_key_hex, message);
    let pub_bytes = {
        let sk = SigningKey::from_bytes(&hex::decode(private_key_hex).unwrap().try_into().unwrap());
        hex::encode(sk.verifying_key().as_bytes())
    };
    
    json!({
        "message": message,
        "signature": signature,
        "public_key": pub_bytes,
        "hash": hash_message(message)
    }).to_string()
}
