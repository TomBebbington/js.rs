use js::value::{Value, VUndefined, ResultValue, to_value};
use std::gc::Gc;

/// Create a new boolean
pub fn make_boolean(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VUndefined))
}
/// Create a new `Boolean` object
pub fn _create(global: Value) -> Value {
	let boolean = to_value(make_boolean);
	boolean
}
/// Initialise the global object with the `Error` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("Boolean", _create(global));
}