//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasmjs_uuid;

use wasm_bindgen_test::*;
use wasmjs_uuid::v3::uuidv3;

wasm_bindgen_test_configure!(run_in_browser);

const MY_NAMESPACE: &str = "1b671a64-40d5-491e-99b0-da01ff1f3341";

#[wasm_bindgen_test]
fn uuidv3_should_work() {
	// https://github.com/kelektiv/node-uuid#version-3
	assert_eq!(
		uuidv3("hello world", MY_NAMESPACE),
		"042ffd34-d989-321c-ad06-f60826172424"
	);
}
