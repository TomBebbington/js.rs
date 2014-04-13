use js::value::{Value, ValueData, ValueConv, VUndefined, VObject, ResultValue, to_value, from_value};
use collections::treemap::TreeMap;
use std::gc::Gc;

#[deriving(Clone)]
pub type ObjectData = TreeMap<~str, Property>;

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

impl ValueConv for Property {
	fn to_value(&self) -> Value {
		let prop = ValueData::new_obj();
		let prop_ref = prop.borrow();
		prop_ref.set_field(~"configurable", to_value(self.configurable));
		prop_ref.set_field(~"enumerable", to_value(self.enumerable));
		prop_ref.set_field(~"writable", to_value(self.writable));
		prop_ref.set_field(~"value", self.value);
		prop_ref.set_field(~"get", self.get);
		prop_ref.set_field(~"set", self.set);
		prop
	}
	fn from_value(v:Value) -> Option<Property> {
		let vb = v.borrow();
		Some(Property {
			configurable: from_value(vb.get_field(~"configurable")).unwrap(),
			enumerable: from_value(vb.get_field(~"enumerable")).unwrap(),
			writable: from_value(vb.get_field(~"writable")).unwrap(),
			value: vb.get_field(~"value"),
			get: vb.get_field(~"get"),
			set: vb.get_field(~"set")
		})
	}
}
/// Create new object
pub fn make_object(_:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VUndefined))
}
/// Get the prototype
pub fn get_proto_of(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	return Ok(obj.borrow().get_field(~"__proto__"));
}
/// Set the prototype
pub fn set_proto_of(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let proto = args.get(1).clone();
	let obj = args.get(0);
	obj.borrow().set_field(~"__proto__", proto);
	return Ok(*obj);
}
/// Define the property
pub fn define_prop(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	let prop = from_value::<~str>(*args.get(1)).unwrap();
	let desc = from_value::<Property>(*args.get(2)).unwrap();
	obj.borrow().set_prop(prop, desc);
	return Ok(Gc::new(VUndefined));
}
/// To string
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	return Ok(to_value(this.borrow().to_str()));
}
/// Check if it has a property
pub fn has_own_prop(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let prop = from_value::<~str>(*args.get(0)).unwrap();
	Ok(to_value::<bool>(match *this.borrow() {
		VObject(ref obj) => obj.borrow().find(&prop).is_some(),
		_ => false
	}))
}
/// Create a new `Object` object
pub fn _create() -> Value {
	let obj = to_value(make_object);
	let prototype = ValueData::new_obj();
	prototype.borrow().set_field(~"hasOwnProperty", to_value(has_own_prop));
	prototype.borrow().set_field(~"toString", to_value(to_string));
	obj.borrow().set_field(~"length", to_value(1i32));
	obj.borrow().set_field(~"prototype", prototype);
	obj.borrow().set_field(~"setPrototypeOf", to_value(set_proto_of));
	obj.borrow().set_field(~"getPrototypeOf", to_value(get_proto_of));
	obj.borrow().set_field(~"defineProperty", to_value(define_prop));
	obj
}