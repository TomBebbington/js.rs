use js::value::{Value, ValueData, VUndefined, VObject, ResultValue, ToValue, FromValue, to_value, from_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
pub static PROTOTYPE: &'static str = "prototype";
pub static INSTANCE_PROTOTYPE: &'static str = "__proto__";
#[deriving(Clone)]
pub type ObjectData = TreeMap<StrBuf, Property>;

#[deriving(Clone)]
/// A Javascript property
pub struct Property {
	/// If the type of this can be changed and this can be deleted
	pub configurable : bool,
	/// If the property shows up in enumeration of the object
	pub enumerable: bool,
	/// If this property can be changed with an assignment
	pub writable: bool,
	/// The value associated with the property
	pub value: Value,
	/// The function serving as getter
	pub get: Value,
	/// The function serving as setter
	pub set: Value
}
impl Property {
	/// Make a new property with the given value
	pub fn new(value : Value) -> Property {
		Property {
			configurable: false,
			enumerable: false,
			writable: false,
			value: value,
			get: Gc::new(VUndefined),
			set: Gc::new(VUndefined)
		}
	}
}

impl ToValue for Property {
	fn to_value(&self) -> Value {
		let prop = ValueData::new_obj(None);
		let prop_ref = prop.borrow();
		prop_ref.set_field_slice("configurable", to_value(self.configurable));
		prop_ref.set_field_slice("enumerable", to_value(self.enumerable));
		prop_ref.set_field_slice("writable", to_value(self.writable));
		prop_ref.set_field_slice("value", self.value);
		prop_ref.set_field_slice("get", self.get);
		prop_ref.set_field_slice("set", self.set);
		prop
	}
}
impl FromValue for Property {
	fn from_value(v:Value) -> Result<Property, &'static str> {
		let vb = v.borrow();
		Ok(Property {
			configurable: from_value(vb.get_field_slice("configurable")).unwrap(),
			enumerable: from_value(vb.get_field_slice("enumerable")).unwrap(),
			writable: from_value(vb.get_field_slice("writable")).unwrap(),
			value: vb.get_field_slice("value"),
			get: vb.get_field_slice("get"),
			set: vb.get_field_slice("set")
		})
	}
}
/// Create a new object
pub fn make_object(_:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VUndefined))
}
/// Get the prototype of an object
pub fn get_proto_of(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	Ok(obj.borrow().get_field_slice(INSTANCE_PROTOTYPE))
}
/// Set the prototype of an object
pub fn set_proto_of(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = *args.get(0);
	let proto = *args.get(1);
	obj.borrow().set_field_slice(INSTANCE_PROTOTYPE, proto);
	Ok(obj)
}
/// Define a property in an object
pub fn define_prop(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	let prop = from_value::<StrBuf>(*args.get(1)).unwrap();
	let desc = from_value::<Property>(*args.get(2)).unwrap();
	obj.borrow().set_prop(prop, desc);
	Ok(Gc::new(VUndefined))
}
/// To string
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(to_value(this.borrow().to_str().into_strbuf()))
}
/// Check if it has a property
pub fn has_own_prop(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let prop = if args.len() == 0 {
		None
	} else {
		from_value::<StrBuf>(*args.get(0)).ok()
	};
	Ok(to_value::<bool>(prop.is_some() && match *this.borrow() {
		VObject(ref obj) => obj.borrow().find(&prop.unwrap()).is_some(),
		_ => false
	}))
}
/// Create a new `Object` object
pub fn _create(global:Value) -> Value {
	let object = to_value(make_object);
	let object_ptr = object.borrow();
	let prototype = ValueData::new_obj(Some(global));
	prototype.borrow().set_field_slice("hasOwnProperty", to_value(has_own_prop));
	prototype.borrow().set_field_slice("toString", to_value(to_string));
	object_ptr.set_field_slice("length", to_value(1i32));
	object_ptr.set_field_slice(PROTOTYPE, prototype);
	object_ptr.set_field_slice("setPrototypeOf", to_value(set_proto_of));
	object_ptr.set_field_slice("getPrototypeOf", to_value(get_proto_of));
	object_ptr.set_field_slice("defineProperty", to_value(define_prop));
	object
}
/// Initialise the `Object` object on the global object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("Object", _create(global));
}