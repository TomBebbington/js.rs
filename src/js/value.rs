use js::object::{PROTOTYPE, INSTANCE_PROTOTYPE, ObjectData, Property};
use js::function::{Function, NativeFunc, RegularFunc, NativeFunction, NativeFunctionData};
use collections::TreeMap;
use serialize::json::{ToJson, Json, Number, String, Boolean, List, Object, Null};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};
use std::f64;
use std::str::MaybeOwned;
use std::gc::Gc;
use std::cell::RefCell;
use std::iter::FromIterator;
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
	VString(StrBuf),
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
	pub fn new_obj(global: Option<Value>) -> Value {
		let mut obj : ObjectData = TreeMap::new();
		if global.is_some() {
			let obj_proto = global.unwrap().borrow().get_field("Object".into_maybe_owned()).borrow().get_field(PROTOTYPE.into_maybe_owned());
			obj.insert(INSTANCE_PROTOTYPE.to_owned(), Property::new(obj_proto));
		}
		Gc::new(VObject(RefCell::new(obj)))
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
			VString(ref s) if s.as_slice() == "1" => true,
			VNumber(n) if n >= 1.0 && n % 1.0 == 0.0 => true,
			VInteger(n) if n > 1 => true,
			VBoolean(v) => v,
			_ => false
		};
	}
	/// Converts the value into a 64-bit floating point number
	pub fn to_num(&self) -> f64 {
		return match *self {
			VObject(_) | VUndefined | VFunction(_) => f64::NAN,
			VString(ref str) => match from_str(str.as_slice()) {
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
			VString(ref str) => match from_str(str.as_slice()) {
				Some(num) => num,
				None => 0
			},
			VNumber(num) => num as i32,
			VBoolean(true) => 1,
			VInteger(num) => num
		}
	}
	/// Resolve the property in the object
	pub fn get_prop(&self, field:MaybeOwned) -> Option<Property> {
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
		match obj.find(&field.clone().into_owned()) {
			Some(val) => Some(*val),
			None => match obj.find(&PROTOTYPE.to_owned()) {
				Some(prop) => 
					prop.value.borrow().get_prop(field),
				None => None
			}
		}
	}
	/// Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist
	pub fn get_field(&self, field:MaybeOwned) -> Value {
		match self.get_prop(field) {
			Some(prop) => prop.value,
			None => Gc::new(VUndefined)
		}
	}
	/// Set the field in the value
	pub fn set_field(&self, field:MaybeOwned, val:Value) -> Value {
		match *self {
			VObject(ref obj) => {
				obj.borrow_mut().insert(field.into_owned(), Property::new(val));
			},
			VFunction(ref func) => {
				match *func.borrow_mut().deref_mut() {
					NativeFunc(ref mut f) => f.object.insert(field.into_owned(), Property::new(val)),
					RegularFunc(ref mut f) => f.object.insert(field.into_owned(), Property::new(val))
				};
			},
			_ => ()
		}
		val
	}
	/// Set the property in the value
	pub fn set_prop(&self, field:MaybeOwned, prop:Property) -> Property {
		match *self {
			VObject(ref obj) => {
				obj.borrow_mut().insert(field.into_owned(), prop);
			},
			VFunction(ref func) => {
				match *func.borrow_mut().deref_mut() {
					NativeFunc(ref mut f) => f.object.insert(field.into_owned(), prop),
					RegularFunc(ref mut f) => f.object.insert(field.into_owned(), prop)
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
				data.insert("length".to_owned(), Property::new(to_value(vs.len() as i32)));
				VObject(RefCell::new(data))
			},
			Object(obj) => {
				let data : ObjectData = FromIterator::from_iter(obj.iter().map(|(key, json)| {
					(key.to_owned(), Property::new(to_value(json.clone())))
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
			VNull => write!(f, "null"),
			VUndefined => write!(f, "undefined"),
			VBoolean(v) => write!(f, "{}", v),
			VString(ref v) => write!(f, "{}", v),
			VNumber(v) => write!(f, "{}", v),
			VObject(ref v) => {
				try!(write!(f, "{}", "{"));
				match v.borrow().iter().last() {
					Some((last_key, _)) => {
						for (key, val) in v.borrow().iter() {
							try!(write!(f, "{}: {}", key, val.value.borrow()));
							if key != last_key {
								try!(write!(f, "{}", ", "));
							}
						}
					},
					None => ()
				}
				write!(f, "{}", "}")
			},
			VInteger(v) => write!(f, "{}", v),
			VFunction(ref v) => {
				match v.borrow().clone() {
					NativeFunc(_) => {
						write!(f, "{}", "function() { [native code] }")
					},
					RegularFunc(rf) => {
						write!(f, "function({}){}", rf.args.connect(", "), rf.expr)
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
					if k.as_slice() != INSTANCE_PROTOTYPE.as_slice() {
						nobj.insert(StrBuf::from_str(*k), v.value.borrow().to_json());
					}
				}
				Object(box nobj)
			},
			VString(ref str) => String(str.clone()),
			VNumber(num) => Number(num),
			VInteger(val) => Number(val as f64),
			VFunction(_) => Null
		}
	}
}
impl Add<ValueData, ValueData> for ValueData {
	fn add(&self, other:&ValueData) -> ValueData {
		return match (self.clone(), other.clone()) {
			(VString(s), other) | (other, VString(s)) => VString(StrBuf::from_str(s.as_slice() + other.to_str())),
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
/// Conversion to Javascript values from Rust values
pub trait ToValue {
	/// Convert this value to a Rust value
	fn to_value(&self) -> Value;
}
/// Conversion to Rust values from Javascript values
pub trait FromValue {
	/// Convert this value to a Javascript value
	fn from_value(value:Value) -> Result<Self, &'static str>;
}
impl ToValue for StrBuf {
	fn to_value(&self) -> Value {
		Gc::new(VString(self.clone()))
	}
}
impl FromValue for StrBuf {
	fn from_value(v:Value) -> Result<StrBuf, &'static str> {
		Ok(StrBuf::from_str(v.borrow().to_str()))
	}
}
impl<'s> ToValue for &'s str {
	fn to_value(&self) -> Value {
		Gc::new(VString(StrBuf::from_str(*self)))
	}
}
impl ToValue for ~str {
	fn to_value(&self) -> Value {
		Gc::new(VString(StrBuf::from_str(self.as_slice())))
	}
}
impl FromValue for ~str {
	fn from_value(v:Value) -> Result<~str, &'static str> {
		Ok(v.borrow().to_str())
	}
}
impl<'s> ToValue for MaybeOwned<'s> {
	fn to_value(&self) -> Value {
		Gc::new(VString(StrBuf::from_str(self.as_slice())))
	}
}
impl<'s> FromValue for MaybeOwned<'s> {
	fn from_value(v:Value) -> Result<MaybeOwned<'s>, &'static str> {
		Ok(v.borrow().to_str().into_maybe_owned())
	}
}
impl ToValue for char {
	fn to_value(&self) -> Value {
		Gc::new(VString(StrBuf::from_char(1, *self)))
	}
}
impl FromValue for char {
	fn from_value(v:Value) -> Result<char, &'static str> {
		Ok(v.borrow().to_str().char_at(0))
	}
}
impl ToValue for f64 {
	fn to_value(&self) -> Value {
		Gc::new(VNumber(self.clone()))
	}
}
impl FromValue for f64 {
	fn from_value(v:Value) -> Result<f64, &'static str> {
		Ok(v.borrow().to_num())
	}
}
impl ToValue for i32 {
	fn to_value(&self) -> Value {
		Gc::new(VInteger(self.clone()))
	}
}
impl FromValue for i32 {
	fn from_value(v:Value) -> Result<i32, &'static str> {
		Ok(v.borrow().to_int())
	}
}
impl ToValue for bool {
	fn to_value(&self) -> Value {
		Gc::new(VBoolean(self.clone()))
	}
}
impl FromValue for bool {
	fn from_value(v:Value) -> Result<bool, &'static str> {
		Ok(v.borrow().is_true())
	}
}
impl<T:ToValue> ToValue for Vec<T> {
	fn to_value(&self) -> Value {
		let mut arr = TreeMap::new();
		for i in range(0, self.len()) {
			arr.insert(i.to_str(), Property::new(self.get(i).to_value()));
		}
		to_value(arr)
	}
}
impl<T:FromValue> FromValue for Vec<T> {
	fn from_value(v:Value) -> Result<Vec<T>, &'static str> {
		let len = v.borrow().get_field("length".into_maybe_owned()).borrow().to_int();
		let mut vec = Vec::with_capacity(len as uint);
		for i in range(0, len) {
			vec.push(try!(from_value(v.borrow().get_field(i.to_str().into_maybe_owned()))))
		}
		Ok(vec)
	}
}
impl ToValue for NativeFunctionData {
	fn to_value(&self) -> Value {
		Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(*self)))))
	}
}
impl FromValue for NativeFunctionData {
	fn from_value(v:Value) -> Result<NativeFunctionData, &'static str> {
		match *v.borrow() {
			VFunction(ref func) => {
				match *func.borrow() {
					NativeFunc(ref data) => Ok(data.data),
					_ => Err("Value is not a native function")
				}
			},
			_ => Err("Value is not a function")
		}
	}
}
impl ToValue for ObjectData {
	fn to_value(&self) -> Value {
		Gc::new(VObject(RefCell::new(self.clone())))
	}
}
impl FromValue for ObjectData {
	fn from_value(v:Value) -> Result<ObjectData, &'static str> {
		match *v.borrow() {
			VObject(ref obj) => Ok(obj.clone().borrow().deref().clone()),
			VFunction(ref func) => {
				Ok(match *func.borrow().deref() {
					NativeFunc(ref data) => data.object.clone(),
					RegularFunc(ref data) => data.object.clone()
				})
			},
			_ => Err("Value is not a valid object")
		}
	}
}
impl ToValue for Json {
	fn to_value(&self) -> Value {
		Gc::new(ValueData::from_json(self.clone()))
	}
}
impl FromValue for Json {
	fn from_value(v:Value) -> Result<Json, &'static str> {
		Ok(v.borrow().to_json())
	}
}
impl ToValue for () {
	fn to_value(&self) -> Value {
		Gc::new(VNull)
	}
}
impl FromValue for () {
	fn from_value(_:Value) -> Result<(), &'static str> {
		Ok(())
	}
}
/// A utility function that just calls FromValue::from_value
pub fn from_value<A: FromValue>(v: Value) -> Result<A, &'static str> {
	FromValue::from_value(v)
}

/// A utility function that just calls ToValue::to_value
pub fn to_value<A: ToValue>(v: A) -> Value {
	v.to_value()
}