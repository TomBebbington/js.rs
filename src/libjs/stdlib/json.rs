use stdlib::value::{Value, ValueData, VString, ResultValue, to_value};
use serialize::json::{ToJson, from_str};
use std::gc::Gc;
/// Parse a JSON string into a Javascript object
pub fn parse(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	match from_str(args.get(0).to_str().as_slice()) {
		Ok(json) => {
			Ok(to_value(json))
		},
		Err(err) => {
			Err(to_value(err.to_str()))
		}
	}
}
/// Process a Javascript object into a JSON string
pub fn stringify(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	let json = obj.to_json();
	Ok(to_value(json.to_pretty_str()))
}
/// Create a new `JSON` object
pub fn _create(global:Value) -> Value {
	let object = Value::new_obj(Some(global));
	object.set_field_slice("stringify", to_value(stringify));
	object.set_field_slice("parse", to_value(parse));
	object
}
/// Initialise the global object with the `JSON` object
pub fn init(global:Value) {
	global.set_field_slice("JSON", _create(global));
}