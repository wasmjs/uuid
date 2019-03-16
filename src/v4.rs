use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uuidv4() -> String {
    Uuid::new_v4().to_string()
}
