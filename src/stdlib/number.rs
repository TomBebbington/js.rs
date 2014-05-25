use stdlib::value::{Value, ValueData, ResultValue, VNumber, VInteger, to_value, from_value};
use std::f64::{NAN, MAX_VALUE, MIN_VALUE, INFINITY, NEG_INFINITY, EPSILON};
/// Parse a float into a value
pub fn parse_float(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let parsed = from_str::<f64>(from_value::<StrBuf>(*args.get(0)).unwrap().as_slice());
	return Ok(to_value(match parsed {
		Some(v) => v,
		None => NAN
	}));
}
/// Parse an int into a value
pub fn parse_int(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let parsed = from_str::<i32>(from_value::<StrBuf>(*args.get(0)).unwrap().as_slice());
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
/// Create a new `Number` object
pub fn _create(global:Value) -> Value {
	let number = ValueData::new_obj(Some(global));
	let number_ptr = number.borrow();
	number_ptr.set_field_slice("NaN", to_value(NAN));
	number_ptr.set_field_slice("MAX_VALUE", to_value(MAX_VALUE));
	number_ptr.set_field_slice("MIN_VALUE", to_value(MIN_VALUE));
	number_ptr.set_field_slice("POSITIVE_INFINITY", to_value(INFINITY));
	number_ptr.set_field_slice("NEGATIVE_INFINITY", to_value(NEG_INFINITY));
	number_ptr.set_field_slice("EPSILON", to_value(EPSILON));
	number_ptr.set_field_slice("parseFloat", to_value(parse_float));
	number_ptr.set_field_slice("parseInt", to_value(parse_int));
	number_ptr.set_field_slice("isFinite", to_value(strict_is_finite));
	number_ptr.set_field_slice("isNaN", to_value(strict_is_nan));
	number
}
/// Initialise the parse functions and `Number` on the global object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("NaN", to_value(NAN));
	global_ptr.set_field_slice("Infinity", to_value(INFINITY));
	global_ptr.set_field_slice("parseFloat", to_value(parse_float));
	global_ptr.set_field_slice("parseInt", to_value(parse_int));
	global_ptr.set_field_slice("isFinite", to_value(is_finite));
	global_ptr.set_field_slice("isNaN", to_value(is_nan));
	global_ptr.set_field_slice("Number", _create(global));
}