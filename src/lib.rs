use wasm_bindgen::prelude::*;
use verifykit_core::caep_document::{CaepDocumentV1, Claim};

/// Verify a CAEP document from JSON string
#[wasm_bindgen]
pub fn verify_caep_json(json: &str) -> bool {
    match serde_json::from_str::<CaepDocumentV1>(json) {
        Ok(doc) => doc.validate().is_ok(),
        Err(_) => false,
    }
}

/// Compute Merkle root from claims JSON array
#[wasm_bindgen]
pub fn compute_merkle_root(claims_json: &str) -> String {
    match serde_json::from_str::<Vec<Claim>>(claims_json) {
        Ok(claims) => {
            let doc = CaepDocumentV1::new(claims);
            doc.compute_root()
        }
        Err(_) => "invalid".to_string(),
    }
}

/// Get the ABI version
#[wasm_bindgen]
pub fn abi_version() -> String {
    "verifykit_caep_v1".to_string()
}
