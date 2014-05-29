use stdlib::value::{Value, ResultValue, to_value};
use stdlib::function::Function;
use serialize::json::{ToJson, from_str};
/// Parse a JSON string into a Javascript object
pub fn parse(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
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
pub fn stringify(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
	let obj = args.get(0);
	let json = obj.to_json();
	Ok(to_value(json.to_pretty_str()))
}
/// Create a new `JSON` object
pub fn _create(global:Value) -> Value {
	let object = Value::new_obj(Some(global));
	object.set_field_slice("stringify", Function::make(stringify, ["JSON"]));
	object.set_field_slice("parse", Function::make(parse, ["JSON_string"]));
	object
}
/// Initialise the global object with the `JSON` object
pub fn init(global:Value) {
	global.set_field_slice("JSON", _create(global));
}