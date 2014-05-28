use stdlib::value::{Value, ResultValue, to_value};
use stdlib::function::Function;
use url::{encode, decode, encode_component, decode_component};

/// Encode a URI
pub fn encode_uri(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(encode(args.get(0).to_str().as_slice()))
	})
}
/// Encode a URI component
/// Rust uses RFC 3986, but standard Javascript doesn't, this will need a fix
pub fn encode_uri_component(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(encode_component(args.get(0).to_str().as_slice()))
	})
}
/// Decode a URI
pub fn decode_uri(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(decode(args.get(0).to_str().as_slice()))
	})
}
/// Decode a URI component
/// Rust uses RFC 3986, but standard Javascript doesn't, this will need a fix
pub fn decode_uri_component(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(decode_component(args.get(0).to_str().as_slice()))
	})
}
/// Initialise the URI functions on the global object
pub fn init(global:Value) {
	global.set_field_slice("encodeURI", Function::make(encode_uri, ["uri"]));
	global.set_field_slice("encodeURIComponent", Function::make(encode_uri_component, ["uri_comp"]));
	global.set_field_slice("decodeURI", Function::make(decode_uri, ["uri"]));
	global.set_field_slice("decodeURIComponent", Function::make(decode_uri_component, ["uri_comp"]));
}