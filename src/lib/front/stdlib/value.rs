use front::stdlib::object::{PROTOTYPE, INSTANCE_PROTOTYPE, ObjectData, Property};
use front::stdlib::function::Function;
use collections::TreeMap;
use serialize::json::{ToJson, Json, Number, String, Boolean, List, Object, Null};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};
use std::f64;
use std::gc::{Gc, GC};
use std::c_str::CString;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::cmp::PartialOrd;
use front::stdlib::*;
#[must_use]
/// The result of a Javascript expression is represented like this so it can succeed (`Ok`) or fail (`Err`)
pub type ResultValue = Result<Value, Value>;
#[deriving(Clone)]
/// A Garbage-collected Javascript value as represented in the interpreter
pub struct Value {
    /// The garbage-collected pointer
    pub ptr: Gc<ValueData>
}
impl Deref<ValueData> for Value {
    #[inline]
    fn deref<'a>(&'a self) -> &'a ValueData {
        &**self
    }
}
#[deriving(Clone)]
/// A Javascript value
pub enum ValueData {
    /// `null` - A null value, for when a value doesn't exist
    VNull,
    /// `undefined` - An undefined value, for when a field or index doesn't exist
    VUndefined,
    /// `boolean` - A `true` / `false` value, for if a certain criteria is met
    VBoolean(bool),
    /// `String` - A UTF-8 string, such as `"Hello, world"`
    VString(String),
    /// `Number` - A 64-bit floating point number, such as `3.1415`
    VNumber(f64),
    /// `Number` - A 32-bit integer, such as `42`
    VInteger(i32),
    /// `Object` - An object, such as `Math`, represented by a binary tree of string keys to Javascript values
    VObject(RefCell<ObjectData>),
    /// `Function` - A runnable block of code, such as `Math.sqrt`, which can take some variables and return a useful value or act upon an object
    VFunction(RefCell<Function>)
}
impl Value {
    #[inline]
    /// Move some value data into a new value
    pub fn new(data: ValueData) -> Value {
        Value {
            ptr: box(GC) data
        }
    }
    /// Create a new global object
    pub fn new_global() -> Value {
        let global = Value::new_obj(None);
        array::init(global);
        boolean::init(global);
        console::init(global);
        error::init(global);
        function::init(global);
        json::init(global);
        math::init(global);
        number::init(global);
        object::init(global);
        string::init(global);
        uri::init(global);
        global
    }
    /// Returns a new empty object
    pub fn new_obj(global: Option<Value>) -> Value {
        let mut obj : ObjectData = TreeMap::new();
        if global.is_some() {
            let obj_proto = global.unwrap().get_field("Object").get_field(PROTOTYPE);
            obj.insert(INSTANCE_PROTOTYPE.into_string(), Property::new(obj_proto));
        }
        Value::new(VObject(RefCell::new(obj)))
    }
    /// Returns true if the value is an object
    pub fn is_object(&self) -> bool {
        match **self {
            VObject(_) => true,
            _ => false
        }
    }
    /// Returns true if the value is undefined
    pub fn is_undefined(&self) -> bool {
        match **self {
            VUndefined => true,
            _ => false
        }
    }
    /// Returns true if the value is null
    pub fn is_null(&self) -> bool {
        match **self {
            VNull => true,
            _ => false
        }
    }
    /// Returns true if the value is null or undefined
    pub fn is_null_or_undefined(&self) -> bool {
        match **self {
            VNull | VUndefined => true,
            _ => false
        }
    }
    /// Returns true if the value is a 64-bit floating-point number
    pub fn is_double(&self) -> bool {
        match **self {
            VNumber(_) => true,
            _ => false
        }
    }
    /// Returns true if the value is a string
    pub fn is_string(&self) -> bool {
        match **self {
            VString(_) => true,
            _ => false
        }
    }
    /// Returns true if the value is true
    pub fn is_true(&self) -> bool {
        match **self {
            VObject(_) => true,
            VString(ref s) if s.as_slice() == "1" => true,
            VNumber(n) if n >= 1.0 && n % 1.0 == 0.0 => true,
            VInteger(n) if n > 1 => true,
            VBoolean(v) => v,
            _ => false
        }
    }
    /// Converts the value into a 64-bit floating point number
    pub fn to_num(&self) -> f64 {
        match **self {
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
        match **self {
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
    pub fn get_prop<'a>(&self, field:&'a str) -> Option<Property> {
        let obj : ObjectData = match **self {
            VObject(ref obj) => obj.borrow().clone(),
            VFunction(ref func) =>func.borrow().object.clone(),
            _ => return None
        };
        match obj.find(&field.into_string()) {
            Some(val) => Some(*val),
            None => match obj.find(&PROTOTYPE.into_string()) {
                Some(prop) => 
                    prop.value.get_prop(field),
                None => None
            }
        }
    }
    /// Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist
    pub fn get_field<'a>(&self, field:&'a str) -> Value {
        match self.get_prop(field) {
            Some(prop) => prop.value,
            None => Value::new(VUndefined)
        }
    }
    /// Set the field in the value
    pub fn set_field<'a>(&self, field:&'a str, val:Value) -> Value {
        match **self {
            VObject(ref obj) => {
                obj.borrow_mut().insert(field.into_string(), Property::new(val));
            },
            VFunction(ref func) => {
                func.borrow_mut().object.insert(field.into_string(), Property::new(val));
            },
            _ => ()
        }
        val
    }
    /// Set the property in the value
    pub fn set_prop<'a>(&self, field:&'a str, prop:Property) -> Property {
        match **self {
            VObject(ref obj) => {
                obj.borrow_mut().insert(field.into_string(), prop);
            },
            VFunction(ref func) => {
                func.borrow_mut().object.insert(field.into_string(), prop);
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
                    ((i - 1).to_str().into_string(), Property::new(to_value(json.clone())))
                }));
                data.insert("length".into_string(), Property::new(to_value(vs.len() as i32)));
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
    /// Get the type of the value
    pub fn get_type(&self) -> &'static str {
        match **self {
            VNumber(_) | VInteger(_) => "number",
            VString(_) => "string",
            VBoolean(_) => "boolean",
            VNull => "null",
            VUndefined => "undefined",
            _ => "object"
        }
    }
    /// Get the value for undefined
    pub fn undefined() -> Value {
        Value::new(VUndefined)
    }
}
impl fmt::Show for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match **self {
            VNull => write!(f, "null"),
            VUndefined => write!(f, "undefined"),
            VBoolean(v) => write!(f, "{}", v),
            VString(ref v) => write!(f, "{}", v),
            VNumber(v) => write!(f, "{}", match v {
                _ if v.is_nan() => "NaN".into_string(),
                f64::INFINITY => "Infinity".into_string(),
                f64::NEG_INFINITY => "-Infinity".into_string(),
                _ => f64::to_str_digits(v, 15)
            }),
            VObject(ref v) => {
                try!(write!(f, "{}", "{"));
                match v.borrow().iter().last() {
                    Some((last_key, _)) => {
                        for (key, val) in v.borrow().iter() {
                            try!(write!(f, "{}: {}", key, val.value));
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
                let args = v.borrow().args.connect(", ");
                write!(f, "function({}){{...}}", args)
            }
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other:&Value) -> bool {
        match ((**self).clone(), (*other.ptr).clone()) {
            _ if self.is_null_or_undefined() && other.is_null_or_undefined() => true,
            (VString(_), _) | (_, VString(_)) => self.to_str() == other.to_str(),
            (VBoolean(a), VBoolean(b)) if a == b => true,
            (VNumber(a), VNumber(b)) if a == b && !a.is_nan() && !b.is_nan() => true,
            (VNumber(a), _) if a == other.to_num() => true,
            (_, VNumber(a)) if a == self.to_num() => true,
            (VInteger(a), VInteger(b)) if a == b => true,
            _ => false
        }
    }
}
impl ToJson for Value {
    fn to_json( &self ) -> Json {
        match **self {
            VNull | VUndefined => Null,
            VBoolean(b) => Boolean(b),
            VObject(ref obj) => {
                let mut nobj = TreeMap::new();
                for (k, v) in obj.borrow().iter() {
                    if k.as_slice() != INSTANCE_PROTOTYPE.as_slice() {
                        nobj.insert(k.clone(), v.value.to_json());
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
impl Add<Value, Value> for Value {
    fn add(&self, other:&Value) -> Value {
        if self.is_string() || other.is_string() {
            let self_str = self.to_str();
            let other_str = other.to_str();
            let mut text = String::with_capacity(self_str.len() + other_str.len());
            text.push_str(self_str.as_slice());
            text.push_str(other_str.as_slice());
            to_value(text)
        } else {
            to_value(self.to_num() + other.to_num())
        }
    }
}
impl Sub<Value, Value> for Value {
    fn sub(&self, other:&Value) -> Value {
        to_value(self.to_num() - other.to_num())
    }
}
impl Mul<Value, Value> for Value {
    fn mul(&self, other:&Value) -> Value {
        to_value(self.to_num() * other.to_num())
    }
}
impl Div<Value, Value> for Value {
    fn div(&self, other:&Value) -> Value {
        to_value(self.to_num() / other.to_num())
    }
}
impl Rem<Value, Value> for Value {
    fn rem(&self, other:&Value) -> Value {
        to_value(self.to_num() % other.to_num())
    }
}
impl BitAnd<Value, Value> for Value {
    fn bitand(&self, other:&Value) -> Value {
        to_value(self.to_int() & other.to_int())
    }
}
impl BitOr<Value, Value> for Value {
    fn bitor(&self, other:&Value) -> Value {
        to_value(self.to_int() | other.to_int())
    }
}
impl BitXor<Value, Value> for Value {
    fn bitxor(&self, other:&Value) -> Value {
        to_value(self.to_int() ^ other.to_int())
    }
}
impl Shl<Value, Value> for Value {
    fn shl(&self, other:&Value) -> Value {
        to_value(self.to_int() << other.to_int())
    }
}
impl Shr<Value, Value> for Value {
    fn shr(&self, other:&Value) -> Value {
        to_value(self.to_int() >> other.to_int())
    }
}
impl Not<Value> for Value {
    fn not(&self) -> Value {
        to_value(!self.is_true())
    }
}
impl Neg<Value> for Value {
    fn neg(&self) -> Value {
        to_value(-self.to_num())
    }
}
impl PartialOrd for Value {
    fn lt(&self, other: &Value) -> bool {
        self.to_num() < other.to_num()
    }
    fn le(&self, other: &Value) -> bool {
        self.to_num() <= other.to_num()
    }
    fn gt(&self, other: &Value) -> bool {
        self.to_num() > other.to_num()
    }
    fn ge(&self, other: &Value) -> bool {
        self.to_num() >= other.to_num()
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
impl ToValue for Value {
    #[inline(always)]
    fn to_value(&self) -> Value {
        *self
    }
}
impl FromValue for Value {
    #[inline(always)]
    fn from_value(v:Value) -> Result<Value, &'static str> {
        Ok(v)
    }
}
impl ToValue for String {
    #[inline(always)]
    fn to_value(&self) -> Value {
        Value::new(VString(self.clone()))
    }
}
impl FromValue for String {
    fn from_value(v:Value) -> Result<String, &'static str> {
        Ok(v.to_str())
    }
}
impl<'s> ToValue for &'s str {
    fn to_value(&self) -> Value {
        Value::new(VString(String::from_str(*self)))
    }
}
impl ToValue for *i8 {
    fn to_value(&self) -> Value {
        unsafe {
            let cstr = CString::new(*self, false);
            to_value(cstr.as_str().unwrap())
        }
    }
}
impl ToValue for char {
    fn to_value(&self) -> Value {
        Value::new(VString(String::from_char(1, *self)))
    }
}
impl FromValue for char {
    fn from_value(v:Value) -> Result<char, &'static str> {
        Ok(v.to_str().as_slice().char_at(0))
    }
}
impl ToValue for f64 {
    fn to_value(&self) -> Value {
        Value::new(VNumber(*self))
    }
}
impl FromValue for f64 {
    fn from_value(v:Value) -> Result<f64, &'static str> {
        Ok(v.to_num())
    }
}
impl ToValue for i32 {
    fn to_value(&self) -> Value {
        Value::new(VInteger(*self))
    }
}
impl FromValue for i32 {
    fn from_value(v:Value) -> Result<i32, &'static str> {
        Ok(v.to_int())
    }
}
impl ToValue for bool {
    fn to_value(&self) -> Value {
        Value::new(VBoolean(*self))
    }
}
impl FromValue for bool {
    fn from_value(v:Value) -> Result<bool, &'static str> {
        Ok(v.is_true())
    }
}
impl<'s, T:ToValue> ToValue for &'s [T] {
    fn to_value(&self) -> Value {
        let arr:ObjectData = self.iter().enumerate().map(|(i, elem)| {
           (i.to_str(), Property::new(elem.to_value()))
        }).collect();
        to_value(arr)
    }
}
impl<T:ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        let arr:ObjectData = self.iter().enumerate().map(|(i, elem)| {
           (i.to_str(), Property::new(elem.to_value()))
        }).collect();
        to_value(arr)
    }
}
impl<T:FromValue> FromValue for Vec<T> {
    fn from_value(v:Value) -> Result<Vec<T>, &'static str> {
        let len = v.get_field("length").to_int();
        let mut vec = Vec::with_capacity(len as uint);
        for i in range(0, len) {
            vec.push(try!(from_value(v.get_field(i.to_str().as_slice()))))
        }
        Ok(vec)
    }
}
impl ToValue for ObjectData {
    fn to_value(&self) -> Value {
        Value::new(VObject(RefCell::new(self.clone())))
    }
}
impl FromValue for ObjectData {
    fn from_value(v:Value) -> Result<ObjectData, &'static str> {
        match *v.ptr {
            VObject(ref obj) => Ok(obj.borrow().deref().clone()),
            VFunction(ref func) => Ok(func.borrow().object.clone()),
            _ => Err("Value is not a valid object")
        }
    }
}
impl ToValue for Json {
    fn to_value(&self) -> Value {
        Value::new(Value::from_json(self.clone()))
    }
}
impl FromValue for Json {
    fn from_value(v:Value) -> Result<Json, &'static str> {
        Ok(v.to_json())
    }
}
impl ToValue for () {
    fn to_value(&self) -> Value {
        Value::new(VNull)
    }
}
impl FromValue for () {
    fn from_value(_:Value) -> Result<(), &'static str> {
        Ok(())
    }
}
impl<T:ToValue> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        match *self {
            Some(ref v) => v.to_value(),
            None => Value::new(VNull)
        }
    }
}
impl<T:FromValue> FromValue for Option<T> {
    fn from_value(value:Value) -> Result<Option<T>, &'static str> {
        Ok(if value.is_null_or_undefined() {
            None
        } else {
            Some(try!(FromValue::from_value(value)))
        })
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