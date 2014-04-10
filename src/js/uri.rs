use js::function::{NativeFunction, NativeFunc};
use js::value::{Value, ResultValue, VFunction, VString, VUndefined};
use url::{encode, decode, encode_component, decode_component};
use std::cell::RefCell;
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
	obj.borrow().set_field(~"encodeURI", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(encode_uri, 1))))));
	obj.borrow().set_field(~"encodeURIComponent", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(encode_uri_component, 1))))));
	obj.borrow().set_field(~"decodeURI", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(decode_uri, 1))))));
	obj.borrow().set_field(~"decodeURIComponent", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(decode_uri_component, 1))))));
}