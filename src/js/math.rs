use js::value::{Value, ValueData, ResultValue, to_value, from_value};
use collections::treemap::TreeMap;
use rand::random;
use std::f64;
use std::gc::Gc;

/// Get the absolute value of a number
pub fn abs(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().abs()
	} else {
		f64::NAN
	}))
}
/// Get the arccos of a number
pub fn acos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().acos()
	} else {
		f64::NAN
	}))
}
/// Get the arcsine of a number
pub fn asin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().asin()
	} else {
		f64::NAN
	}))
}
/// Get the arctangent of a number
pub fn atan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().atan()
	} else {
		f64::NAN
	}))
}
/// Get the arctangent of a numbers
pub fn atan2(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().atan2(&args.get(1).borrow().to_num())
	} else {
		f64::NAN
	}))
}
/// Get the cubic root of a number
pub fn cbrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().cbrt()
	} else {
		f64::NAN
	}))
}
/// Get lowest integer above a number
pub fn ceil(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().ceil()
	} else {
		f64::NAN
	}))
}
/// Get the cosine of a number
pub fn cos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().cos()
	} else {
		f64::NAN
	}))
}
/// Get the power to raise the natural logarithm to get the number
pub fn exp(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().exp()
	} else {
		f64::NAN
	}))
}
/// Get the highest integer below a number
pub fn floor(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().floor()
	} else {
		f64::NAN
	}))
}
/// Get the natural logarithm of a number
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().log(&f64::consts::E)
	} else {
		f64::NAN
	}))
}
/// Get the maximum of several numbers
pub fn max(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let mut max = f64::NEG_INFINITY;
	for arg in args.iter() {
		let num = arg.borrow().to_num();
		max = max.max(num);
	}
	Ok(to_value(max))
}
/// Get the minimum of several numbers
pub fn min(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let mut max = f64::INFINITY;
	for arg in args.iter() {
		let num = arg.borrow().to_num();
		max = max.min(num);
	}
	Ok(to_value(max))
}
/// Raise a number to a power
pub fn pow(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 2 {
		let num : f64 = from_value(*args.get(0)).unwrap();
		let power : f64 = from_value(*args.get(1)).unwrap();
		num.powf(&power)
	} else {
		f64::NAN
	}))
}
/// Generate a random floating-point number between 0 and 1
pub fn _random(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(random::<f64>()))
}
/// Round a number to the nearest integer
pub fn round(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().round()
	} else {
		f64::NAN
	}))
}
/// Get the sine of a number
pub fn sin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().sin()
	} else {
		f64::NAN
	}))
}
/// Get the square root of a number
pub fn sqrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().sqrt()
	} else {
		f64::NAN
	}))
}
/// Get the tangent of a number
pub fn tan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().tan()
	} else {
		f64::NAN
	}))
}
/// Create a new 'Math' object
pub fn _create() -> Value {
	let mut math = ValueData::new_obj();
	math.borrow().set_field(~"E", to_value(f64::consts::E));
	math.borrow().set_field(~"LN2", to_value(f64::consts::LN_2));
	math.borrow().set_field(~"LN10", to_value(f64::consts::LN_10));
	math.borrow().set_field(~"LOG2E", to_value(f64::consts::LOG2_E));
	math.borrow().set_field(~"LOG10E", to_value(f64::consts::LOG10_E));
	math.borrow().set_field(~"SQRT1_2", to_value(0.5f64.sqrt()));
	math.borrow().set_field(~"SQRT2", to_value(f64::consts::SQRT2));
	math.borrow().set_field(~"PI", to_value(f64::consts::PI));
	math.borrow().set_field(~"abs", to_value(abs));
	math.borrow().set_field(~"acos", to_value(acos));
	math.borrow().set_field(~"asin", to_value(asin));
	math.borrow().set_field(~"atan", to_value(atan));
	math.borrow().set_field(~"atan2", to_value(atan2));
	math.borrow().set_field(~"cbrt", to_value(cbrt));
	math.borrow().set_field(~"ceil", to_value(ceil));
	math.borrow().set_field(~"cos", to_value(cos));
	math.borrow().set_field(~"exp", to_value(exp));
	math.borrow().set_field(~"floor", to_value(floor));
	math.borrow().set_field(~"log", to_value(log));
	math.borrow().set_field(~"max", to_value(max));
	math.borrow().set_field(~"min", to_value(min));
	math.borrow().set_field(~"pow", to_value(pow));
	math.borrow().set_field(~"random", to_value(_random));
	math.borrow().set_field(~"round", to_value(round));
	math.borrow().set_field(~"sin", to_value(sin));
	math.borrow().set_field(~"sqrt", to_value(sqrt));
	math.borrow().set_field(~"tan", to_value(tan));
	math
}