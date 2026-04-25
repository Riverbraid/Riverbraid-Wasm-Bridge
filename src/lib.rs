use wasm_bindgen::prelude::*;
use riverbraid_tsh::{Validator, EvalInput};
use riverbraid_types::AnchorHash;

#[wasm_bindgen]
pub fn evaluate_state(anchor_bytes: &[u8], state_bytes: &[u8], sequence: u64) -> String {
    if anchor_bytes.len() != 32 {
        return serde_json::json!({
            "ok": false,
            "error": "Invalid anchor length"
        }).to_string();
    }

    let mut anchor_array = [0u8; 32];
    anchor_array.copy_from_slice(anchor_bytes);

    let input = EvalInput {
        anchor: AnchorHash(anchor_array),
        state_data: state_bytes.to_vec(),
        sequence,
    };

    let (seal, results) = Validator::evaluate(input);

    serde_json::json!({
        "ok": seal.is_some(),
        "seal": seal,
        "failures": results.into_iter().filter(|r| !r.passed).collect::<Vec<_>>()
    }).to_string()
}
