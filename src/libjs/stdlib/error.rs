use stdlib::object::PROTOTYPE;
use stdlib::value::{Value, ResultValue, to_value};

/// Create a new error
pub fn make_error(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() >= 1 {
		this.set_field_slice("message", to_value(args.get(0).to_str().into_string()));
	}
	Ok(Value::undefined())
}
/// Get the string representation of the error
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let name = this.get_field_slice("name");
	let message = this.get_field_slice("message");
	Ok(to_value(format!("{}: {}", name, message).into_string()))
}
/// Create a new `Error` object
pub fn _create(global: Value) -> Value {
	let prototype = Value::new_obj(Some(global));
	prototype.set_field_slice("message", to_value(""));
	prototype.set_field_slice("name", to_value("Error"));
	prototype.set_field_slice("toString", to_value(to_string));
	let error = to_value(make_error);
	error.set_field_slice(PROTOTYPE, prototype);
	error
}
/// Initialise the global object with the `Error` object
pub fn init(global:Value) {
	global.set_field_slice("Error", _create(global));
}