use stdlib::value::{Value, ResultValue, VString, VUndefined, to_value};
use url::{encode, decode, encode_component, decode_component};
use std::gc::Gc;

/// Encode a URI
pub fn encode_uri(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(encode(args.get(0).borrow().to_str().as_slice()))
	}))
}
/// Encode a URI component
/// Rust uses RFC 3986, but standard Javascript doesn't, this will need a fix
pub fn encode_uri_component(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(encode_component(args.get(0).borrow().to_str().as_slice()))
	}))
}
/// Decode a URI
pub fn decode_uri(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(decode(args.get(0).borrow().to_str().as_slice()))
	}))
}
/// Decode a URI component
/// Rust uses RFC 3986, but standard Javascript doesn't, this will need a fix
pub fn decode_uri_component(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(decode_component(args.get(0).borrow().to_str().as_slice()))
	}))
}
/// Initialise the URI functions on the global object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("encodeURI", to_value(encode_uri));
	global_ptr.set_field_slice("encodeURIComponent", to_value(encode_uri_component));
	global_ptr.set_field_slice("decodeURI", to_value(decode_uri));
	global_ptr.set_field_slice("decodeURIComponent", to_value(decode_uri_component));
}