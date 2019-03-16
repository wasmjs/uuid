use utils;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uuidv5(name: &str, namespace: &str) -> String {
	// https://rustwasm.github.io/docs/book/game-of-life/debugging.html#enable-logging-for-panics
	utils::set_panic_hook();

	let decoded = utils::clean_namespace(namespace);

	match Uuid::from_slice(&decoded) {
		Ok(uuid) => Uuid::new_v5(&uuid, name.as_bytes()).to_string(),
		Err(err) => panic!(
			"[uuidv5] Can't decode namespace as uuid '{:?}', error: '{:?}'",
			namespace, err
		),
	}
}
