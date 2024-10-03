use serde_molecule::to_vec;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ProtoStruct {
    pub f1: u64,
    pub f2: u64,
}

#[wasm_bindgen]
pub fn main() -> Vec<u8> {
    let proto = ProtoStruct { f1: 1, f2: 2 };
    let bytes = to_vec(&proto, false).unwrap();
    bytes
}


#[wasm_bindgen]
pub fn proto_to_js() -> Result<JsValue, JsValue> {
    let proto = ProtoStruct { f1: 1, f2: 2 };

    Ok(serde_wasm_bindgen::to_value(&proto)?)
}