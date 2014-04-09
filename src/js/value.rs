use js::object::ObjectData;
use js::function::{Function, NativeFunc, RegularFunc};
use collections::TreeMap;
use serialize::json;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};
use std::f64;
use std::gc::Gc;
use std::cell::RefCell;
static PROTOTYPE : &'static str = "__proto__";
#[must_use]
pub type ResultValue = Result<Value, Value>;
pub type Value = Gc<ValueData>;
#[deriving(Clone)]
/// Represents a Javascript value at runtime
pub enum ValueData {
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
	VObject(RefCell<ObjectData>),
	/// A value that is a function
	VFunction(RefCell<Function>)
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
		let obj : ObjectData = match *self {
			VObject(ref obj) => obj.borrow().clone(),
			VFunction(ref func) => {
				let func = func.borrow().clone();
				match func {
					NativeFunc(f) => f.object.clone(),
					RegularFunc(f) => f.object.clone()
				}
			},
			_ => return Gc::new(VUndefined)
		};
		match obj.find(&field) {
			Some(val) => *val,
			None => match obj.find(&PROTOTYPE.to_owned()) {
				Some(val) => 
					val.borrow().get_field(field),
				None => Gc::new(VUndefined)
			}
		}
	}
	/// Set the field in the value
	pub fn set_field(&self, field:~str, val:Value) -> Value {
		match *self {
			VObject(ref obj) => {
				obj.borrow_mut().insert(field, val);
			},
			VFunction(ref func) => {
				match *func.borrow_mut().deref_mut() {
					NativeFunc(ref mut f) => f.object.insert(field, val),
					RegularFunc(ref mut f) => f.object.insert(field, val)
				};
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
			VObject(ref v) => {
				try!(f.buf.write_str("{"));
				match v.borrow().iter().last() {
					Some((last_key, _)) => {
						for (key, val) in v.borrow().iter() {
							try!(write!(f.buf, "{}: {}", key, val.borrow()));
							if key != last_key {
								try!(f.buf.write_str(", "));
							}
						}
					},
					None => ()
				}
				f.buf.write_str("}")
			},
			VInteger(v) => write!(f.buf, "{}", v),
			VFunction(ref v) => {
				match v.borrow().clone() {
					NativeFunc(nf) => {
						try!(f.buf.write_str("function("));
						let mut letter = 'a';
						for i in range(0, nf.nargs) {
							try!(write!(f.buf, "{}", letter));
							letter = ((letter as u8) + 1u8) as char;
							if i < nf.nargs - 1 {
								try!(f.buf.write_str(", "));
							}
						}
						f.buf.write_str(") {...}")
					},
					RegularFunc(rf) => {
						try!(f.buf.write_str("function("));
						try!(f.buf.write_str(rf.args.connect(", ")));
						try!(f.buf.write_str(") "));
						write!(f.buf, "{}", rf.expr)
					}
				}
			}
		}
	}
}
impl Eq for ValueData {
	fn eq(&self, other:&ValueData) -> bool {
		match (self.clone(), other.clone()) {
			(VNull, VNull) | (VUndefined, VUndefined) => true,
			(VBoolean(a), VBoolean(b)) if a == b => true,
			(VString(ref a), VString(ref b)) if a == b => true,
			(VNumber(a), VNumber(b)) if a == b => true,
			(VObject(ref a), VObject(ref b)) if self == other => true,
			(VInteger(a), VInteger(b)) if a == b => true,
			(VFunction(ref a), VFunction(ref b)) if self == other => true,
			_ => false
		}
	}
}
impl json::ToJson for ValueData {
	fn to_json( &self ) -> json::Json {
		match *self {
			VNull | VUndefined => json::Null,
			VBoolean(b) => json::Boolean(b),
			VObject(ref obj) => {
				let mut nobj = TreeMap::new();
				for (k, v) in obj.borrow().iter() {
					nobj.insert(k.clone(), v.borrow().to_json());
				}
				json::Object(~nobj)
			},
			VString(ref str) => json::String(str.to_owned()),
			VNumber(num) => json::Number(num),
			VInteger(val) => json::Number(val as f64),
			VFunction(_) => json::Null
		}
	}
}
impl Add<ValueData, ValueData> for ValueData {
	fn add(&self, other:&ValueData) -> ValueData {
		return match (self.clone(), other.clone()) {
			(VString(s), other) | (other, VString(s)) => VString(s.to_owned() + other.to_str()),
			(_, _) => VNumber(self.to_num() + other.to_num())
		}
	}
}
impl Sub<ValueData, ValueData> for ValueData {
	fn sub(&self, other:&ValueData) -> ValueData {
		return VNumber(self.to_num() - other.to_num());
	}
}
impl Mul<ValueData, ValueData> for ValueData {
	fn mul(&self, other:&ValueData) -> ValueData {
		return VNumber(self.to_num() * other.to_num());
	}
}
impl Div<ValueData, ValueData> for ValueData {
	fn div(&self, other:&ValueData) -> ValueData {
		return VNumber(self.to_num() / other.to_num());
	}
}
impl Rem<ValueData, ValueData> for ValueData {
	fn rem(&self, other:&ValueData) -> ValueData {
		return VNumber(self.to_num() % other.to_num());
	}
}
impl BitAnd<ValueData, ValueData> for ValueData {
	fn bitand(&self, other:&ValueData) -> ValueData {
		return VInteger(self.to_int() & other.to_int());
	}
}
impl BitOr<ValueData, ValueData> for ValueData {
	fn bitor(&self, other:&ValueData) -> ValueData {
		return VInteger(self.to_int() | other.to_int());
	}
}
impl BitXor<ValueData, ValueData> for ValueData {
	fn bitxor(&self, other:&ValueData) -> ValueData {
		return VInteger(self.to_int() ^ other.to_int());
	}
}
impl Shl<ValueData, ValueData> for ValueData {
	fn shl(&self, other:&ValueData) -> ValueData {
		return VInteger(self.to_int() << other.to_int());
	}
}
impl Shr<ValueData, ValueData> for ValueData {
	fn shr(&self, other:&ValueData) -> ValueData {
		return VInteger(self.to_int() >> other.to_int());
	}
}
impl Not<ValueData> for ValueData {
	fn not(&self) -> ValueData {
		return VInteger(!self.to_int());
	}
}