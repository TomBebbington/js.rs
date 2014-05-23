use js::value::{Value, ValueData, VString, ResultValue, to_value};
use serialize::json::{ToJson, from_str};
use std::gc::Gc;
/// Parse a JSON string into a Javascript object
pub fn parse(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	match from_str(args.get(0).borrow().to_str().as_slice()) {
		Ok(json) => {
			Ok(Gc::new(ValueData::from_json(json)))
		},
		Err(err) => {
			Err(Gc::new(VString(err.to_str())))
		}
	}
}
/// Process a Javascript object into a JSON string
pub fn stringify(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	let json = (obj.borrow() as &ToJson).to_json();
	Ok(Gc::new(VString(json.to_pretty_str())))
}
/// Create a new `JSON` object
pub fn _create(global:Value) -> Value {
	let object = ValueData::new_obj(Some(global));
	let object_ptr = object.borrow();
	object_ptr.set_field_slice("stringify", to_value(stringify));
	object_ptr.set_field_slice("parse", to_value(parse));
	object
}
/// Initialise the global object with the `JSON` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("JSON", _create(global));
}