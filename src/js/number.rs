use js::value::{Value, ResultValue, VFunction, VNumber, VInteger, VObject};
use js::function::{NativeFunc, NativeFunction};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use std::f64::NAN;
/// Parse a float
pub fn parse_float(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let parsed = from_str(args.get(0).borrow().to_str());
	return Ok(Gc::new(VNumber(match parsed {
		Some(v) => v,
		None => NAN
	})));
}
/// Parse an int
pub fn parse_int(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let parsed = from_str(args.get(0).borrow().to_str());
	return Ok(Gc::new(match parsed {
		Some(v) => VInteger(v),
		None => VNumber(NAN)
	}));
}
/// Initialise the parse functions on a global object
pub fn init(obj:Value) {
	obj.borrow().set_field(~"parseFloat", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_float, 1))))));
	obj.borrow().set_field(~"parseInt", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_int, 1))))));
}
/// Create a new 'Number' object
pub fn _create() -> Value {
	let mut number = TreeMap::new();
	number.insert(~"parseFloat", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_float, 1))))));
	number.insert(~"parseInt", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_int, 1))))));
	Gc::new(VObject(RefCell::new(number)))
}