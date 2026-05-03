use wasm_bindgen::prelude::*;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde_json::json;
use hex;

#[wasm_bindgen]
pub struct KeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

#[wasm_bindgen]
impl KeyPair {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        Self {
            public_key: verifying_key.to_bytes().to_vec(),
            private_key: signing_key.to_bytes().to_vec(),
        }
    }
    
    pub fn public_key_hex(&self) -> String {
        hex::encode(&self.public_key)
    }
}

#[wasm_bindgen]
pub fn sign_message(message: &str, private_key_hex: &str) -> String {
    let private_key_bytes = hex::decode(private_key_hex).unwrap();
    let signing_key = SigningKey::from_bytes(&private_key_bytes.try_into().unwrap());
    let signature = signing_key.sign(message.as_bytes());
    hex::encode(signature.to_bytes())
}

#[wasm_bindgen]
pub fn verify_signature(message: &str, signature_hex: &str, public_key_hex: &str) -> bool {
    let public_key_bytes = hex::decode(public_key_hex).unwrap();
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap()).unwrap();
    let signature_bytes = hex::decode(signature_hex).unwrap();
    let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap());
    verifying_key.verify(message.as_bytes(), &signature).is_ok()
}

#[wasm_bindgen]
pub fn hash_message(message: &str) -> String {
    use blake3;
    hex::encode(blake3::hash(message.as_bytes()).as_bytes())
}

#[wasm_bindgen]
pub fn verify_kit_demo() -> String {
    let keypair = KeyPair::new();
    let message = "CAEP Treasury: $242,019.07";
    let signature = sign_message(message, &hex::encode(&keypair.private_key));
    let is_valid = verify_signature(message, &signature, &keypair.public_key_hex());
    
    json!({
        "status": "verified",
        "message": message,
        "signature_valid": is_valid,
        "public_key": keypair.public_key_hex(),
        "treasury": "$242,019.07",
        "seal": "🜏 SOVEREIGN SEALED 🜏"
    }).to_string()
}
