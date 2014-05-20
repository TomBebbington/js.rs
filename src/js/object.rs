use js::value::{Value, ValueData, ValueConv, VUndefined, VObject, ResultValue, to_value, from_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::str::MaybeOwned;
pub static PROTOTYPE: &'static str = "prototype";
pub static INSTANCE_PROTOTYPE: &'static str = "__proto__";
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
		let prop = ValueData::new_obj(None);
		let prop_ref = prop.borrow();
		prop_ref.set_field("configurable".into_maybe_owned(), to_value(self.configurable));
		prop_ref.set_field("enumerable".into_maybe_owned(), to_value(self.enumerable));
		prop_ref.set_field("writable".into_maybe_owned(), to_value(self.writable));
		prop_ref.set_field("value".into_maybe_owned(), self.value);
		prop_ref.set_field("get".into_maybe_owned(), self.get);
		prop_ref.set_field("set".into_maybe_owned(), self.set);
		prop
	}
	fn from_value(v:Value) -> Option<Property> {
		let vb = v.borrow();
		Some(Property {
			configurable: from_value(vb.get_field("configurable".into_maybe_owned())).unwrap(),
			enumerable: from_value(vb.get_field("enumerable".into_maybe_owned())).unwrap(),
			writable: from_value(vb.get_field("writable".into_maybe_owned())).unwrap(),
			value: vb.get_field("value".into_maybe_owned()),
			get: vb.get_field("get".into_maybe_owned()),
			set: vb.get_field("set".into_maybe_owned())
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
	Ok(obj.borrow().get_field(INSTANCE_PROTOTYPE.into_maybe_owned()))
}
/// Set the prototype
pub fn set_proto_of(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let proto = args.get(1).clone();
	let obj = args.get(0);
	obj.borrow().set_field(INSTANCE_PROTOTYPE.into_maybe_owned(), proto);
	Ok(*obj)
}
/// Define the property
pub fn define_prop(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	let prop = from_value::<MaybeOwned>(*args.get(1)).unwrap();
	let desc = from_value::<Property>(*args.get(2)).unwrap();
	obj.borrow().set_prop(prop, desc);
	Ok(Gc::new(VUndefined))
}
/// To string
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(to_value(this.borrow().to_str().into_maybe_owned()))
}
/// Check if it has a property
pub fn has_own_prop(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let prop = if args.len() == 0 {
		None
	} else {
		from_value::<MaybeOwned>(*args.get(0))
	};
	Ok(to_value::<bool>(prop.is_some() && match *this.borrow() {
		VObject(ref obj) => obj.borrow().find(&prop.unwrap().into_owned()).is_some(),
		_ => false
	}))
}
/// Create a new `Object` object
pub fn _create(global:Value) -> Value {
	let object = to_value(make_object);
	let object_ptr = object.borrow();
	let prototype = ValueData::new_obj(Some(global));
	prototype.borrow().set_field("hasOwnProperty".into_maybe_owned(), to_value(has_own_prop));
	prototype.borrow().set_field("toString".into_maybe_owned(), to_value(to_string));
	object_ptr.set_field("length".into_maybe_owned(), to_value(1i32));
	object_ptr.set_field(PROTOTYPE.into_maybe_owned(), prototype);
	object_ptr.set_field("setPrototypeOf".into_maybe_owned(), to_value(set_proto_of));
	object_ptr.set_field("getPrototypeOf".into_maybe_owned(), to_value(get_proto_of));
	object_ptr.set_field("defineProperty".into_maybe_owned(), to_value(define_prop));
	object
}
/// Initialise the `Object` object on the global object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field("Object".into_maybe_owned(), _create(global));
}