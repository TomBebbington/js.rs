use js::value::{Value, VUndefined, ResultValue, to_value};
use std::gc::Gc;

/// Create new array
pub fn make_array(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	this.borrow().set_field(~"length", to_value(0i32));
	Ok(Gc::new(VUndefined))
}
/// Create a new `Array` object
pub fn _create() -> Value {
	let array = to_value(make_array);
	array
}