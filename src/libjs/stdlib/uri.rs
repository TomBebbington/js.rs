use stdlib::value::{Value, ResultValue, to_value};
use url::{encode, decode, encode_component, decode_component};

/// Encode a URI
pub fn encode_uri(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(encode(args.get(0).to_str().as_slice()))
	})
}
/// Encode a URI component
/// Rust uses RFC 3986, but standard Javascript doesn't, this will need a fix
pub fn encode_uri_component(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(encode_component(args.get(0).to_str().as_slice()))
	})
}
/// Decode a URI
pub fn decode_uri(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(decode(args.get(0).to_str().as_slice()))
	})
}
/// Decode a URI component
/// Rust uses RFC 3986, but standard Javascript doesn't, this will need a fix
pub fn decode_uri_component(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(if args.len() == 0 {
		Value::undefined()
	} else {
		to_value(decode_component(args.get(0).to_str().as_slice()))
	})
}
/// Initialise the URI functions on the global object
pub fn init(global:Value) {
	global.set_field_slice("encodeURI", to_value(encode_uri));
	global.set_field_slice("encodeURIComponent", to_value(encode_uri_component));
	global.set_field_slice("decodeURI", to_value(decode_uri));
	global.set_field_slice("decodeURIComponent", to_value(decode_uri_component));
}