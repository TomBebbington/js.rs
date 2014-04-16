use js::value::{Value, ValueData, VUndefined, ResultValue, to_value, from_value};
use js::object::Property;
use std::gc::Gc;
use std::str::MaybeOwned;

/// Create new string
pub fn make_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	this.borrow().set_field("length".into_maybe_owned(), to_value(0i32));
	Ok(Gc::new(VUndefined))
}
/// Get a string's length
pub fn get_string_length(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let this_str: MaybeOwned = from_value(this).unwrap();
	Ok(to_value::<i32>(this_str.len() as i32))
}
/// Create a new `String` object
pub fn _create(global: Value) -> Value {
	let string = to_value(make_string);
	let proto = ValueData::new_obj(Some(global));
	let prop = Property {
		configurable: false,
		enumerable: false,
		writable: false,
		value: Gc::new(VUndefined),
		get: to_value(get_string_length),
		set: Gc::new(VUndefined)
	};
	proto.borrow().set_prop("length".into_maybe_owned(), prop);
	string.borrow().set_field("prototype".into_maybe_owned(), proto);
	string
}
/// Initialise the global object with the `String` object
pub fn init(global:Value) {
	global.borrow().set_field("String".into_maybe_owned(), _create(global));
}