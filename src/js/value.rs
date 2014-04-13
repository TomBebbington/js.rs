use js::object::{ObjectData, Property};
use js::function::{Function, NativeFunc, RegularFunc, NativeFunction, NativeFunctionData};
use collections::TreeMap;
use serialize::json::{ToJson, Json, Number, String, Boolean, List, Object, Null};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};
use std::f64;
use std::gc::Gc;
use std::cell::RefCell;
use std::iter::FromIterator;
static PROTOTYPE : &'static str = "__proto__";
#[must_use]
/// The result of a Javascript expression is represented like this so it can succeed (`Ok`) or fail (`Err`)
pub type ResultValue = Result<Value, Value>;
/// A Garbage-collected Javascript value as represented in the interpreter
pub type Value = Gc<ValueData>;
#[deriving(Clone)]
/// A Javascript value
pub enum ValueData {
	/// `null` - A null value, such as the `parentNode` of a newly created HTML Element
	VNull,
	/// `undefined` - An undefined value, such as `{}.children`
	VUndefined,
	/// `boolean` - A `true` / `false` value, for if a certain criteria is met
	VBoolean(bool),
	/// `String` - A UTF-8 string, such as `"hello"`
	VString(~str),
	/// `Number` - A 64-bit floating point number, such as `-1293.0625`
	VNumber(f64),
	/// `Number` - A 32-bit integer, such as `0x5f3759df`
	VInteger(i32),
	/// `Object` - An object, such as `Math`, represented by a binary tree of string keys to javascript values
	VObject(RefCell<ObjectData>),
	/// `Function` - A runnable block of code, such as `Math.sqrt`, which can take some variables and return a useful value or act upon an object
	VFunction(RefCell<Function>)
}
impl ValueData {
	/// Returns a new empty object
	pub fn new_obj() -> Value {
		Gc::new(VObject(RefCell::new(TreeMap::new())))
	}
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
	/// Resolve the property in the object
	pub fn get_prop(&self, field:~str) -> Option<Property> {
		let obj : ObjectData = match *self {
			VObject(ref obj) => obj.borrow().clone(),
			VFunction(ref func) => {
				let func = func.borrow().clone();
				match func {
					NativeFunc(f) => f.object.clone(),
					RegularFunc(f) => f.object.clone()
				}
			},
			_ => return None
		};
		match obj.find(&field) {
			Some(val) => Some(*val),
			None => match obj.find(&PROTOTYPE.to_owned()) {
				Some(prop) => 
					prop.value.borrow().get_prop(field),
				None => None
			}
		}
	}
	/// Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist
	pub fn get_field(&self, field:~str) -> Value {
		match self.get_prop(field) {
			Some(prop) => prop.value,
			None => Gc::new(VUndefined)
		}
	}
	/// Set the field in the value
	pub fn set_field(&self, field:~str, val:Value) -> Value {
		match *self {
			VObject(ref obj) => {
				obj.borrow_mut().insert(field, Property::new(val));
			},
			VFunction(ref func) => {
				match *func.borrow_mut().deref_mut() {
					NativeFunc(ref mut f) => f.object.insert(field, Property::new(val)),
					RegularFunc(ref mut f) => f.object.insert(field, Property::new(val))
				};
			},
			_ => ()
		}
		val
	}
	/// Set the property in the value
	pub fn set_prop(&self, field:~str, prop:Property) -> Property {
		match *self {
			VObject(ref obj) => {
				obj.borrow_mut().insert(field, prop);
			},
			VFunction(ref func) => {
				match *func.borrow_mut().deref_mut() {
					NativeFunc(ref mut f) => f.object.insert(field, prop),
					RegularFunc(ref mut f) => f.object.insert(field, prop)
				};
			},
			_ => ()
		}
		prop
	}
	/// Convert from a JSON value to a JS value
	pub fn from_json(json:Json) -> ValueData {
		match json {
			Number(v) => VNumber(v),
			String(v) => VString(v),
			Boolean(v) => VBoolean(v),
			List(vs) => {
				let mut i = 0;
				let mut data : ObjectData = FromIterator::from_iter(vs.iter().map(|json| {
					i += 1;
					((i - 1).to_str(), Property::new(to_value(json.clone())))
				}));
				data.insert(~"length", Property::new(to_value(vs.len() as i32)));
				VObject(RefCell::new(data))
			},
			Object(obj) => {
				let data : ObjectData = FromIterator::from_iter(obj.iter().map(|(key, json)| {
					(key.clone(), Property::new(to_value(json.clone())))
				}));
				VObject(RefCell::new(data))
			},
			Null => VNull
		}
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
							try!(write!(f.buf, "{}: {}", key, val.value.borrow()));
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
					NativeFunc(_) => {
						f.buf.write_str("function() { [native code] }")
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
			(VInteger(a), VInteger(b)) if a == b => true,
			(VObject(_), VObject(_)) if self == other => true,
			(VFunction(_), VFunction(_)) if self == other => true,
			_ => false
		}
	}
}
impl ToJson for ValueData {
	fn to_json( &self ) -> Json {
		match *self {
			VNull | VUndefined => Null,
			VBoolean(b) => Boolean(b),
			VObject(ref obj) => {
				let mut nobj = TreeMap::new();
				for (k, v) in obj.borrow().iter() {
					if *k != ~"__proto__" {
						nobj.insert(k.clone(), v.value.borrow().to_json());
					}
				}
				Object(~nobj)
			},
			VString(ref str) => String(str.to_owned()),
			VNumber(num) => Number(num),
			VInteger(val) => Number(val as f64),
			VFunction(_) => Null
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
/// Conversion between Rust and Javascript values
pub trait ValueConv {
	/// Convert this Rust value into a Javascript value
	fn to_value(&self) -> Value;
	/// Convert a Javascript value to a Rust value
	fn from_value(v:Value) -> Option<Self>;
}
impl ValueConv for ~str {
	fn to_value(&self) -> Value {
		Gc::new(VString(self.clone()))
	}
	fn from_value(v:Value) -> Option<~str> {
		Some(v.borrow().to_str())
	}
}
impl ValueConv for f64 {
	fn to_value(&self) -> Value {
		Gc::new(VNumber(self.clone()))
	}
	fn from_value(v:Value) -> Option<f64> {
		Some(v.borrow().to_num())
	}
}
impl ValueConv for i32 {
	fn to_value(&self) -> Value {
		Gc::new(VInteger(self.clone()))
	}
	fn from_value(v:Value) -> Option<i32> {
		Some(v.borrow().to_int())
	}
}
impl ValueConv for bool {
	fn to_value(&self) -> Value {
		Gc::new(VBoolean(self.clone()))
	}
	fn from_value(v:Value) -> Option<bool> {
		Some(v.borrow().is_true())
	}
}
impl<T:ValueConv> ValueConv for Vec<T> {
	fn to_value(&self) -> Value {
		let mut arr = TreeMap::new();
		for i in range(0, self.len()) {
			arr.insert(i.to_str(), Property::new(self.get(i).to_value()));
		}
		to_value(arr)
	}
	fn from_value(v:Value) -> Option<Vec<T>> {
		let len = v.borrow().get_field(~"length").borrow().to_int();
		let mut vec = Vec::with_capacity(len as uint);
		for i in range(0, len) {
			match ValueConv::from_value(v.borrow().get_field(i.to_str())) {
				Some(v) => vec.push(v),
				None => ()
			}
		}
		Some(vec)
	}
}
impl ValueConv for NativeFunctionData {
	fn to_value(&self) -> Value {
		Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(*self)))))
	}
	fn from_value(v:Value) -> Option<NativeFunctionData> {
		match *v.borrow() {
			VFunction(ref func) => {
				match *func.borrow() {
					NativeFunc(ref data) => Some(data.data),
					_ => None
				}
			},
			_ => None
		}
	}
}
impl ValueConv for ObjectData {
	fn to_value(&self) -> Value {
		Gc::new(VObject(RefCell::new(self.clone())))
	}
	fn from_value(v:Value) -> Option<ObjectData> {
		match *v.borrow() {
			VObject(ref obj) => Some(obj.clone().borrow().deref().clone()),
			VFunction(ref func) => {
				Some(match *func.borrow().deref() {
					NativeFunc(ref data) => data.object.clone(),
					RegularFunc(ref data) => data.object.clone()
				})
			},
			_ => None
		}
	}
}
impl ValueConv for Json {
	fn to_value(&self) -> Value {
		Gc::new(ValueData::from_json(self.clone()))
	}
	fn from_value(v:Value) -> Option<Json> {
		Some(v.borrow().to_json())
	}
}
impl ValueConv for () {
	fn to_value(&self) -> Value {
		Gc::new(VNull)
	}
	fn from_value(_:Value) -> Option<()> {
		Some(())
	}
}
/// A utility function that just calls ValueConv::from_value
pub fn from_value<A: ValueConv>(v: Value) -> Option<A> {
	ValueConv::from_value(v)
}

/// A utility function that just calls ValueConv::to_value
pub fn to_value<A: ValueConv>(v: A) -> Value {
	v.to_value()
}