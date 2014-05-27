use stdlib::value::{Value, ValueData, VUndefined, ResultValue, to_value, from_value};
use stdlib::object::{PROTOTYPE, Property};
use std::gc::Gc;

/// Create new string
pub fn make_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	this.set_field_slice("length", to_value(0i32));
	Ok(Value::undefined())
}
/// Get a string's length
pub fn get_string_length(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let this_str: String = from_value(this).unwrap();
	Ok(to_value::<i32>(this_str.len() as i32))
}
/// Create a new `String` object
pub fn _create(global: Value) -> Value {
	let string = to_value(make_string);
	let proto = Value::new_obj(Some(global));
	let prop = Property {
		configurable: false,
		enumerable: false,
		writable: false,
		value: Value::undefined(),
		get: to_value(get_string_length),
		set: Value::undefined()
	};
	proto.set_prop("length".into_strbuf(), prop);
	string.set_field_slice(PROTOTYPE, proto);
	string
}
/// Initialise the `String` object on the global object
pub fn init(global:Value) {
	global.set_field_slice("String", _create(global));
}