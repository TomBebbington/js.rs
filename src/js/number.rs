use js::value::{Value, ResultValue, VFunction, VNumber, VInteger, VObject, VBoolean};
use js::function::{NativeFunc, NativeFunction};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use std::f64::{NAN, MAX_VALUE, MIN_VALUE, INFINITY, NEG_INFINITY, EPSILON};
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
/// Check if a value when converted to a number is finite
pub fn is_finite(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() == 0 {
		Ok(Gc::new(VBoolean(false)))
	} else {
		let num = args.get(0).borrow().to_num();
		Ok(Gc::new(VBoolean(num.is_finite())))
	}
}
/// Check if a number is finite
pub fn strict_is_finite(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() == 0 {
		Ok(Gc::new(VBoolean(false)))
	} else {
	let num = args.get(0).borrow();
		Ok(Gc::new(VBoolean(match *num {
			VNumber(v) => v.is_finite(),
			VInteger(_) => true, // integers can't be infinite
			_ => false
		})))
	}
}
/// Initialise the parse functions on a global object
pub fn init(obj:Value) {
	obj.borrow().set_field(~"parseFloat", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_float, 1))))));
	obj.borrow().set_field(~"parseInt", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_int, 1))))));
	obj.borrow().set_field(~"isFinite", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(is_finite, 1))))));
}
/// Create a new 'Number' object
pub fn _create() -> Value {
	let mut number = TreeMap::new();
	number.insert(~"NaN", Gc::new(VNumber(NAN)));
	number.insert(~"MAX_VALUE", Gc::new(VNumber(MAX_VALUE)));
	number.insert(~"MIN_VALUE", Gc::new(VNumber(MIN_VALUE)));
	number.insert(~"POSITIVE_INFINITY", Gc::new(VNumber(INFINITY)));
	number.insert(~"NEGATIVE_INFINITY", Gc::new(VNumber(NEG_INFINITY)));
	number.insert(~"EPSILON", Gc::new(VNumber(EPSILON)));
	number.insert(~"parseFloat", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_float, 1))))));
	number.insert(~"parseInt", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(parse_int, 1))))));
	number.insert(~"isFinite", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(strict_is_finite, 1))))));
	Gc::new(VObject(RefCell::new(number)))
}