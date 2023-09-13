use core::*;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsValue};

#[wasm_bindgen]
#[cfg(feature = "console_error_panic_hook")]
pub fn exec_mosaic(buf: Clamped<Vec<u8>>, grain: u32, width: u32, height: u32) -> Vec<u8> {
    let result = core::exec_mosaic(buf.0, grain, width, height);
    result
}

#[wasm_bindgen]
#[cfg(feature = "console_error_panic_hook")]
pub fn create_proof(buf: Clamped<Vec<u8>>, width: u32, height: u32) -> Vec<u8> {
    let proof = create_img_proof(buf.0, width, height);
    // JsValue::from_serde(&proof).unwrap()
    proof
}

#[wasm_bindgen]
pub fn verify_proof(proof: Clamped<Vec<u8>>, small_buf: Clamped<Vec<u8>>, s_width: u32, s_height: u32) -> bool {
    verify_img(proof.0, small_buf.0, s_width, s_height)
}