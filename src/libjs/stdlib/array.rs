use stdlib::value::{Value, VUndefined, ResultValue, to_value};
use std::gc::Gc;

/// Create a new array
pub fn make_array(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let this_ptr = this.borrow();
	this_ptr.set_field_slice("length", to_value(0i32));
	Ok(Gc::new(VUndefined))
}
/// Create a new `Array` object
pub fn _create(_: Value) -> Value {
	let array = to_value(make_array);
	array
}
/// Initialise the global object with the `Array` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("Array", _create(global));
}