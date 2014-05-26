use stdlib::value::{Value, ValueData, VUndefined, ResultValue, to_value, from_value};
use stdlib::object::{PROTOTYPE, Property};
use std::gc::Gc;

/// Create new string
pub fn make_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	this.borrow().set_field_slice("length", to_value(0i32));
	Ok(Gc::new(VUndefined))
}
/// Get a string's length
pub fn get_string_length(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let this_str: String = from_value(this).unwrap();
	Ok(to_value::<i32>(this_str.len() as i32))
}
/// Create a new `String` object
pub fn _create(global: Value) -> Value {
	let string = to_value(make_string);
	let string_ptr = string.borrow();
	let proto = ValueData::new_obj(Some(global));
	let proto_ptr = proto.borrow();
	let prop = Property {
		configurable: false,
		enumerable: false,
		writable: false,
		value: Gc::new(VUndefined),
		get: to_value(get_string_length),
		set: Gc::new(VUndefined)
	};
	proto_ptr.set_prop("length".into_strbuf(), prop);
	string_ptr.set_field_slice(PROTOTYPE, proto);
	string
}
/// Initialise the `String` object on the global object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("String", _create(global));
}