use stdlib::object::PROTOTYPE;
use stdlib::value::{Value, ValueData, ResultValue, VUndefined, to_value};
use std::gc::Gc;

/// Create a new error
pub fn make_error(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() >= 1 {
		this.borrow().set_field_slice("message", to_value(args.get(0).borrow().to_str().into_strbuf()));
	}
	Ok(Gc::new(VUndefined))
}
/// Get the string representation of the error
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let this_ptr = this.borrow();
	let name = this_ptr.get_field_slice("name");
	let message = this_ptr.get_field_slice("message");
	Ok(to_value(format!("{}: {}", name.borrow(), message.borrow()).into_strbuf()))
}
/// Create a new `Error` object
pub fn _create(global: Value) -> Value {
	let prototype = ValueData::new_obj(Some(global));
	let prototype_ptr = prototype.borrow();
	prototype_ptr.set_field_slice("message", to_value(""));
	prototype_ptr.set_field_slice("name", to_value("Error"));
	prototype_ptr.set_field_slice("toString", to_value(to_string));
	let error = to_value(make_error);
	let error_ptr = error.borrow();
	error_ptr.set_field_slice(PROTOTYPE, prototype);
	error
}
/// Initialise the global object with the `Error` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("Error", _create(global));
}