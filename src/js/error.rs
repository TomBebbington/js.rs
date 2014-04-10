use js::value::{Value, ValueData, ResultValue, VUndefined, to_value};
use collections::treemap::TreeMap;
use std::gc::Gc;

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
	let error = to_value(make_error);
	let prototype = ValueData::new_obj();
	prototype.borrow().set_field(~"message", to_value(~""));
	prototype.borrow().set_field(~"name", to_value(~"Error"));
	prototype.borrow().set_field(~"toString", to_value(to_string));
	error.borrow().set_field(~"prototype", prototype);
	error
}