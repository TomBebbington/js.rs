use js::value::{Value, ResultValue, VString, VUndefined, to_value};
use url::{encode, decode, encode_component, decode_component};
use std::gc::Gc;

/// Encode a URI
pub fn encode_uri(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(encode(args.get(0).borrow().to_str()))
	}))
}
/// Encode a URI component
pub fn encode_uri_component(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(encode_component(args.get(0).borrow().to_str()))
	}))
}
/// Decode a URI
pub fn decode_uri(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(decode(args.get(0).borrow().to_str()))
	}))
}
/// Decode a URI component
pub fn decode_uri_component(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(if args.len() == 0 {
		VUndefined
	} else {
		VString(decode_component(args.get(0).borrow().to_str()))
	}))
}
/// Initialise the URI functions on a global object
pub fn init(obj:Value) {
	obj.borrow().set_field(~"encodeURI", to_value(encode_uri));
	obj.borrow().set_field(~"encodeURIComponent", to_value(encode_uri_component));
	obj.borrow().set_field(~"decodeURI", to_value(decode_uri));
	obj.borrow().set_field(~"decodeURIComponent", to_value(decode_uri_component));
}