use utils;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uuidv4() -> String {
	// https://rustwasm.github.io/docs/book/game-of-life/debugging.html#enable-logging-for-panics
	utils::set_panic_hook();

	Uuid::new_v4().to_string()
}
