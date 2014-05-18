use js::value::{Value, ValueData, VString, ResultValue, to_value};
use serialize::json::{ToJson, from_str};
use std::gc::Gc;
/// Parse a JSON string into a Javascript object
pub fn parse(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	match from_str(args.get(0).borrow().to_str()) {
		Ok(json) => {
			Ok(Gc::new(ValueData::from_json(json)))
		},
		Err(err) => {
			Err(Gc::new(VString(StrBuf::from_str(err.to_str()))))
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
	let obj = ValueData::new_obj(Some(global));
	obj.borrow().set_field("stringify".into_maybe_owned(), to_value(stringify));
	obj.borrow().set_field("parse".into_maybe_owned(), to_value(parse));
	obj
}
/// Initialise the global object with the `JSON` object
pub fn init(global:Value) {
	global.borrow().set_field("JSON".into_maybe_owned(), _create(global));
}