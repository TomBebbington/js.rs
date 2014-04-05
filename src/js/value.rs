use js::object::ObjectData;
use js::function::Function;
use collections::TreeMap;
use serialize::json;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};
use std::f64;
use std::gc::Gc;
static PROTOTYPE : &'static str = "__proto__";
#[must_use]
pub type ResultValue = Result<Value, Value>;
#[deriving(Clone, Eq)]
pub type Value = Gc<ValueData>;
#[deriving(Clone)]
/// Represents a Javascript value at runtime
enum ValueData {
	/// The null value
	VNull,
	/// The undefined value
	VUndefined,
	/// A boolean true / false value
	VBoolean(bool),
	/// A string value
	VString(~str),
	/// A numeric value
	VNumber(f64),
	/// An integer value
	VInteger(i32),
	/// A value that is an object
	VObject(ObjectData),
	/// A value that is a function
	VFunction(Function)
}
impl ValueData {
	/// Returns true if the value is undefined
	pub fn is_undefined(&self) -> bool {
		return match *self {
			VUndefined => true,
			_ => false
		}
	}
	/// Returns true if the value is null
	pub fn is_null(&self) -> bool {
		return match *self {
			VNull => true,
			_ => false
		}
	}
	/// Returns true if the value is null or undefined
	pub fn is_null_or_undefined(&self) -> bool {
		return match *self {
			VNull | VUndefined => true,
			_ => false
		}
	}
	/// Returns true if the value is a 64-bit floating-point number
	pub fn is_double(&self) -> bool {
		return match *self {
			VNumber(_) => true,
			_ => false
		}
	}
	/// Returns true if the value is true
	pub fn is_true(&self) -> bool {
		return match *self {
			VObject(_) => true,
			VString(ref s) if *s == "1".to_owned() => true,
			VNumber(n) if n >= 1.0 && n % 1.0 == 0.0 => true,
			VInteger(n) if n > 1 => true,
			_ => false
		};
	}
	/// Converts the value into a 64-bit floating point number
	pub fn to_num(&self) -> f64 {
		return match *self {
			VObject(_) | VUndefined | VFunction(_) => f64::NAN,
			VString(ref str) => match from_str(*str) {
				Some(num) => num,
				None => f64::NAN
			},
			VNumber(num) => num,
			VBoolean(true) => 1.0,
			VBoolean(false) | VNull => 0.0,
			VInteger(num) => num as f64
		}
	}
	/// Converts the value into a 32-bit integer
	pub fn to_int(&self) -> i32 {
		return match *self {
			VObject(_) | VUndefined | VNull | VBoolean(false) | VFunction(_) => 0,
			VString(ref str) => match from_str(*str) {
				Some(num) => num,
				None => 0
			},
			VNumber(num) => num as i32,
			VBoolean(true) => 1,
			VInteger(num) => num
		}
	}
	/// Resolved the field in the value
	pub fn get_field(&self, field:~str) -> Value {
		let obj_data : &ObjectData = match *self {
			VObject(ref obj) => obj,
			VFunction(ref func) => &func.object,
			_ => return Gc::new(VUndefined)
		};
		let mut curr = obj_data;
		loop {
			match curr.find(&field) {
				Some(val) => return val.clone(),
				None => match curr.find(&PROTOTYPE.to_owned()) {
					Some(val) => match *val.borrow() {
						VObject(ref obj) => curr = obj,
						_ => ()
					},
					_ => return Gc::new(VUndefined)
				}
			}
		}
	}
	/// Set the field in the value
	pub fn set_field(&mut self, field:~str, val:Value) -> Value {
		match *self {
			VObject(ref mut obj) => {
				obj.swap(field, val.clone());
			},
			VFunction(ref mut func) => {
				func.object.swap(field, val.clone());
			},
			_ => ()
		}
		val
	}
}
impl fmt::Show for ValueData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			VNull => write!(f.buf, "null"),
			VUndefined => write!(f.buf, "undefined"),
			VBoolean(v) => write!(f.buf, "{}", v),
			VString(ref v) => write!(f.buf, "{}", v),
			VNumber(v) => write!(f.buf, "{}", v),
			VObject(ref v) => write!(f.buf, "{}", v),
			VInteger(v) => write!(f.buf, "{}", v),
			VFunction(ref v) => write!(f.buf, "function ...")
		}
	}
}
impl Eq for ValueData {
	fn eq(&self, other:&ValueData) -> bool {
		match (*self, *other) {
			(VNull, VNull) | (VUndefined, VUndefined) => true,
			(VBoolean(a), VBoolean(b)) if a == b => true,
			(VString(a), VString(b)) if a == b => true,
			(VNumber(a), VNumber(b)) if a == b => true,
			(VObject(a), VObject(b)) if a == b => true,
			(VInteger(a), VInteger(b)) if a == b => true,
			(VFunction(a), VFunction(b)) if a == b => true,
			_ => false
		}
	}
}
impl json::ToJson for Value {
	fn to_json( &self ) -> json::Json {
		match *self {
			VNull | VUndefined => json::Null,
			VBoolean(b) => json::Boolean(b),
			VObject(ref obj) => {
				let mut nobj = TreeMap::new();
				json::Object(~nobj)
			},
			VString(ref str) => json::String(str.to_owned()),
			VNumber(num) => json::Number(num),
			VInteger(val) => json::Number(val as f64),
			VFunction(_) => json::Null
		}
	}
}
impl Add<Value, Value> for Value {
	fn add(&self, other:&Value) -> Value {
		return match (self.clone(), other.clone()) {
			(VString(s), other) | (other, VString(s)) => VString(s.to_owned() + other.to_str()),
			(_, _) => VNumber(self.to_num() + other.to_num())
		}
	}
}
impl Sub<Value, Value> for Value {
	fn sub(&self, other:&Value) -> Value {
		return VNumber(self.to_num() - other.to_num());
	}
}
impl Mul<Value, Value> for Value {
	fn mul(&self, other:&Value) -> Value {
		return VNumber(self.to_num() * other.to_num());
	}
}
impl Div<Value, Value> for Value {
	fn div(&self, other:&Value) -> Value {
		return VNumber(self.to_num() / other.to_num());
	}
}
impl Rem<Value, Value> for Value {
	fn rem(&self, other:&Value) -> Value {
		return VNumber(self.to_num() % other.to_num());
	}
}
impl BitAnd<Value, Value> for Value {
	fn bitand(&self, other:&Value) -> Value {
		return VInteger(self.to_int() & other.to_int());
	}
}
impl BitOr<Value, Value> for Value {
	fn bitor(&self, other:&Value) -> Value {
		return VInteger(self.to_int() | other.to_int());
	}
}
impl BitXor<Value, Value> for Value {
	fn bitxor(&self, other:&Value) -> Value {
		return VInteger(self.to_int() ^ other.to_int());
	}
}
impl Shl<Value, Value> for Value {
	fn shl(&self, other:&Value) -> Value {
		return VInteger(self.to_int() << other.to_int());
	}
}
impl Shr<Value, Value> for Value {
	fn shr(&self, other:&Value) -> Value {
		return VInteger(self.to_int() >> other.to_int());
	}
}
impl Not<Value> for Value {
	fn not(&self) -> Value {
		return VInteger(!self.to_int());
	}
}
impl Index<Value, Value> for Value {
	fn index(&self, other:&Value) -> Value {
		return self.get_field(other.to_str());
	}
}