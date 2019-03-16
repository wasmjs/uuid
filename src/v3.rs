use utils;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uuidv3(name: &str, namespace: &str) -> String {
	// https://rustwasm.github.io/docs/book/game-of-life/debugging.html#enable-logging-for-panics
	utils::set_panic_hook();
	let decoded = utils::clean_namespace(namespace);

	match Uuid::from_slice(&decoded) {
		Ok(uuid) => Uuid::new_v3(&uuid, name.as_bytes()).to_string(),
		Err(err) => panic!(
			"[uuidv3] Can't decode as uuid namespace '{:?}', error: '{:?}'",
			namespace, err
		),
	}
}
