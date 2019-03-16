extern crate cfg_if;
extern crate hex;
extern crate uuid;
extern crate wasm_bindgen;

mod utils;
pub mod v3;
pub mod v4;
pub mod v5;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
	// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
	// allocator.
	if #[cfg(feature = "wee_alloc")] {
		extern crate wee_alloc;
		#[global_allocator]
		static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
	}
}

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
	alert("Hello, uuid!");
}
