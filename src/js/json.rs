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
/// Create a new 'JSON' object
pub fn _create() -> Value {
	let obj = ValueData::new_obj();
	obj.borrow().set_field(~"stringify", to_value(stringify));
	obj.borrow().set_field(~"parse", to_value(parse));
	obj
}