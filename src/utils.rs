use cfg_if::cfg_if;
use hex;

cfg_if! {
	// When the `console_error_panic_hook` feature is enabled, we can call the
	// `set_panic_hook` function at least once during initialization, and then
	// we will get better error messages if our code ever panics.
	//
	// For more details see
	// https://github.com/rustwasm/console_error_panic_hook#readme
	if #[cfg(feature = "console_error_panic_hook")] {
		extern crate console_error_panic_hook;
		pub use self::console_error_panic_hook::set_once as set_panic_hook;
	} else {
		#[inline]
		pub fn set_panic_hook() {}
	}
}

/// Namespace given by JS is a &str, possible hyphenated. We sanitize it, and
/// return the associated Vec<u8>.
pub fn clean_namespace(namespace: &str) -> Vec<u8> {
	match hex::decode(namespace.replace("-", "")) {
		Ok(decoded) => decoded,
		Err(err) => panic!(
			"[clean_namespace] Can't decode namespace as hex '{:?}', error: '{:?}'",
			namespace, err
		),
	}
}
