use js::value::{Value, ValueData, ResultValue, VFunction, VNumber, VInteger, VObject, VBoolean, to_value, from_value};
use js::function::{NativeFunc, NativeFunction};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use std::f64::{NAN, MAX_VALUE, MIN_VALUE, INFINITY, NEG_INFINITY, EPSILON};
/// Parse a float
pub fn parse_float(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let parsed = from_str::<f64>(from_value::<~str>(*args.get(0)).unwrap());
	return Ok(to_value(match parsed {
		Some(v) => v,
		None => NAN
	}));
}
/// Parse an int
pub fn parse_int(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let parsed = from_str::<i32>(from_value::<~str>(*args.get(0)).unwrap());
	return Ok(match parsed {
		Some(v) => to_value(v),
		None => to_value(NAN)
	});
}
/// Check if a value when converted to a number is finite
pub fn is_finite(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() == 0 {
		false
	} else {
		from_value::<f64>(*args.get(0)).unwrap().is_finite()
	}))
}
/// Check if a number is finite
pub fn strict_is_finite(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() == 0 {
		false
	} else {
		let num = args.get(0).borrow();
		match *num {
			VNumber(v) => v.is_finite(),
			VInteger(_) => true, // integers can't be infinite
			_ => false
		}
	}))
}
/// Check if a value when converted to a number is equal to NaN
pub fn is_nan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() == 0 {
		false
	} else {
		from_value::<f64>(*args.get(0)).unwrap().is_nan()
	}))
}
/// Check if a number is equal to NaN
pub fn strict_is_nan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() == 0 {
		false
	} else {
		let num = args.get(0).borrow();
		match *num {
			VNumber(v) => v.is_nan(),
			VInteger(_) => true, // integers can't be NaN
			_ => false
		}
	}))
}
/// Initialise the parse functions on a global object
pub fn init(obj:Value) {
	obj.borrow().set_field(~"NaN", to_value(NAN));
	obj.borrow().set_field(~"Infinity", to_value(INFINITY));
	obj.borrow().set_field(~"parseFloat", to_value(parse_float));
	obj.borrow().set_field(~"parseInt", to_value(parse_int));
	obj.borrow().set_field(~"isFinite", to_value(is_finite));
	obj.borrow().set_field(~"isNaN", to_value(is_nan));
}
/// Create a new 'Number' object
pub fn _create() -> Value {
	let mut number = ValueData::new_obj();
	number.borrow().set_field(~"NaN", to_value(NAN));
	number.borrow().set_field(~"MAX_VALUE", to_value(MAX_VALUE));
	number.borrow().set_field(~"MIN_VALUE", to_value(MIN_VALUE));
	number.borrow().set_field(~"POSITIVE_INFINITY", to_value(INFINITY));
	number.borrow().set_field(~"NEGATIVE_INFINITY", to_value(NEG_INFINITY));
	number.borrow().set_field(~"EPSILON", to_value(EPSILON));
	number.borrow().set_field(~"parseFloat", to_value(parse_float));
	number.borrow().set_field(~"parseInt", to_value(parse_int));
	number.borrow().set_field(~"isFinite", to_value(strict_is_finite));
	number.borrow().set_field(~"isNaN", to_value(strict_is_nan));
	number
}