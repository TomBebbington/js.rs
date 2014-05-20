use js::object::PROTOTYPE;
use js::value::{Value, ValueData, ResultValue, VUndefined, to_value};
use std::gc::Gc;

/// Create a new error
pub fn make_error(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() >= 1 {
		this.borrow().set_field("message".into_maybe_owned(), to_value(args.get(0).borrow().to_str().into_maybe_owned()));
	}
	Ok(Gc::new(VUndefined))
}
/// Get the string representation of the error
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let this_ptr = this.borrow();
	let name = this_ptr.get_field("name".into_maybe_owned());
	let message = this_ptr.get_field("message".into_maybe_owned());
	Ok(to_value(format!("{}: {}", name.borrow(), message.borrow()).into_maybe_owned()))
}
/// Create a new `Error` object
pub fn _create(global: Value) -> Value {
	let prototype = ValueData::new_obj(Some(global));
	let prototype_ptr = prototype.borrow();
	prototype_ptr.set_field("message".into_maybe_owned(), to_value("".into_maybe_owned()));
	prototype_ptr.set_field("name".into_maybe_owned(), to_value("Error".into_maybe_owned()));
	prototype_ptr.set_field("toString".into_maybe_owned(), to_value(to_string));
	let error = to_value(make_error);
	let error_ptr = error.borrow();
	error_ptr.set_field(PROTOTYPE.into_maybe_owned(), prototype);
	error
}
/// Initialise the global object with the `Error` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field("Error".into_maybe_owned(), _create(global));
}