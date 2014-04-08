use js::value::{Value, VNumber, VInteger, VFunction, VObject, ResultValue};
use js::function::{NativeFunction, NativeFunc};
use js::object::ObjectData;
use collections::treemap::TreeMap;
use rand::random;
use std::io::stdio;
use std::f64;
use std::gc::Gc;
use std::cell::RefCell;

/// Get the absolute value of a number
pub fn abs(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().abs()
	} else {
		f64::NAN
	})))
}
/// Get the arccos of a number
pub fn acos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().acos()
	} else {
		f64::NAN
	})))
}
/// Get the arcsine of a number
pub fn asin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().asin()
	} else {
		f64::NAN
	})))
}
/// Get the arctangent of a number
pub fn atan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().atan()
	} else {
		f64::NAN
	})))
}
/// Get the arctangent of a numbers
pub fn atan2(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().atan2(&args.get(1).borrow().to_num())
	} else {
		f64::NAN
	})))
}
/// Get the cubic root of a number
pub fn cbrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().cbrt()
	} else {
		f64::NAN
	})))
}
/// Get lowest integer above a number
pub fn ceil(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().ceil()
	} else {
		f64::NAN
	})))
}
/// Get the cosine of a number
pub fn cos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().cos()
	} else {
		f64::NAN
	})))
}
/// Get the power to raise the natural logarithm to get the number
pub fn exp(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().exp()
	} else {
		f64::NAN
	})))
}
/// Get the highest integer below a number
pub fn floor(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().floor()
	} else {
		f64::NAN
	})))
}
/// Generate a random floating-point number between 0 and 1
pub fn _random(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(random())))
}
/// Get the sine of a number
pub fn sin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().sin()
	} else {
		f64::NAN
	})))
}
/// Get the tangent of a number
pub fn tan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().tan()
	} else {
		f64::NAN
	})))
}
/// Create a new 'Math' object
pub fn _create() -> Value {
	let mut math = TreeMap::new();
	math.insert(~"E", Gc::new(VNumber(f64::consts::E)));
	math.insert(~"LN2", Gc::new(VNumber(f64::consts::LN_2)));
	math.insert(~"LN10", Gc::new(VNumber(f64::consts::LN_10)));
	math.insert(~"LOG2E", Gc::new(VNumber(f64::consts::LOG2_E)));
	math.insert(~"LOG10E", Gc::new(VNumber(f64::consts::LOG10_E)));
	math.insert(~"SQRT1_2", Gc::new(VNumber(0.5f64.sqrt())));
	math.insert(~"SQRT2", Gc::new(VNumber(f64::consts::SQRT2)));
	math.insert(~"PI", Gc::new(VNumber(f64::consts::PI)));
	math.insert(~"abs", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(abs, 1))))));
	math.insert(~"acos", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(acos, 1))))));
	math.insert(~"asin", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(asin, 1))))));
	math.insert(~"atan", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(atan, 1))))));
	math.insert(~"atan2", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(atan2, 2))))));
	math.insert(~"cbrt", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(cbrt, 1))))));
	math.insert(~"ceil", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(ceil, 1))))));
	math.insert(~"cos", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(cos, 1))))));
	math.insert(~"exp", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(exp, 1))))));
	math.insert(~"floor", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(floor, 1))))));
	math.insert(~"random", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(_random, 0))))));
	math.insert(~"sin", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(sin, 1))))));
	math.insert(~"tan", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(tan, 1))))));
	Gc::new(VObject(RefCell::new(math)))
}