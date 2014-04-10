use js::function::{NativeFunction, NativeFunc};
use js::object::ObjectData;
use js::value::{Value, ValueData, ResultValue, VFunction, VString, VObject, VUndefined, to_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;

/// Create a new error
pub fn make_error(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() >= 1 {
		this.borrow().set_field(~"message", to_value(args.get(0).borrow().to_str()));
	}
	Ok(Gc::new(VUndefined))
}
/// Get the string representation of the error
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let name = this.borrow().get_field(~"name");
	let message = this.borrow().get_field(~"message");
	Ok(to_value(format!("{}: {}", name.borrow(), message.borrow())))
}
/// Create a new 'Error' object
pub fn _create() -> Value {
	let mut func = to_value(make_error);
	let mut prototype = ValueData::new_obj();
	prototype.borrow().set_field(~"message", to_value(~""));
	prototype.borrow().set_field(~"name", to_value(~"Error"));
	prototype.borrow().set_field(~"toString", to_value(to_string));
	func.borrow().set_field(~"prototype", prototype);
	func
}